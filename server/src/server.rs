use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{anyhow, Result};
use axum::extract::{DefaultBodyLimit, Multipart, Query};
use axum::routing::post;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use http::header::CONTENT_TYPE;
use serde_json::Value;
use sha2::Digest;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use sqlx::{ConnectOptions, Pool, Postgres, QueryBuilder};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::api::fetch_dogs;
use crate::entity::{Dogs, Filter};

fn db_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        let user = std::env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string());
        let password =
            std::env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "password".to_string());
        let host = std::env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string());
        let database = std::env::var("DATABASE_DB").unwrap_or_else(|_| "axum_dogs".to_string());
        format!("postgres://{user}:{password}@{host}:{port}/{database}",)
    })
}

async fn get_pool() -> Result<Pool<Postgres>> {
    let db_connection_str = db_url();
    let mut option = PgConnectOptions::from_str(&db_connection_str)?;
    option.disable_statement_logging();
    // option.log_statements(log::LevelFilter::Debug);

    let retry_count = std::env::var("DB_RETRY")
        .unwrap_or_else(|_| "5".to_string())
        .parse()?;

    for _ in 1..retry_count {
        if let Ok(pool) = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect_with(option.clone())
            .await
        {
            return Ok(pool);
        }
    }

    Ok(PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(option)
        .await?)
}

fn make_app(pool: Pool<Postgres>) -> Router {
    let base_path = std::env::var("IMAGE_BASE").unwrap_or_else(|_| "./images".to_string());

    Router::new()
        .route("/", get(dogs))
        .route("/upload-image", post(search_image))
        .nest_service("/image", ServeDir::new(base_path))
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_headers([CONTENT_TYPE])
                .allow_credentials(false),
        )
        .layer(DefaultBodyLimit::max(1 << 30))
        .layer(TraceLayer::new_for_http())
}

fn setup_log() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
}

pub async fn serve() -> Result<()> {
    let _ = dotenvy::dotenv();
    setup_log();

    let pool = get_pool().await?;

    sqlx::query(include_str!("./dogs.sql"))
        .execute(&pool)
        .await
        .expect("Failed to create table");

    let pool_clone = pool.clone();
    tokio::spawn(async move {
        if let Err(e) = fetch_dogs(pool_clone).await {
            tracing::error!("Failed to fetch dogs: {}", e);
        }
    });

    let app = make_app(pool);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    tracing::info!(
        "Listening on http://{host}:{port}",
        host = host,
        port = port,
    );

    axum::Server::bind(&format!("{host}:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn dogs(
    Query(filter): Query<Filter>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Dogs>>, (StatusCode, String)> {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("select * from dogs where 1=1 ");

    if let Some(happen_dt) = &filter.happen_dt {
        query.push(" and happen_dt >= ");
        query.push_bind(happen_dt);
    }

    if let Some(kind_cd) = &filter.kind_cd {
        query.push(" and kind_cd like ");
        query.push_bind(format!("%{kind_cd}%"));
    }

    if let Some(sex_cd) = &filter.sex_cd {
        query.push(" and sex_cd = ");
        query.push_bind(sex_cd);
    }

    if let Some(neuter_yn) = &filter.neuter_yn {
        query.push(" and neuter_yn = ");
        query.push_bind(neuter_yn);
    }

    query
        .build_query_as()
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(internal_error)
}

async fn search_image(
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> Result<Json<Vec<(Dogs, f64)>>, (StatusCode, String)> {
    let mut hashmap = HashMap::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        hashmap.insert(name, data.to_vec());
    }

    let ext = hashmap
        .get("filename")
        .cloned()
        .unwrap_or_else(|| "example.jpg".as_bytes().to_vec());
    let ext = String::from_utf8(ext).map_err(internal_error)?;
    let ext = ext.split('.').last().unwrap_or("jpg");

    let data = hashmap
        .get("image")
        .ok_or_else(|| internal_error(anyhow!("no image")))?;
    let sha256 = format!("{:x}", sha2::Sha256::digest(&data));
    let base_path = std::env::var("IMAGE_BASE").unwrap_or_else(|_| "./images".to_string());
    let base_path = format!("{base_path}/input");
    tokio::fs::create_dir_all(&base_path)
        .await
        .map_err(internal_error)?;
    let path = format!("{base_path}/{id}.{ext}", id = sha256);
    let mut file = File::create(&path).await.map_err(internal_error)?;
    let image = data.as_slice();
    file.write_all(image).await.map_err(internal_error)?;

    let url = std::env::var("AI_URL").unwrap_or_else(|_| "http://localhost:3030".to_string());

    let res = reqwest::get(format!("{url}/acc?path={path}"))
        .await
        .map_err(internal_error)?;
    let res = res.text().await.map_err(internal_error)?;
    tracing::info!("res: {:?}", res);
    let value: Value = serde_json::from_str(&res).map_err(internal_error)?;
    let value = value
        .get("results")
        .ok_or_else(|| internal_error(anyhow!("no result")))?;
    let mut value = value
        .as_array()
        .ok_or_else(|| internal_error(anyhow!("no array")))?
        .clone();

    value.sort_by(|a, b| {
        let a = a.get("acc").unwrap().as_f64().unwrap();
        let b = b.get("acc").unwrap().as_f64().unwrap();
        b.partial_cmp(&a).unwrap()
    });

    let res = value
        .into_iter()
        .map(|v| {
            let id = v.get("key").unwrap().as_str().unwrap().to_string();
            let acc = v.get("acc").unwrap().as_f64().unwrap();
            let pool = pool.clone();

            async move {
                (
                    sqlx::query_as::<Postgres, Dogs>("select * from dogs where desertion_no = $1")
                        .bind(id)
                        .fetch_one(&pool)
                        .await,
                    acc,
                )
            }
        })
        .collect::<Vec<_>>();

    let mut ret = Vec::new();

    for fut in res {
        let (dog, acc) = fut.await;
        ret.push((dog.map_err(internal_error)?, acc));
    }

    Ok(Json(ret))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: Display,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

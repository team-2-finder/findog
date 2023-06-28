use std::time::Duration;

use anyhow::Result;
use axum::extract::Query;
use axum::routing::post;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Pool, Postgres, QueryBuilder};

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

    let retry_count = std::env::var("DB_RETRY")
        .unwrap_or_else(|_| "5".to_string())
        .parse()?;

    for _ in 1..retry_count {
        if let Ok(pool) = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
        {
            return Ok(pool);
        }
    }

    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await?)
}

fn make_app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(dogs))
        .route("/upload-image", post(search_image))
        .with_state(pool)
}

fn setup_log() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

pub async fn serve() -> Result<()> {
    dotenvy::dotenv()?;
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
        query.push_bind(format!("'%{kind_cd}%'"));
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

async fn search_image(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

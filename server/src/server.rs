use std::time::Duration;

use anyhow::Result;
use axum::extract::Query;
use axum::routing::post;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Execute, Pool, Postgres, QueryBuilder};

async fn get_pool() -> Result<Pool<Postgres>> {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

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
        .with_max_level(tracing::Level::DEBUG)
        .init();
}

pub async fn serve() -> Result<()> {
    dotenvy::dotenv()?;
    setup_log();

    let pool = get_pool().await?;

    sqlx::query_file!("src/dogs.sql")
        .execute(&pool)
        .await
        .expect("Failed to create table");

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

mod date_serializer {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Ok(s) = String::deserialize(deserializer) {
            NaiveDate::parse_from_str(&s, FORMAT)
                .map(Some)
                .map_err(serde::de::Error::custom)
        } else {
            Ok(None)
        }
    }

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}

#[derive(Debug, Deserialize)]
struct Filter {
    #[serde(default)]
    #[serde(with = "date_serializer")]
    happen_dt: Option<NaiveDate>,
    kind_cd: Option<String>,
    sex_cd: Option<String>,
    neuter_yn: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Dogs {
    id: String,
    #[serde(with = "date_serializer")]
    happen_dt: NaiveDate,
    kind_cd: String,
    color_cd: String,
    age: i32,
    weight: i32,
    sex_cd: String,
    neuter_yn: String,
    care_nm: String,
    care_tel: String,
    care_addr: String,
    charge_nm: String,
    officetel: String,
    notice_comment: String,
}

async fn dogs(
    Query(filter): Query<Filter>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Dogs>>, (StatusCode, String)> {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("select * from dogs ");

    if let Some(happen_dt) = &filter.happen_dt {
        query.push(" and happen_dt >= ");
        query.push_bind(happen_dt);
    }

    if let Some(kind_cd) = &filter.kind_cd {
        query.push(" and kind_cd = ");
        query.push_bind(kind_cd);
    }

    if let Some(sex_cd) = &filter.sex_cd {
        query.push(" and sex_cd = ");
        query.push_bind(sex_cd);
    }

    if let Some(neuter_yn) = &filter.neuter_yn {
        query.push(" and neuter_yn = ");
        query.push_bind(neuter_yn);
    }

    sqlx::query_as(query.build().sql())
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

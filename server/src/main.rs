use std::time::Duration;

use anyhow::Result;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::get,
    Router,
};
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Pool, Postgres};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = get_pool().await?;

    let app = Router::new()
        .route(
            "/",
            get(using_connection_pool_extractor).post(using_connection_extractor),
        )
        .with_state(pool);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    axum::Server::bind(&format!("{host}:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

struct DatabaseConnection(PoolConnection<Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

async fn using_connection_extractor(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let mut conn = conn;
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&mut conn)
        .await
        .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

use anyhow::Result;

mod api;
mod entity;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    server::serve().await
}

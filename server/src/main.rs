use anyhow::Result;

mod server;

#[tokio::main]
async fn main() -> Result<()> {
    server::serve().await
}

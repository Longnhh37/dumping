#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::serve().await
}

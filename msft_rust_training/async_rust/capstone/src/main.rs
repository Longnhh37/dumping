mod client;
mod command;
mod error;
mod message;
mod room;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::serve().await
}

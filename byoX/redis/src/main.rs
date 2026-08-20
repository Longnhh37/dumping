mod command;
mod net;
mod resp;
mod storage;

use anyhow::Result;
use net::listener::Listener;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:6379";
    let listener = Listener::bind(addr).await?;
    println!("listening on: {}", addr);
    net::serve(listener).await?;
    Ok(())
}

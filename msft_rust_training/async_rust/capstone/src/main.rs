mod room;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on :8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[{addr}] connected");

        tokio::spawn(async move {
            let (half_read, mut writer) = socket.into_split();
            let mut reader = BufReader::new(half_read);
            let mut line = String::new();

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        let _ = writer.write_all(line.as_bytes()).await;
                    }
                }
            }
            println!("[{addr}] disconnected");
        });
    }
}

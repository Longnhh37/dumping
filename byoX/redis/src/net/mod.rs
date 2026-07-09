mod buffer;
pub mod connection;
pub mod listener;

use crate::command::Command;
use crate::net::{connection::Connection, listener::Listener};
use crate::resp::{ParseResult, encode, parse};
use crate::storage::Db;
use anyhow::Result;

pub async fn serve(listener: Listener) -> Result<()> {
    loop {
        let conn = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(conn).await {
                eprintln!("connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(mut conn: Connection) -> Result<()> {
    let mut db = Db::new();

    loop {
        // 1. read more bytes from stream to read_buf
        let bytes_read = conn.read_more().await?;

        // 2. EOF - client closes conenction
        if bytes_read == 0 {
            return Ok(());
        }

        // 3. try parsing frame from buffer. Read more if incomplete
        let (frame, consumed) = loop {
            match parse(conn.buffer()) {
                ParseResult::Complete(frame, consumed) => break (frame, consumed),
                ParseResult::Incomplete => {
                    let n = conn.read_more().await?;
                    if n == 0 {
                        return Ok(());
                    }
                }
                ParseResult::Error(e) => return Err(e),
            }
        };

        // 4. consume parsed bytes
        conn.consume(consumed);

        // 5. dispatch -> execute
        let response = match Command::from_frame(frame) {
            Ok(cmd) => cmd.execute(&mut db),
            Err(e) => crate::resp::Frame::Error(format!("ERR {}", e)),
        };

        // 6. encode response -> write to write_buf -> flush to stream
        conn.write(&encode(&response))?;
        conn.flush().await?;
    }
}

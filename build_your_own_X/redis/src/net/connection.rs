use anyhow::Result;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::net::buffer::Buffer;

pub struct Connection {
    pub stream: TcpStream,
    pub read_buf: Buffer,
    pub write_buf: Buffer,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            read_buf: Buffer::new(),
            write_buf: Buffer::new(),
        }
    }

    pub async fn read_more(&mut self) -> Result<usize> {
        let mut buf = [0u8; 4096];
        let bytes = self.stream.read(&mut buf).await?;
        self.read_buf.append(&buf[..bytes]);
        Ok(bytes)
    }

    pub async fn flush(&mut self) -> Result<()> {
        self.stream.write_all(&self.write_buf.data[self.write_buf.read_pos..]).await?;
        self.write_buf = Buffer::new();
        Ok(())
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.write_buf.append(buf);
        Ok(())
    }

    pub fn buffer(&self) -> &[u8] {
        self.read_buf.as_slice()
    }

    pub fn consume(&mut self, n: usize) {
        self.read_buf.consume(n);
    }
}

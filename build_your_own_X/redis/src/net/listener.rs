use anyhow::Result;
use tokio::net::TcpListener;

use crate::net::connection::Connection;

pub struct Listener {
    inner: TcpListener,
}

impl Listener {
    pub async fn bind(addr: &str) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Listener { inner: listener })
    }

    pub async fn accept(&self) -> Result<Connection> {
        let (stream, _) = self.inner.accept().await?;
        let conn = Connection::new(stream);
        Ok(conn)
    }
}

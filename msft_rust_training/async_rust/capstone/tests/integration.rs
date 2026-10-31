use std::{net::SocketAddr, time::Duration};

use chat_server::{
    error::ChatError,
    server::{self, Config},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        TcpListener, TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::oneshot,
    task::JoinHandle,
    time::timeout,
};

const WAIT: Duration = Duration::from_secs(3);

struct TestServer {
    addr: SocketAddr,
    stop: oneshot::Sender<()>,
    handle: JoinHandle<Result<(), ChatError>>,
}

impl TestServer {
    async fn start(config: Config) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (stop, stop_rx) = oneshot::channel::<()>();
        let handle = tokio::spawn(server::run(listener, config, async move {
            let _ = stop_rx.await;
        }));
        Self { addr, stop, handle }
    }

    fn shutdown(self) -> JoinHandle<Result<(), ChatError>> {
        let _ = self.stop.send(());
        self.handle
    }
}

struct TestClient {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
}

impl TestClient {
    async fn connect(addr: SocketAddr) -> Self {
        let (r, w) = TcpStream::connect(addr).await.unwrap().into_split();
        let mut client = Self { reader: BufReader::new(r), writer: w };
        let welcome = client.read_line().await;
        assert!(welcome.starts_with("Welcome"), "unexpected greeting: {welcome}");
        client
    }

    async fn send(&mut self, line: &str) {
        self.writer.write_all(format!("{line}\n").as_bytes()).await.unwrap();
    }

    async fn read_raw_line(&mut self) -> Option<String> {
        let mut line = String::new();
        let n = timeout(WAIT, self.reader.read_line(&mut line))
            .await
            .expect("timed out waiting for the server")
            .expect("read error");
        (n > 0).then(|| line.trim_end().to_owned())
    }

    async fn read_line(&mut self) -> String {
        self.read_raw_line().await.expect("connection closed")
    }

    async fn read_until(&mut self, needle: &str) -> Vec<String> {
        let mut seen = Vec::new();
        loop {
            let line = self.read_line().await;
            let done = line.contains(needle);
            seen.push(line);
            if done {
                return seen;
            }
        }
    }

    async fn read_until_eof(&mut self) -> Vec<String> {
        let mut lines = Vec::new();
        while let Some(line) = self.read_raw_line().await {
            lines.push(line);
        }
        lines
    }
}

#[tokio::test]
async fn two_clients_can_chat() {
    let server = TestServer::start(Config::default()).await;
    let mut c1 = TestClient::connect(server.addr).await;
    let mut c2 = TestClient::connect(server.addr).await;

    c1.send("Hello from client 1").await;

    let seen = c2.read_until("Hello from client 1").await;
    assert_eq!(seen.last().unwrap(), "guest-1: Hello from client 1");
}

#[tokio::test]
async fn messages_stay_in_their_room() {
    let server = TestServer::start(Config::default()).await;
    let mut a = TestClient::connect(server.addr).await;
    let mut b = TestClient::connect(server.addr).await;
    let mut c = TestClient::connect(server.addr).await;

    c.send("/join rust").await;
    c.read_until("you joined #rust").await;

    a.send("to general").await;
    b.read_until("guest-1: to general").await;

    c.send("secret").await;
    c.send("/rooms").await;
    let seen = c.read_until("active rooms (2)").await;
    assert!(!seen.iter().any(|l| l.contains("to general")), "leaked: {seen:?}");
    assert_eq!(c.read_line().await, "\t#general (2)");
    assert_eq!(c.read_line().await, "\t#rust (1)");

    a.send("marker").await;
    let seen = b.read_until("guest-1: marker").await;
    assert!(!seen.iter().any(|l| l.contains("secret")), "leaked: {seen:?}");
}

#[tokio::test]
async fn nick_changes_and_validation() {
    let server = TestServer::start(Config::default()).await;
    let mut a = TestClient::connect(server.addr).await;
    let mut b = TestClient::connect(server.addr).await;

    a.send("/nick alice").await;
    a.read_until("you are now known as alice").await;
    b.read_until("guest-1 is now known as alice").await;

    a.send("/nick").await;
    a.read_until("error").await;
    a.send(&format!("/nick {}", "x".repeat(50))).await;
    a.read_until("error").await;

    a.send("still alice").await;
    let seen = b.read_until("still alice").await;
    assert_eq!(seen.last().unwrap(), "alice: still alice");
}

#[tokio::test]
async fn quit_disconnects_and_notifies_room() {
    let server = TestServer::start(Config::default()).await;
    let mut a = TestClient::connect(server.addr).await;
    let mut b = TestClient::connect(server.addr).await;

    b.send("/quit").await;
    let lines = b.read_until_eof().await;
    assert!(lines.iter().any(|l| l.contains("bye")), "{lines:?}");

    a.read_until("guest-2 left #general").await;
}

#[tokio::test]
async fn idle_clients_are_disconnected() {
    let config = Config { idle_timeout: Duration::from_millis(200), ..Config::default() };
    let server = TestServer::start(config).await;
    let mut c = TestClient::connect(server.addr).await;

    let lines = c.read_until_eof().await;
    assert!(lines.iter().any(|l| l.contains("idle")), "{lines:?}");
}

#[tokio::test]
async fn graceful_shutdown_notifies_clients_and_exits() {
    let server = TestServer::start(Config::default()).await;
    let mut a = TestClient::connect(server.addr).await;
    let mut b = TestClient::connect(server.addr).await;

    let handle = server.shutdown();

    for client in [&mut a, &mut b] {
        let lines = client.read_until_eof().await;
        assert!(lines.iter().any(|l| l.contains("Server shutting down")), "{lines:?}");
    }

    timeout(WAIT, handle)
        .await
        .expect("server did not stop in time")
        .unwrap()
        .unwrap();
}

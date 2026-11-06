// Before
struct Server;

fn create_server(host: &str, port: &str, max_conn: &str) -> Server {
    todo!()
}

// ====================================================================
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

// After
#[derive(Debug)]
pub enum ConfigError {
    InvalidHost,
    InvalidPort,
    InvalidMaxConn,
}

struct ServerConfig {
    host: Host,
    port: Port,
    max_conn: MaxConnections,
}

struct Host;
struct Port(u16);
struct MaxConnections(u32);

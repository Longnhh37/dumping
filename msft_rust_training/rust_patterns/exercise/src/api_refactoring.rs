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
#[derive(Debug, Clone)]
struct Host(String);

impl TryFrom<&str> for Host {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err("host cannot be emty".into());
        }
        if s.contains(char::is_whitespace) {
            return Err("host cannot contain spaces".into());
        }
        Ok(Host(s.to_string()))
    }
}

#[derive(Debug, Clone, Copy)]
struct Port(u16);

impl TryFrom<u16> for Port {
    type Error = String;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err("port must be >= 1".into());
        }
        Ok(Port(value))
    }
}

#[derive(Debug, Clone, Copy)]
struct MaxConnections(u32);

impl TryFrom<u32> for MaxConnections {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 || value > 10_000 {
            return Err(format!("max connections must be 1-10_000, got {value}"));
        }
        Ok(MaxConnections(value))
    }
}

#[derive(Debug)]
struct ServerConfig {
    host: Host,
    port: Port,
    max_connections: MaxConnections,
}

impl ServerConfig {
    fn new(host: Host, port: Port, max_connections: MaxConnections) -> Self {
        Self {
            host,
            port,
            max_connections,
        }
    }
}

fn example() {
    let config = ServerConfig::new(
        Host::try_from("localhost").unwrap(),
        Port::try_from(8080).unwrap(),
        MaxConnections::try_from(100).unwrap(),
    );

    println!("{config:?}");
}

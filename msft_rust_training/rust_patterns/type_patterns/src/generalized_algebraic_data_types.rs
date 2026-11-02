// Redis-like client
// Problem with untyped: fn send(&self, ags: &[&str]) -> Reply:
// caller have to intepret itself, bugs:
//      - TTL returns -2 (key not exist), -1 (no expiry) (magic number)
//      - GET returns nil or string
//      - INCR returns INT, caller matches to BULK

use std::cell::RefCell;
use std::collections::HashMap;
use std::io;

// ----------------------------------------------------------------
// Wire level (what the "server" sends back)
// ----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Reply {
    Status(String),
    Int(i64),
    Bulk(Option<String>),
    Array(Vec<Reply>),
}

fn protocol_err<T>(msg: &str) -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::InvalidData, msg.to_owned()))
}

// ----------------------------------------------------------------
// Domain types
// ----------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Counter(i64);

// replace magic number of TTL
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Expiry {
    Missing,
    Persistent,
    Expires(u64),
}

// ----------------------------------------------------------------
// The "GADT" trait
// ----------------------------------------------------------------
trait Command {
    type Response;

    fn encode(&self) -> Vec<String>;
    fn parse(&self, reply: Reply) -> io::Result<Self::Response>;
}

// ----------------------------------------------------------------
// One struct per command
// ----------------------------------------------------------------
struct Ping;
impl Command for Ping {
    type Response = ();
    fn encode(&self) -> Vec<String> {
        vec!["PING".into()]
    }
    fn parse(&self, reply: Reply) -> io::Result<Self::Response> {
        match reply {
            Reply::Status(s) if s == "PONG" => Ok(()),
            other => protocol_err(&format!("PING: unexpected {other:?}")),
        }
    }
}

struct Get<'a> {
    key: &'a str,
}
impl Command for Get<'_> {
    type Response = Option<String>;
    fn encode(&self) -> Vec<String> {
        vec!["GET".into(), self.key.into()]
    }
    fn parse(&self, reply: Reply) -> io::Result<Self::Response> {
        match reply {
            Reply::Bulk(v) => Ok(v),
            other => protocol_err("GET: unexpected {other:?}"),
        }
    }
}

struct Incr<'a> {
    key: &'a str,
}
impl Command for Incr<'_> {
    type Response = Counter;
    fn encode(&self) -> Vec<String> {
        vec!["INCR".into(), self.key.into()]
    }
    fn parse(&self, reply: Reply) -> io::Result<Self::Response> {
        match reply {
            Reply::Int(n) => Ok(Counter(n)),
            other => protocol_err(&format!("INCR: unexpected {other:?}")),
        }
    }
}

struct Ttl<'a> {
    key: &'a str,
}
impl Command for Ttl<'_> {
    type Response = Expiry;
    fn encode(&self) -> Vec<String> {
        vec!["TTL".into(), self.key.into()]
    }
    fn parse(&self, reply: Reply) -> io::Result<Self::Response> {
        match reply {
            Reply::Int(-2) => Ok(Expiry::Missing),
            Reply::Int(-1) => Ok(Expiry::Persistent),
            Reply::Int(n) if n >= 0 => Ok(Expiry::Expires(n.cast_unsigned())),
            other => protocol_err(&format!("TTL: unexpected {other:?}")),
        }
    }
}

struct LRange<'a> {
    key: &'a str,
    start: i64,
    stop: i64,
}
impl Command for LRange<'_> {
    type Response = Vec<String>;
    fn encode(&self) -> Vec<String> {
        vec!["LRANGE".into(), self.key.into()]
    }
    fn parse(&self, reply: Reply) -> io::Result<Self::Response> {
        let Reply::Array(items) = reply else {
            return protocol_err("LRANGE: expected array");
        };
        items
            .into_iter()
            .map(|item| match item {
                Reply::Bulk(Some(s)) => Ok(s),
                other => protocol_err(&format!("LRANGE: bad element {other:?}")),
            })
            .collect()
    }
}

// ----------------------------------------------------------------
// Executor (monomorphized, not dyn)
// ----------------------------------------------------------------
struct Client {
    counters: RefCell<HashMap<String, i64>>,
}

impl Client {
    fn new() -> Self {
        Self {
            counters: RefCell::new(HashMap::new()),
        }
    }

    fn execute<C: Command>(&self, cmd: &C) -> io::Result<C::Response> {
        let reply = self.send(&cmd.encode())?;
        cmd.parse(reply)
    }

    fn send(&self, args: &[String]) -> io::Result<Reply> {
        match args[0].as_str() {
            "PING" => Ok(Reply::Status("PONG".into())),
            "GET" => Ok(Reply::Bulk(match args[1].as_str() {
                "name" => Some("bob".into()),
                _ => None,
            })),
            "INCR" => {
                let mut map = self.counters.borrow_mut();
                let c = map.entry(args[1].clone()).or_insert(0);
                *c += 1;
                Ok(Reply::Int(*c))
            }
            "TTL" => Ok(Reply::Int(match args[1].as_str() {
                "session" => 120,
                "name" => -1,
                _ => -2,
            })),
            "LRANGE" => Ok(Reply::Array(vec![
                Reply::Bulk(Some("a".into())),
                Reply::Bulk(Some("b".into())),
            ])),
            _ => protocol_err("unknown command"),
        }
    }
}

// ----------------------------------------------------------------
// macro: tuple of typed results
// ----------------------------------------------------------------
macro_rules! pipeline {
    ($client:expr; $($cmd:expr), + $(,)?) => {{
        ( $( $client.execute(&$cmd)?, )+ )
        }};
}

// ----------------------------------------------------------------
// Caller
// ----------------------------------------------------------------
fn example() -> io::Result<()> {
    let client = Client::new();

    let name: Option<String> = client.execute(&Get { key: "name " })?;
    let hits: Counter = client.execute(&Incr { key: "hits" })?;
    let expiry: Expiry = client.execute(&Ttl { key: "session" })?;

    // Compile errors (uncomment to see):
    // let x: Counter = client.execute(&Get { key: "name" })?;
    //     expected `Counter`, found `Option<String>`
    // if hits > expiry {}
    //     `Counter` vs `Expiry`: mismatched types

    match expiry {
        Expiry::Missing => println!("no suck key"),
        Expiry::Persistent => println!("never expires"),
        Expiry::Expires(n) => println!("expires in {n}s",),
    }
    println!("name = {name:?}, hits = {hits:?}");

    let (_, who, n, list) = pipeline!(client;
        Ping,
        Get { key: "name" },
        Incr { key: "hits" },
        LRange { key: "queue", start: 0, stop: -1 },
    );
    println!("{who:?} {n:?} {list:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ttl_decodes_magic_numbers() {
        let cmd = Ttl { key: "k" };
        assert_eq!(cmd.parse(Reply::Int(-2)).unwrap(), Expiry::Missing);
        assert_eq!(cmd.parse(Reply::Int(-1)).unwrap(), Expiry::Persistent);
        assert_eq!(cmd.parse(Reply::Int(30)).unwrap(), Expiry::Expires(30));
    }

    #[test]
    fn get_maps_nil_to_none() {
        let cmd = Get { key: "k" };
        assert_eq!(cmd.parse(Reply::Bulk(None)).unwrap(), None);
    }

    #[test]
    fn wrong_reply_shape_is_an_error() {
        assert!(Incr { key: "k" }.parse(Reply::Status("OK".into())).is_err());
    }
}

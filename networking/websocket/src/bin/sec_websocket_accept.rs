use base64::{Engine, engine::general_purpose::STANDARD};
use sha1::{Digest, Sha1};

fn accept_key(client_key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(client_key.as_bytes());
    hasher.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    STANDARD.encode(hasher.finalize())
}

fn main() {
    // RFC 6455
    let key = accept_key("dGhlIHNhbXBsZSBub25jZQ==");
    assert_eq!(key, "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=");
    println!("{key}");
}

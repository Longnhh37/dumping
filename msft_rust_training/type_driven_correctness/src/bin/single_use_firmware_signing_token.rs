pub struct SigningToken {
    key_id: String,
}

pub struct SignedImage {
    pub signature: Vec<u8>,
    pub key_id: String,
}

impl SigningToken {
    pub fn issue(key_id: &str) -> Self {
        Self {
            key_id: key_id.into(),
        }
    }
}

pub fn sign(token: SigningToken, _image: &[u8]) -> SignedImage {
    SignedImage {
        signature: vec![0xDE, 0xAD],
        key_id: token.key_id,
    }
}

fn main() {
    let image = b"sample image bytes".to_vec();

    let tok = SigningToken::issue("release-key");
    let _signed = sign(tok, &image);

    // Error: use of moved value
    // let _signed2 = sign(tok, &image);
}

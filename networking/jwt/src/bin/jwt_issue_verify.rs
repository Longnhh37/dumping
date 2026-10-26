use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    iat: u64,
    exp: u64,
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn main() -> Result<(), jsonwebtoken::errors::Error> {
    let secret = b"replace-this-with-a-randomized-secret-thats-longer-than-32-byte";

    let claims = Claims {
        sub: "user_42".into(),
        role: "admin".into(),
        iat: now(),
        exp: now() + 15 * 60, // 15 minutes
    };
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret),
    )?;
    println!("{token}");

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp", "sub"]);

    let data = decode::<Claims>(&token, &DecodingKey::from_secret(secret), &validation)?;
    println!("{:?}", data.claims);
    Ok(())
}

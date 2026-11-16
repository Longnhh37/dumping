use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::{NanoServiceError, NanoServiceErrorStatus, ResultExt};

const TOKEN_TTL_SECS: u64 = 24 * 60 * 60;

#[derive(Serialize, Deserialize)]
struct Claims {
    unique_id: String,
    exp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderToken {
    pub unique_id: String,
}

impl HeaderToken {
    pub fn get_key() -> Result<String, NanoServiceError> {
        std::env::var("JWT_SECRET").eject(NanoServiceErrorStatus::Unknown)
    }

    pub fn encode(self) -> Result<String, NanoServiceError> {
        let key = EncodingKey::from_secret(Self::get_key()?.as_bytes());
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .eject(NanoServiceErrorStatus::Unknown)?
            .as_secs()
            + TOKEN_TTL_SECS;
        let claims = Claims {
            unique_id: self.unique_id,
            exp,
        };
        jsonwebtoken::encode(&Header::default(), &claims, &key)
            .eject(NanoServiceErrorStatus::Unknown)
    }

    pub fn decode(token: &str) -> Result<Self, NanoServiceError> {
        let key = DecodingKey::from_secret(Self::get_key()?.as_bytes());
        jsonwebtoken::decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256))
            .map(|data| HeaderToken {
                unique_id: data.claims.unique_id,
            })
            .eject(NanoServiceErrorStatus::Unauthorized)
    }
}

#[cfg(feature = "axum")]
mod axum_impl {
    use super::HeaderToken;
    use crate::errors::{NanoServiceError, NanoServiceErrorStatus};
    use axum::{extract::FromRequestParts, http::request::Parts};

    impl<S> FromRequestParts<S> for HeaderToken
    where
        S: Send + Sync,
    {
        type Rejection = NanoServiceError;

        async fn from_request_parts(
            parts: &mut Parts,
            _state: &S,
        ) -> Result<Self, Self::Rejection> {
            let Some(raw_data) = parts.headers.get("token") else {
                return Err(NanoServiceError::new(
                    "Token not found in header under key 'token'",
                    NanoServiceErrorStatus::Unauthorized,
                ));
            };

            let Ok(token) = raw_data.to_str() else {
                return Err(NanoServiceError::new(
                    "Token is not a valid string",
                    NanoServiceErrorStatus::Unauthorized,
                ));
            };

            HeaderToken::decode(token)
        }
    }
}

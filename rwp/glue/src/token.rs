use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderToken {
    pub unique_id: String,
}

impl HeaderToken {
    pub fn get_key() -> Result<String, NanoServiceError> {
        std::env::var("JWT_SECRET").map_err(|err| {
            NanoServiceError::new(err.to_string(), NanoServiceErrorStatus::Unauthorized)
        })
    }

    pub fn encode(self) -> Result<String, NanoServiceError> {
        let key_str = Self::get_key()?;
        let key = EncodingKey::from_secret(key_str.as_ref());
        jsonwebtoken::encode(&Header::default(), &self, &key).map_err(|err| {
            NanoServiceError::new(err.to_string(), NanoServiceErrorStatus::Unauthorized)
        })
    }

    pub fn decode(token: &str) -> Result<Self, NanoServiceError> {
        let key_str = Self::get_key()?;
        let key = DecodingKey::from_secret(key_str.as_ref());

        let mut validation = Validation::new(Algorithm::HS256);
        validation.required_spec_claims.remove("exp");

        jsonwebtoken::decode(token, &key, &validation)
            .map(|ok| ok.claims)
            .map_err(|err| {
                NanoServiceError::new(err.to_string(), NanoServiceErrorStatus::Unauthorized)
            })
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

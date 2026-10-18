use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderToken {
    pub message: String,
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

            Ok(HeaderToken {
                message: token.to_owned(),
            })
        }
    }
}

use std::future::Future;

use cache_client::logout as cache_logout;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

use crate::user_session::descriptors::RedisSessionDescriptor;

pub trait LogoutUserSession {
    fn logout_user_session(
        &self,
        address: &str,
        user_id: &str,
    ) -> impl Future<Output = Result<(), NanoServiceError>> + Send;
}

impl<T: Sync> LogoutUserSession for RedisSessionDescriptor<T> {
    async fn logout_user_session(
        &self,
        address: &str,
        user_id: &str,
    ) -> Result<(), NanoServiceError> {
        match cache_logout(address, user_id).await?.as_str() {
            "OK" | "NOT_FOUND" => Ok(()),
            other => Err(NanoServiceError::new(
                format!("Unexpected Redis logout response: {other}"),
                NanoServiceErrorStatus::Unknown,
            )),
        }
    }
}

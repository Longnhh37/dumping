use std::future::Future;

use cache_client::{UserSessionStatus, login, update};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus, ResultExt};

#[cfg(feature = "core-postgres")]
use crate::api::users::get::get_user_by_unique_id_core;
#[cfg(all(feature = "http", not(feature = "core-postgres")))]
use crate::api::users::get::get_user_by_unique_id_http;
#[cfg(feature = "core-postgres")]
use crate::user_session::descriptors::RedisSessionDescriptor;
#[cfg(all(feature = "http", not(feature = "core-postgres")))]
use crate::user_session::descriptors::RedisSessionDescriptor;
use crate::user_session::schema::UserSession;

const TIMEOUT_MINS: usize = 20;

pub trait GetUserSession {
    fn get_user_session(
        &self,
        unique_id: &str,
    ) -> impl Future<Output = Result<UserSession, NanoServiceError>> + Send;
}

#[cfg(feature = "core-postgres")]
impl<T> GetUserSession for RedisSessionDescriptor<T>
where
    T: auth_dal::users::transactions::get::GetByUniqueId + Send + Sync,
{
    async fn get_user_session(&self, unique_id: &str) -> Result<UserSession, NanoServiceError> {
        let addr = std::env::var("CACHE_API_URL").eject(NanoServiceErrorStatus::BadRequest)?;
        let session_status = update(&addr, unique_id).await?;
        match session_status {
            UserSessionStatus::Ok(user_id) => Ok(UserSession { user_id }),
            UserSessionStatus::Refresh => {
                let user = get_user_by_unique_id_core(&self.store, unique_id).await?;
                login(&addr, unique_id, TIMEOUT_MINS, user.id).await?;
                Ok(UserSession { user_id: user.id })
            }
        }
    }
}

#[cfg(all(feature = "http", not(feature = "core-postgres")))]
impl GetUserSession for RedisSessionDescriptor {
    async fn get_user_session(&self, unique_id: &str) -> Result<UserSession, NanoServiceError> {
        let addr = std::env::var("CACHE_API_URL").eject(NanoServiceErrorStatus::BadRequest)?;
        let session_status = update(&addr, unique_id).await?;
        match session_status {
            UserSessionStatus::Ok(user_id) => Ok(UserSession { user_id }),
            UserSessionStatus::Refresh => {
                let user = get_user_by_unique_id_http(unique_id).await?;
                login(&addr, unique_id, TIMEOUT_MINS, user.id).await?;
                Ok(UserSession { user_id: user.id })
            }
        }
    }
}

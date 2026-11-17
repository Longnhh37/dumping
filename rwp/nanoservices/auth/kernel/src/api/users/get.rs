#[cfg(any(feature = "core-postgres", feature = "http"))]
mod common_imports {
    pub use auth_dal::users::schema::TrimmedUser;
    pub use glue::errors::NanoServiceError;
}

#[cfg(feature = "core-postgres")]
mod core_imports {
    pub use auth_core::api::users::get::get_by_unique_id as get_by_unique_id_core;
    pub use auth_dal::users::transactions::get::GetByUniqueId;
}

#[cfg(all(feature = "http", not(feature = "core-postgres")))]
mod reqwest_imports {
    pub use glue::errors::{NanoServiceErrorStatus, ResultExt};
    pub use glue::token::HeaderToken;
    pub use reqwest::Client;
}

#[cfg(any(feature = "core-postgres", feature = "http"))]
use common_imports::*;

#[cfg(feature = "core-postgres")]
use core_imports::*;

#[cfg(all(feature = "http", not(feature = "core-postgres")))]
use reqwest_imports::*;

#[cfg(feature = "core-postgres")]
pub async fn get_user_by_unique_id<T: GetByUniqueId>(
    store: &T,
    id: &str,
) -> Result<TrimmedUser, NanoServiceError> {
    get_by_unique_id_core(store, id).await
}

#[cfg(all(feature = "http", not(feature = "core-postgres")))]
pub async fn get_user_by_unique_id(id: &str) -> Result<TrimmedUser, NanoServiceError> {
    get_user_by_unique_id_api_call(id).await
}

#[cfg(all(feature = "http", not(feature = "core-postgres")))]
async fn get_user_by_unique_id_api_call(id: &str) -> Result<TrimmedUser, NanoServiceError> {
    let url = std::env::var("AUTH_API_URL").eject(NanoServiceErrorStatus::BadRequest)?;
    let full_url = format!("{}/api/v1/users/me", url);
    let header_token = HeaderToken {
        unique_id: id.to_owned(),
    }
    .encode()?;

    let client = Client::new();
    let response = client
        .get(&full_url)
        .header("token", header_token)
        .send()
        .await
        .eject(NanoServiceErrorStatus::BadRequest)?;

    if response.status().is_success() {
        let trimmed_user = response
            .json::<TrimmedUser>()
            .await
            .eject(NanoServiceErrorStatus::BadRequest)?;

        Ok(trimmed_user)
    } else {
        Err(NanoServiceError::new(
            format!("failed to get user: {}", response.status()),
            NanoServiceErrorStatus::BadRequest,
        ))
    }
}

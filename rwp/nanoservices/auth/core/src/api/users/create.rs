use serde::Deserialize;

use auth_dal::users::schema::{NewUser, User};
use auth_dal::users::transactions::create::SaveOne;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus, ResultExt};

#[derive(Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

pub async fn create<T: SaveOne>(store: &T, data: CreateUser) -> Result<User, NanoServiceError> {
    let user = tokio::task::spawn_blocking(move || NewUser::new(data.email, data.password))
        .await
        .eject(NanoServiceErrorStatus::Unknown)??;
    let created_item = store.save_one(user).await?;
    Ok(created_item)
}

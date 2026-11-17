use auth_dal::users::schema::TrimmedUser;
use auth_dal::users::transactions::get::GetByUniqueId;
use glue::errors::NanoServiceError;

pub async fn get_by_unique_id<T: GetByUniqueId>(
    store: &T,
    id: &str,
) -> Result<TrimmedUser, NanoServiceError> {
    let user = T::get_by_unique_id(store, id).await?;
    let trimmed_user: TrimmedUser = user.into();
    Ok(trimmed_user)
}

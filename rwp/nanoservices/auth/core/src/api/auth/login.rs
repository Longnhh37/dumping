use auth_dal::users::transactions::get::GetByEmail;
use glue::{
    errors::{NanoServiceError, NanoServiceErrorStatus},
    token::HeaderToken,
};

pub async fn login<T: GetByEmail>(
    email: String,
    password: String,
) -> Result<String, NanoServiceError> {
    let user = T::get_by_email(&self, email).await?;
    let verified = user.verify_password(password)?;
    if verified {
        Ok(HeaderToken {
            unique_id: user.unique_id,
        }
        .encode()?)
    } else {
        Err(NanoServiceError::new(
            "invalid password".to_string(),
            NanoServiceErrorStatus::Unauthorized,
        ))
    }
}

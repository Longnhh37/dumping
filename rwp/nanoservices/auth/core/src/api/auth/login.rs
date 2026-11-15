use auth_dal::users::transactions::get::GetByEmail;
use glue::{
    errors::{NanoServiceError, NanoServiceErrorStatus},
    token::HeaderToken,
};

pub async fn login<T: GetByEmail>(
    store: &T,
    email: String,
    password: String,
) -> Result<String, NanoServiceError> {
    let user = store.get_by_email(email).await.map_err(|error| {
        if error.status == NanoServiceErrorStatus::NotFound {
            NanoServiceError::new(
                "invalid email or password",
                NanoServiceErrorStatus::Unauthorized,
            )
        } else {
            error
        }
    })?;

    if !user.verify_password(password)? {
        return Err(NanoServiceError::new(
            "invalid email or password",
            NanoServiceErrorStatus::Unauthorized,
        ));
    }

    HeaderToken {
        unique_id: user.unique_id,
    }
    .encode()
}

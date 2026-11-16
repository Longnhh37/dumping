use auth_dal::users::{schema::User, transactions::get::GetByEmail};
use glue::{
    errors::{NanoServiceError, NanoServiceErrorStatus, ResultExt},
    token::HeaderToken,
};

fn invalid_credentials() -> NanoServiceError {
    NanoServiceError::new(
        "invalid email or password",
        NanoServiceErrorStatus::Unauthorized,
    )
}

pub async fn login<T: GetByEmail>(
    store: &T,
    email: &str,
    password: String,
) -> Result<String, NanoServiceError> {
    let user = match store.get_by_email(email).await {
        Ok(user) => user,
        Err(e) if e.status == NanoServiceErrorStatus::NotFound => {
            tokio::task::spawn_blocking(move || User::verify_dummy_password(&password))
                .await
                .eject(NanoServiceErrorStatus::Unknown)?;
            return Err(invalid_credentials());
        }
        Err(e) => return Err(e),
    };

    let (verified, unique_id) = tokio::task::spawn_blocking(move || {
        user.verify_password(&password)
            .map(|ok| (ok, user.unique_id))
    })
    .await
    .eject(NanoServiceErrorStatus::Unknown)??;

    if !verified {
        return Err(invalid_credentials());
    }

    HeaderToken { unique_id }.encode()
}

use std::sync::OnceLock;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};

use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub unique_id: String,
}

impl NewUser {
    pub fn new(email: String, password: String) -> Result<Self, NanoServiceError> {
        let unique_id = uuid::Uuid::new_v4().to_string();
        let argon2_hasher = Argon2::default();

        let hashed_password = argon2_hasher
            .hash_password(password.as_bytes())
            .map_err(|e| {
                NanoServiceError::new(
                    format!("Failed to hash password: {e}"),
                    NanoServiceErrorStatus::Unknown,
                )
            })?
            .to_string();

        Ok(Self {
            email,
            password_hash: hashed_password,
            unique_id,
        })
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[sqlx(rename = "password")]
    pub password_hash: String,
    pub unique_id: String,
}

impl User {
    // Burns the same CPU as a real verification so unknown emails are not faster.
    pub fn verify_dummy_password(password: &str) {
        static DUMMY: OnceLock<String> = OnceLock::new();
        let hash = DUMMY.get_or_init(|| {
            Argon2::default()
                .hash_password(b"dummy-password")
                .map(|h| h.to_string())
                .unwrap_or_default()
        });
        if let Ok(parsed) = PasswordHash::new(hash) {
            let _ = Argon2::default().verify_password(password.as_bytes(), &parsed);
        }
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, NanoServiceError> {
        let parsed = PasswordHash::new(&self.password_hash).map_err(|e| {
            NanoServiceError::new(
                format!("Failed to parse password hash: {e}"),
                NanoServiceErrorStatus::Unknown,
            )
        })?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrimmedUser {
    pub id: i32,
    pub email: String,
    pub unique_id: String,
}

impl From<User> for TrimmedUser {
    fn from(user: User) -> Self {
        TrimmedUser {
            id: user.id,
            email: user.email,
            unique_id: user.unique_id,
        }
    }
}

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};

use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub email: String,
    pub password: String,
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
            password: hashed_password,
            unique_id,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl User {
    pub fn verify_password(&self, password: String) -> Result<bool, NanoServiceError> {
        let argon2_hasher = Argon2::default();
        let parsed_hash = PasswordHash::new(&self.password).map_err(|e| {
            NanoServiceError::new(
                format!("Failed to parse password hash: {e}"),
                NanoServiceErrorStatus::Unknown,
            )
        })?;
        let is_valid = argon2_hasher
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_valid)
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

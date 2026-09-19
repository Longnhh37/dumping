use serde::{Serialize, Deserialize};

use crate::models::user::{NewUser, User};

/// Incoming request body for POST /users
#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
}

/// Outgoing response body for all user endpoints
#[derive(Debug, Serialize)]
pub struct UserReponseDto {
    pub id: i32,
    pub username: String,
}

impl From<CreateUserDto> for NewUser {
    fn from(dto: CreateUserDto) -> Self {
        Self {
            username: dto.username,
        }
    }
}

impl From<User> for UserReponseDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
        }
    }
}


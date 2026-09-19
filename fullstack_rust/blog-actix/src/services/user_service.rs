use diesel::SqliteConnection;

use crate::{
    db,
    dto::user::{CreateUserDto, UserReponseDto},
    errors::AppError,
};

pub fn list(conn: &mut SqliteConnection) -> Result<Vec<UserReponseDto>, AppError> {
    let users = db::users::list(conn)?;
    Ok(users.into_iter().map(UserReponseDto::from).collect())
}

pub fn get(conn: &mut SqliteConnection, user_id: i32) -> Result<UserReponseDto, AppError> {
    let user = db::users::find(conn, user_id)?;
    Ok(UserReponseDto::from(user))
}

/// Create a new user after validation
///
/// Rules:
/// - 'username' must not be blank
/// - Uniqueness is enforced by the DB UNIQUE constraint -> RecordAlreadyExists
pub fn create(conn: &mut SqliteConnection, dto: CreateUserDto) -> Result<UserReponseDto, AppError> {
    if dto.username.trim().is_empty() {
        return Err(AppError::BadRequest("username cannot be blank".to_string()));
    }

    let user = db::users::insert(conn, dto.into())?;
    Ok(UserReponseDto::from(user))
}

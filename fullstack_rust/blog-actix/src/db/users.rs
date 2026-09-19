use diesel::{SqliteConnection, prelude::*};

use crate::{
    errors::AppError,
    models::user::{NewUser, User},
    schema::users,
};

pub fn list(conn: &mut SqliteConnection) -> Result<Vec<User>, AppError> {
    users::table
        .select(User::as_select())
        .load(conn)
        .map_err(AppError::from)
}

pub fn find(conn: &mut SqliteConnection, user_id: i32) -> Result<User, AppError> {
    users::table
        .find(user_id)
        .select(User::as_select())
        .first(conn)
        .map_err(AppError::from)
}

pub fn insert(conn: &mut SqliteConnection, new_user: NewUser) -> Result<User, AppError> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .map_err(AppError::from)?;

    users::table
        .order(users::id.desc())
        .select(User::as_select())
        .first(conn)
        .map_err(AppError::from)
}

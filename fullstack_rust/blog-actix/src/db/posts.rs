use diesel::{SqliteConnection, prelude::*};

use crate::{
    errors::AppError,
    models::post::{NewPost, Post},
    schema::posts,
};

pub fn list(conn: &mut SqliteConnection) -> Result<Vec<Post>, AppError> {
    posts::table
        .select(Post::as_select())
        .load(conn)
        .map_err(AppError::from)
}

pub fn list_by_user(conn: &mut SqliteConnection, user_id: i32) -> Result<Vec<Post>, AppError> {
    posts::table
        .filter(posts::user_id.eq(user_id))
        .select(Post::as_select())
        .load(conn)
        .map_err(AppError::from)
}

pub fn find(conn: &mut SqliteConnection, post_id: i32) -> Result<Post, AppError> {
    posts::table
        .find(post_id)
        .select(Post::as_select())
        .first(conn)
        .map_err(AppError::from)
}

pub fn insert(conn: &mut SqliteConnection, new_post: NewPost) -> Result<Post, AppError> {
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .map_err(AppError::from)?;

    posts::table
        .order(posts::id.desc())
        .select(Post::as_select())
        .first(conn)
        .map_err(AppError::from)
}

pub fn publish(conn: &mut SqliteConnection, post_id: i32) -> Result<Post, AppError> {
    diesel::update(posts::table.find(post_id))
        .set(posts::published.eq(true))
        .execute(conn)
        .map_err(AppError::from)?;

    find(conn, post_id)
}

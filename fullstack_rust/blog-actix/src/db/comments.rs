use diesel::{SqliteConnection, prelude::*};

use crate::{
    errors::AppError,
    models::comment::{Comment, NewComment},
    schema::comments,
};

pub fn list_by_post(conn: &mut SqliteConnection, lookup_id: i32) -> Result<Vec<Comment>, AppError> {
    comments::table
        .filter(comments::post_id.eq(lookup_id))
        .select(Comment::as_select())
        .load(conn)
        .map_err(AppError::from)
}

pub fn find(conn: &mut SqliteConnection, cmt_id: i32) -> Result<Comment, AppError> {
    comments::table
        .find(cmt_id)
        .select(Comment::as_select())
        .first(conn)
        .map_err(AppError::from)
}

pub fn insert(conn: &mut SqliteConnection, new_cmt: NewComment) -> Result<Comment, AppError> {
    diesel::insert_into(comments::table)
        .values(&new_cmt)
        .execute(conn)
        .map_err(AppError::from)?;

    comments::table
        .order(comments::id.desc())
        .select(Comment::as_select())
        .first(conn)
        .map_err(AppError::from)
}

pub fn delete(conn: &mut SqliteConnection, cmt_id: i32) -> Result<(), AppError> {
    let affected = diesel::delete(comments::table.find(cmt_id))
        .execute(conn)
        .map_err(AppError::from)?;

    if affected == 0 {
        return Err(AppError::RecordNotFound);
    }

    Ok(())
}

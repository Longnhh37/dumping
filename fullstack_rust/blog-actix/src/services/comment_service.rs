use diesel::SqliteConnection;

use crate::{
    db,
    dto::comment::{CommentReponseDto, CreateCommentDto},
    errors::AppError,
};

/// List all comments from a CommentReponseDto
///
/// Return 'RecordNotFound' if post not exists
pub fn list_by_post(
    conn: &mut SqliteConnection,
    post_id: i32,
) -> Result<Vec<CommentReponseDto>, AppError> {
    db::posts::find(conn, post_id)?;

    let cmts = db::comments::list_by_post(conn, post_id)?;
    Ok(cmts.into_iter().map(CommentReponseDto::from).collect())
}

/// Get a comment by id
pub fn get(conn: &mut SqliteConnection, cmt_id: i32) -> Result<CommentReponseDto, AppError> {
    let cmt = db::comments::find(conn, cmt_id)?;
    Ok(CommentReponseDto::from(cmt))
}

/// Create a new comment
///
/// Rules:
/// - 'body' cannot be blank
/// - 'post_id' must exist -> 'RecordNotFound' if not
/// - 'user_id' must exist -> 'RecordNotFound' if not
pub fn create(
    conn: &mut SqliteConnection,
    post_id: i32,
    dto: CreateCommentDto,
) -> Result<CommentReponseDto, AppError> {
    if dto.body.trim().is_empty() {
        return Err(AppError::BadRequest("body cannot be blank".to_string()));
    }

    db::posts::find(conn, post_id)?;
    db::users::find(conn, dto.user_id)?;

    let new_cmt = (dto, post_id).into();
    let cmt = db::comments::insert(conn, new_cmt)?;

    Ok(CommentReponseDto::from(cmt))
}

/// Delete a comment by id
///
/// Return 'RecordNotFound' if comment does not exist
pub fn delete(conn: &mut SqliteConnection, cmt_id: i32) -> Result<(), AppError> {
    db::comments::delete(conn, cmt_id)
}

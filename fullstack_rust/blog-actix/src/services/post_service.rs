use diesel::SqliteConnection;

use crate::{
    db,
    dto::post::{CreatePostDto, PostReponseDto},
    errors::AppError,
};

pub fn list(conn: &mut SqliteConnection) -> Result<Vec<PostReponseDto>, AppError> {
    let posts = db::posts::list(conn)?;
    Ok(posts.into_iter().map(PostReponseDto::from).collect())
}

/// Lists all posts belonging to 'user_id'
///
/// Return 'RecordNotFound' if user does not exist
pub fn list_by_user(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Vec<PostReponseDto>, AppError> {
    // verify the user exists before querying posts
    db::users::find(conn, user_id)?;

    let posts = db::posts::list_by_user(conn, user_id)?;
    Ok(posts.into_iter().map(PostReponseDto::from).collect())
}

pub fn get(conn: &mut SqliteConnection, post_id: i32) -> Result<PostReponseDto, AppError> {
    let post = db::posts::find(conn, post_id)?;
    Ok(PostReponseDto::from(post))
}

/// Create a new post after validation
///
/// Rules:
/// - 'title' and 'body' must not be blank
/// - 'user_id' must reference an existing user -> 'RecordNotFound' if not
pub fn create(conn: &mut SqliteConnection, dto: CreatePostDto)
-> Result<PostReponseDto, AppError> {
    if dto.title.trim().is_empty() {
        return Err(AppError::BadRequest("title cannot be blank".to_string()));
    }
    if dto.body.trim().is_empty() {
        return Err(AppError::BadRequest("body cannot be blank".to_string()));
    }

    // Verify the parent user exists
    db::users::find(conn, dto.user_id)?;

    let post = db::posts::insert(conn, dto.into())?;
    Ok(PostReponseDto::from(post))
}

/// Publish a post after validation
///
/// Rules:
/// - 'post_id' must exist and not yet published
pub fn publish(conn: &mut SqliteConnection, post_id: i32) -> Result<PostReponseDto, AppError> {
    // verify post exists
    let post = db::posts::find(conn, post_id)?;

    if post.published {
        return Err(AppError::BadRequest("post is already published".to_string()));
    }

    let post = db::posts::publish(conn, post_id)?;
    Ok(PostReponseDto::from(post))
}


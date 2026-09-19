use serde::{Deserialize, Serialize};

use crate::models::comment::{NewComment, Comment};

/// Incoming request body for POST /posts/{post_id}/comments
#[derive(Debug, Deserialize)]
pub struct CreateCommentDto {
    pub user_id: i32,
    pub body: String,
}

/// Outgoing response body for all comment endpoints
#[derive(Debug, Serialize)]
pub struct CommentReponseDto {
    pub id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub body: String,
}

impl From<(CreateCommentDto, i32)> for NewComment {
    /// post_id is taken from URL path
    fn from((dto, post_id): (CreateCommentDto, i32)) -> Self {
        Self {
            user_id: dto.user_id,
            post_id,
            body: dto.body,
        }
    }
}

impl From<Comment> for CommentReponseDto {
    fn from(cmt: Comment) -> Self {
        Self {
            id: cmt.id,
            user_id: cmt.user_id,
            post_id: cmt.post_id,
            body: cmt.body,
        }
    }
}



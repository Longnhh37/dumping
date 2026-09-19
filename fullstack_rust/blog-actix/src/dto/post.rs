use serde::{Deserialize, Serialize};

use crate::models::post::{NewPost, Post};

/// Incoming request body for POST /posts
#[derive(Debug, Deserialize)]
pub struct CreatePostDto {
    pub user_id: i32,
    pub title: String,
    pub body: String,
}

/// Outgoing response body for all post endpoints
#[derive(Debug, Serialize)]
pub struct PostReponseDto {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl From<CreatePostDto> for NewPost {
    fn from(dto: CreatePostDto) -> Self {
        Self {
            user_id: dto.user_id,
            title: dto.title,
            body: dto.body,
        }
    }
}

impl From<Post> for PostReponseDto {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            user_id: post.user_id,
            title: post.title,
            body: post.body,
            published: post.published,
        }
    }
}


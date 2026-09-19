use actix_web::{HttpResponse, delete, get, post, web};

use crate::{
    DbPool, dto::comment::CreateCommentDto, errors::AppError, routes::get_conn,
    services::comment_service,
};

/// GET /posts/{post_id}/comments
#[get("/posts/{post_id}/comments")]
async fn list_comments(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let mut conn = get_conn(&pool)?;

    let comments = web::block(move || comment_service::list_by_post(&mut conn, post_id))
        .await
        .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::Ok().json(comments))
}

/// GET /comments/{id}
#[get("/comments/{id}")]
async fn get_comment(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let comment_id = path.into_inner();
    let mut conn = get_conn(&pool)?;

    let comment = web::block(move || comment_service::get(&mut conn, comment_id))
        .await
        .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::Ok().json(comment))
}

/// POST /posts/{post_id}/comments
#[post("/posts/{post_id}/comments")]
async fn create_comment(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body: web::Json<CreateCommentDto>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let dto = body.into_inner();
    let mut conn = get_conn(&pool)?;

    let comment = web::block(move || comment_service::create(&mut conn, post_id, dto))
        .await
        .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::Created().json(comment))
}

/// DELETE /comments/{id}
#[delete("/comments/{id}")]
async fn delete_comment(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let comment_id = path.into_inner();
    let mut conn = get_conn(&pool)?;

    web::block(move || comment_service::delete(&mut conn, comment_id))
        .await
        .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_comments)
        .service(get_comment)
        .service(create_comment)
        .service(delete_comment);
}

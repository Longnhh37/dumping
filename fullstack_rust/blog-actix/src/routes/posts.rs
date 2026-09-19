use actix_web::{HttpResponse, get, post, web};

use crate::{
    DbPool, dto::post::CreatePostDto, errors::AppError, routes::get_conn, services::post_service,
};

/// GET /posts
#[get("/posts")]
async fn list_posts(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let posts = web::block(move || {
        let mut conn = get_conn(&pool)?;
        post_service::list(&mut conn)
    })
    .await
    .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::Ok().json(posts))
}

/// GET /users/{user_id}/posts
#[get("/users/{user_id}/posts")]
async fn list_posts_by_user(
    dbpool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    let output = web::block(move || {
        let mut conn = get_conn(&dbpool)?;
        post_service::list_by_user(&mut conn, user_id)
    })
    .await
    .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::Ok().json(output))
}

/// GET /posts/{id}
#[get("/posts/{id}")]
async fn get_post(
    dbpool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();

    let output = web::block(move || {
        let mut conn = get_conn(&dbpool)?;
        post_service::get(&mut conn, post_id)
    })
    .await
    .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::Ok().json(output))
}

/// POST /posts
#[post("/posts")]
async fn create_post(
    dbpool: web::Data<DbPool>,
    body: web::Json<CreatePostDto>,
) -> Result<HttpResponse, AppError> {
    let dto = body.into_inner();

    let post = web::block(move || {
        let mut conn = get_conn(&dbpool)?;
        post_service::create(&mut conn, dto)
    })
    .await
    .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::Created().json(post))
}

/// POST /posts/{id}/publish
#[post("/posts/{id}/publish")]
async fn publish_post(
    dbpool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();

    let post = web::block(move || {
        let mut conn = get_conn(&dbpool)?;
        post_service::publish(&mut conn, post_id)
    })
    .await
    .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::Ok().json(post))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_posts)
        .service(list_posts_by_user)
        .service(get_post)
        .service(create_post)
        .service(publish_post);
}

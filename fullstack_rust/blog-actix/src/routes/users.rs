use actix_web::{HttpResponse, get, post, web};

use crate::{
    DbPool, dto::user::CreateUserDto, errors::AppError, routes::get_conn, services::user_service,
};

/// GET /users
#[get("/users")]
async fn list_users(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let users = web::block(move || {
        let mut conn = get_conn(&pool)?;
        user_service::list(&mut conn)
    })
    .await
    .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::Ok().json(users))
}

/// GET /users/{id}
#[get("/users/{id}")]
async fn get_user(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    let user = web::block(move || {
        let mut conn = get_conn(&pool)?;
        user_service::get(&mut conn, user_id)
    })
    .await
    .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::Ok().json(user))
}

/// POST /users
#[post("/users")]
async fn create_user(
    pool: web::Data<DbPool>,
    body: web::Json<CreateUserDto>,
) -> Result<HttpResponse, AppError> {
    let dto = body.into_inner();

    let user = web::block(move || {
        let mut conn = get_conn(&pool)?;
        user_service::create(&mut conn, dto)
    })
    .await
    .map_err(|_| AppError::OperationCanceled)??;

    Ok(HttpResponse::Created().json(user))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users)
        .service(get_user)
        .service(create_user);
}

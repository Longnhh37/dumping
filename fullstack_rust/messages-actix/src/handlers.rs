use actix_web::{HttpRequest, Result, web};
use std::sync::atomic::Ordering;

use crate::{
    models::{IndexResponse, LookUpResponse, PostInput, PostResponse},
    state::AppState,
};

pub async fn index(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<web::Json<IndexResponse>> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("world");

    let request_count = state.request_count.fetch_add(1, Ordering::Relaxed) + 1;

    let mut msgs = state.messages.lock().unwrap();
    msgs.push(hello.to_owned());

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        message: msgs.clone(),
    }))
}

pub async fn post(
    msg: web::Json<PostInput>,
    state: web::Data<AppState>,
) -> Result<web::Json<PostResponse>> {
    let request_count = state.request_count.fetch_add(1, Ordering::Relaxed) + 1;

    let mut msgs = state.messages.lock().unwrap();
    msgs.push(msg.message.clone());

    Ok(web::Json(PostResponse {
        server_id: state.server_id,
        request_count,
        message: msg.message.clone(),
    }))
}

pub async fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.fetch_add(1, Ordering::Relaxed) + 1;
    let mut msgs = state.messages.lock().unwrap();
    msgs.clear();

    Ok(web::Json(IndexResponse {
        request_count,
        server_id: state.server_id,
        message: vec![],
    }))
}

pub async fn lookup(
    state: web::Data<AppState>,
    idx: web::Path<usize>,
) -> Result<web::Json<LookUpResponse>> {
    let request_count = state.request_count.fetch_add(1, Ordering::Relaxed) + 1;

    let msgs = state.messages.lock().unwrap();
    let result = msgs.get(idx.into_inner()).cloned();

    Ok(web::Json(LookUpResponse {
        server_id: state.server_id,
        request_count,
        result,
    }))
}

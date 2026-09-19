use crate::{models::PostError, state::AppState};
use actix_web::{
    HttpRequest, HttpResponse,
    error::{Error, InternalError, JsonPayloadError},
    web,
};
use std::sync::atomic::Ordering;

pub fn post_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
    let state = req.app_data::<web::Data<AppState>>().unwrap();

    let request_count = state.request_count.fetch_add(1, Ordering::Relaxed) + 1;

    let body = PostError {
        server_id: state.server_id,
        request_count,
        error: format!("{}", err),
    };

    InternalError::from_response(err, HttpResponse::BadRequest().json(body)).into()
}

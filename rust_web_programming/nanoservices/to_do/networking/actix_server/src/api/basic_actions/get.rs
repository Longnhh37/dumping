use actix_web::{HttpRequest, HttpResponse};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use to_do_core::api::basic_actions::get::{
    get_all as get_all_core, get_by_name as get_by_name_core,
};

pub async fn get_all() -> Result<HttpResponse, NanoServiceError> {
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}

pub async fn get_by_name(req: HttpRequest) -> Result<HttpResponse, NanoServiceError> {
    let name = req.match_info().get("name").ok_or_else(|| {
        NanoServiceError::new(
            "Name not provided".to_string(),
            NanoServiceErrorStatus::BadRequest,
        )
    })?;

    Ok(HttpResponse::Ok().json(get_by_name_core(name).await?))
}

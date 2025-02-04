use actix_web::{get, Responder, web};
use crate::response::ApiResponse;

pub fn setup_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

#[utoipa::path(
    context_path = "/health"
)]
#[get("")]
pub async fn health_check() -> impl Responder {
    ApiResponse("OK".to_string())
}

use actix_web::HttpRequest;
use actix_web::web::Data;
use sqlx::PgPool;
use crate::config::Configuration;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Configuration,
}

impl AppState {
    pub fn from_request(req: &HttpRequest) -> Data<AppState> {
        req.app_data::<Data<AppState>>().unwrap().clone()
    }
}
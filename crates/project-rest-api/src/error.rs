use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal Server Error")]
    DomainError(#[from] project_core::domain::error::DomainError),
}

#[derive(Serialize, Debug)]
pub struct JsonErrorResponse {
    pub error: String
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(JsonErrorResponse {
                error: self.to_string()
            })
    }
}
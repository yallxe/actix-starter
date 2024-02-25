use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use serde::Serialize;
use thiserror::Error;
use project_core::error::ClientError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Username {username} is already taken")]
    UsernameIsTaken {
        username: String
    },
    #[error("Email {email} is already registered")]
    EmailAlreadyRegistered {
        email: String
    },
}

#[derive(Serialize, Debug)]
pub struct JsonErrorResponse {
    pub error: String
}

impl From<project_core::Error> for Error {
    fn from(value: project_core::Error) -> Self {
        match value {
            project_core::Error::ClientError(client_err) => match client_err {
                ClientError::UsernameIsTaken { username } => Error::UsernameIsTaken { username },
                ClientError::EmailAlreadyRegistered { email } => Error::EmailAlreadyRegistered { email }
            },
            _ => panic!("{}", value)
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::UsernameIsTaken { .. } => StatusCode::CONFLICT,
            Error::EmailAlreadyRegistered { .. } => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(JsonErrorResponse {
                error: self.to_string()
            })
    }
}
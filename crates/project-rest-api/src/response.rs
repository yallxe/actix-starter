use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::EitherBody;
use actix_web::web::Json;
use serde::Serialize;

pub struct ApiResponse<T>(pub T);

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = EitherBody<String>;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        Json(self.0).respond_to(req)
    }
}

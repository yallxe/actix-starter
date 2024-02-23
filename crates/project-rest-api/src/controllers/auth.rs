use crate::prelude::*;
use actix_web::{post, Responder, web};
use project_core::domain::services::RegistrationService;
use crate::dto::{RegisterUserDto, UserViewDto};
use crate::response::ApiResponse;

pub fn setup_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
}

#[post("/register")]
pub async fn register_user(
    form: web::Json<RegisterUserDto>,
    registration_service: web::Data<dyn RegistrationService>
) -> Result<impl Responder> {
    let res = registration_service.try_register_user(
        form.0.into()
    ).await?;

    Ok(ApiResponse(UserViewDto::from(res)))
}

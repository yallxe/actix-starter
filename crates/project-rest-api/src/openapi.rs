use utoipa::OpenApi;
use crate::controllers::{
    auth, health
};
use crate::dto::{RegisterUserDto, UserViewDto};

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        auth::register_user
    ),
    components(
        schemas(
            RegisterUserDto,
            UserViewDto
        )
    )
)]
pub struct ApiSpec;


impl ApiSpec {
    pub fn new() -> utoipa::openapi::OpenApi {
        ApiSpec::openapi()
    }
}
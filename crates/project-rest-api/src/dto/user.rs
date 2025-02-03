use serde::{Serialize, Deserialize};
use project_core::domain::models::{RegisterUserModel, UserModel};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserViewDto {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserModel> for UserViewDto {
    fn from(value: UserModel) -> Self {
        UserViewDto {
            id: value.id,
            username: value.username,
            email: value.email,
            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<RegisterUserDto> for RegisterUserModel {
    fn from(value: RegisterUserDto) -> Self {
        RegisterUserModel {
            username: value.username,
            email: value.email,
            password: value.password,
        }
    }
}

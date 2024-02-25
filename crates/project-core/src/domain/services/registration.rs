use crate::prelude::*;
use crate::domain::models::{RegisterUserModel, UserModel};

#[async_trait::async_trait]
pub trait RegistrationService: Sync + Send + 'static {
    async fn try_register_user(&self, user: RegisterUserModel) -> Result<UserModel>;
}
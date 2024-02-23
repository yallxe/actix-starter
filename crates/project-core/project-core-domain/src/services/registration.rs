use crate::models::{RegisterUserModel, UserModel};
use crate::result::DomainResult;

#[async_trait::async_trait]
pub trait RegistrationService: Sync + Send {
    async fn try_register_user(&self, user: RegisterUserModel) -> DomainResult<UserModel>;
}
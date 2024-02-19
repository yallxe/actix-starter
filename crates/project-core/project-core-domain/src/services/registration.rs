use crate::models::{RegisterUserModel, UserModel};
use crate::result::DomainResult;

#[allow(async_fn_in_trait)]
pub trait RegistrationService {
    async fn try_register_user(&self, user: RegisterUserModel) -> DomainResult<UserModel>;
}
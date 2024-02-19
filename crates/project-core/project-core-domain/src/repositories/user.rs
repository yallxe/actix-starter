use crate::models::{CreateUserModel, UserModel};
use crate::result::DomainResult;

#[allow(async_fn_in_trait)]
pub trait UserRepository {
    async fn create(&self, user: CreateUserModel) -> DomainResult<UserModel>;
}
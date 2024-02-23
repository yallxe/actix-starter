use crate::models::{CreateUserModel, UserModel};
use crate::result::DomainResult;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: CreateUserModel) -> DomainResult<UserModel>;
}

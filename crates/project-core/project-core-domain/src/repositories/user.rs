use crate::models::{CreateUserModel, UserModel};
use crate::result::DomainResult;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: CreateUserModel) -> DomainResult<UserModel>;
}

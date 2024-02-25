use crate::prelude::*;
use crate::domain::models::{CreateUserModel, UserModel};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: CreateUserModel) -> Result<UserModel>;
    async fn find_by_username(&self, username: String) -> Result<Option<UserModel>>;
}

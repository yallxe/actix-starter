use crate::prelude::*;
use crate::domain::models::{CreateUserModel, RegisterUserModel, UserModel};
use crate::domain::repositories::UserRepository;
use crate::domain::services::{RegistrationService};
use crate::error::ClientError;
use std::sync::Arc;

pub struct RegistrationServiceImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl RegistrationServiceImpl {
    pub fn new<A: UserRepository>(user_repository: A) -> Self {
        Self {
            user_repository: Arc::new(user_repository),
        }
    }
    
    pub fn from(user_repository: Arc<dyn UserRepository>) -> Self {
        Self {
            user_repository,
        }
    }
}

#[async_trait::async_trait]
impl RegistrationService for RegistrationServiceImpl {
    async fn try_register_user(&self, user: RegisterUserModel) -> Result<UserModel> {
        let existing_user = self.user_repository.find_by_username(user.username.clone()).await?;
        if existing_user.is_some() {
            return Err(ClientError::UsernameIsTaken {
                username: user.username,
            }.into());
        }
        
        self.user_repository.create(CreateUserModel {
            username: user.username,
            email: user.email,
            password: user.password,
        }).await
    }
}
use std::sync::Arc;
use project_core_domain::models::{CreateUserModel, RegisterUserModel, UserModel};
use project_core_domain::repositories::UserRepository;
use project_core_domain::services::{RegistrationService};
use project_core_domain::result::DomainResult;

pub struct RegistrationServiceImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl RegistrationServiceImpl {
    pub fn new<A: UserRepository>(user_repository: A) -> Self {
        Self {
            user_repository: Arc::new(user_repository),
        }
    }
}

#[async_trait::async_trait]
impl RegistrationService for RegistrationServiceImpl {
    async fn try_register_user(&self, user: RegisterUserModel) -> DomainResult<UserModel> {
        // TODO: Check if user is already registered, if so return an error
        self.user_repository.create(CreateUserModel {
            username: user.username,
            email: user.email,
            password: user.password,
        }).await
    }
}
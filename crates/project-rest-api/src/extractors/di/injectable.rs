use crate::prelude::*;
use crate::state::AppState;
use std::sync::Arc;
use actix_web::HttpRequest;
use project_core::domain::repositories::UserRepository;
use project_core::domain::services::RegistrationService;
use project_core::infrastructure::repositories::UserRepositoryImpl;
use project_core::infrastructure::services::RegistrationServiceImpl;

pub trait Injectable: 'static {
    async fn get_instance(req: HttpRequest) -> Result<Arc<Self>>;
}

impl Injectable for dyn UserRepository {
    async fn get_instance(req: HttpRequest) -> Result<Arc<Self>> {
        let app_state = AppState::from_request(&req);

        Ok(Arc::new(UserRepositoryImpl::new(app_state.db.clone())))
    }
}

impl Injectable for dyn RegistrationService {
    async fn get_instance(req: HttpRequest) -> Result<Arc<Self>> {
        Ok(Arc::new(RegistrationServiceImpl::from(
            <dyn UserRepository>::get_instance(req.clone()).await?,
        )))
    }
}
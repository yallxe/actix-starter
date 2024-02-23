use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use project_core::domain::repositories::UserRepository;
use project_core::domain::services::RegistrationService;
use project_core::infrastructure::repositories::UserRepositoryImpl;
use project_core::services::RegistrationServiceImpl;
use crate::config::Configuration;
use crate::state::AppState;

mod logging;
mod controllers;
mod response;
mod state;
mod config;
mod error;
mod prelude;
mod dto;

pub struct Container {
    user_repository: Arc<dyn UserRepository>,
    registration_service: Arc<dyn RegistrationService>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    logging::setup_logging();
    let config = Configuration::from_env();

    let db = PgPool::connect_lazy(config.database_url.clone().as_str()).unwrap();
    let state = AppState {
        db: db.clone(), config: config.clone()
    };

    // TODO: Repositories, services, etc. should be injected through FromRequest trait
    let user_repository = UserRepositoryImpl::new(db.clone());
    let registration_service = RegistrationServiceImpl::new(user_repository.clone());

    let container = Container {
        user_repository: Arc::new(user_repository),
        registration_service: Arc::new(registration_service),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(container.registration_service.clone()))
            .app_data(web::Data::new(container.user_repository.clone()))
            .service(
                web::scope("/health")
                    .configure(controllers::health::setup_controller)
            )
            .service(
                web::scope("/auth")
                    .configure(controllers::auth::setup_controller)
            )
    })
        .bind((config.bind_host, config.bind_port))?
        .run()
        .await
}

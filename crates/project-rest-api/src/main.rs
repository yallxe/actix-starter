use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger as ActixLoggerMiddleware;
use openapi::ApiSpec;
use sqlx::PgPool;
use utoipa_swagger_ui::SwaggerUi;
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
mod extractors;
mod openapi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    logging::setup_logging();
    let config = Configuration::from_env();

    let db = PgPool::connect_lazy(config.database_url.clone().as_str()).unwrap();
    let state = AppState {
        db: db.clone(), config: config.clone()
    };

    HttpServer::new(move || {
        let openapi = ApiSpec::new();

        App::new()
            .wrap(ActixLoggerMiddleware::default())
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/health")
                    .configure(controllers::health::setup_controller)
            )
            .service(
                web::scope("/auth")
                    .configure(controllers::auth::setup_controller)
            )
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/docs/openapi.json", openapi)
            )
    })
        .bind((config.bind_host, config.bind_port))?
        .run()
        .await
}

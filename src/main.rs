use std::sync::RwLock;

use actix_web::{middleware, web, App, HttpServer};

use log::info;

use crate::config::AppState;
use crate::routes::{predict, update_config, update_model};

mod config;
mod error;
mod predictor;
mod routes;
mod utils;

const HOST: &'static str = "0.0.0.0";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config_state: web::Data<RwLock<AppState>> =
        web::Data::new(RwLock::new(AppState::default()));

    info!("Server starting on {}:{}", HOST, PORT);

    HttpServer::new(move || {
        App::new()
            .app_data(config_state.clone())
            .wrap(middleware::Logger::default())
            .service(update_config)
            .service(update_model)
            .service(predict)
    })
    .bind((HOST, PORT))?
    .run()
    .await
}

// todo: print address

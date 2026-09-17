use actix_web::{web, App, HttpServer, middleware};
use log::info;
use std::io;

mod interfaces;
mod application;
mod domain;
mod infrastructure;
mod config;

use interfaces::controllers::loan_request_controller;
use config::database;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    info!("Iniciando API de Solicitudes de Préstamos - Fintech v1.0.0");
    info!("Configurando pool de conexiones a base de datos...");
    
    let db_pool = database::create_pool().await.expect("Failed to create database pool");
    
    info!("Iniciando servidor HTTP en 0.0.0.0:8080...");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(config::risk_engine::create_risk_engine_client()))
            .app_data(web::Data::new(config::audit::create_audit_producer()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(loan_request_controller::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
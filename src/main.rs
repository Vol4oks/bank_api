mod application;
mod data;
mod domain;
mod presentation;

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use application::bank_service::BankService;
use data::account_repository::InMemoryAccountRepository;
use presentation::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let repo = Arc::new(InMemoryAccountRepository::default());
    let service = BankService::new(Arc::clone(&repo));

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(service.clone()))
            .service(web::scope("/api").configure(handlers::configure))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

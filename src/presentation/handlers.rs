use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    application::bank_service::BankService,
    data::account_repository::InMemoryAccountRepository,
    domain::DomainError,
    presentation::dto::{AccountResponse, AmountRequest, ApiError, CreateAccountRquest, TransferRequest},
};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[post("/accounts")]
async fn create_account(
    service: web::Data<BankService<InMemoryAccountRepository>>,
    payload: web::Json<CreateAccountRquest>,
) -> actix_web::Result<impl Responder> {
    service
        .create_account(payload.id, payload.initial)
        .await
        .map_err(map_domain_error)?;

    let account = service
        .get_account(payload.id)
        .await
        .map_err(map_domain_error)?;

    Ok(HttpResponse::Created().json(AccountResponse::from(account)))
}

#[get("/accounts/{id}")]
async fn get_account(
    service: web::Data<BankService<InMemoryAccountRepository>>,
    path: web::Path<u32>,
) -> actix_web::Result<impl Responder> {
    let account = service
        .get_account(path.into_inner())
        .await
        .map_err(map_domain_error)?;

    Ok(HttpResponse::Ok().json(AccountResponse::from(account)))
}

#[post("/accounts/{id}/deposit")]
async fn deposit(
    service: web::Data<BankService<InMemoryAccountRepository>>,
    path: web::Path<u32>,
    payload: web::Json<AmountRequest>,
) -> actix_web::Result<impl Responder> {
    let account = service
        .deposit(path.into_inner(), payload.amount)
        .await
        .map_err(map_domain_error)?;

    Ok(HttpResponse::Ok().json(AccountResponse::from(account)))
}

#[post("/accounts/{id}/withdraw")]
async fn withdraw(
    service: web::Data<BankService<InMemoryAccountRepository>>,
    path: web::Path<u32>,
    payload: web::Json<AmountRequest>,
) -> actix_web::Result<impl Responder> {
    let account = service
        .withdraw(path.into_inner(), payload.amount)
        .await
        .map_err(map_domain_error)?;

    Ok(HttpResponse::Ok().json(AccountResponse::from(account)))
}

#[post("/transfer")]
async fn transfer(
    service: web::Data<BankService<InMemoryAccountRepository>>,
    payload: web::Json<TransferRequest>,
) -> actix_web::Result<impl Responder> {
    service
        .transfer(payload.from, payload.to, payload.amount)
        .await
        .map_err(map_domain_error)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"status":"transferred"})))
}

fn map_domain_error(error: DomainError) -> actix_web::Error {
    match error {
        DomainError::InvalodAmount(msg) => {
            let body = ApiError::new(msg.clone());
            actix_web::error::InternalError::from_response(
                DomainError::InvalodAmount(msg),
                HttpResponse::BadRequest().json(body),
            )
            .into()
        }
        DomainError::InsufficientFunds => actix_web::error::InternalError::from_response( 
            error, 
            HttpResponse::BadRequest().json(ApiError::new("Insufficient funds"))
        )
        .into(),
        DomainError::AccountNotFound => actix_web::error::InternalError::from_response( 
            error, 
            HttpResponse::NotFound().json(ApiError::new("account not found"))
        )
        .into(),
    }
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(health)
        .service(create_account)
        .service(get_account)
        .service(deposit)
        .service(withdraw)
        .service(transfer);
}

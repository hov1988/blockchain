use actix_web::{post, web, HttpResponse, Responder};
use utoipa::path;
use crate::apis::models::Transaction;

#[utoipa::path(
    post,
    path = "/blockchain",
    request_body = Transaction,
    responses(
        (status = 200, description = "Transaction added successfully")
    )
)]
#[post("/blockchain")]
async fn add_transaction(transaction: web::Json<Transaction>) -> impl Responder {
    println!("Transaction received: {:?}", transaction);
    HttpResponse::Ok().body("Transaction added successfully")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(add_transaction);
}

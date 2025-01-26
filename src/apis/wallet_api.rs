use actix_web::{get, HttpResponse, Responder};
use utoipa::path;
use crate::apis::models::Wallet;

#[utoipa::path(
    get,
    path = "/wallet",
    responses(
        (status = 200, description = "Wallet information retrieved successfully", body = Wallet)
    )
)]
#[get("/wallet")]
async fn get_wallet() -> impl Responder {
    // Mock wallet response
    let wallet = Wallet {
        address: "sample_address".to_string(),
        public_key: "sample_public_key".to_string(),
        private_key: "sample_private_key".to_string(),
    };

    HttpResponse::Ok().json(wallet)
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_wallet);
}

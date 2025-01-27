use actix_web::{get, post, HttpResponse, Responder};
use utoipa::path;
use crate::wallet::Wallet as BlockchainWallet;
use crate::apis::models::Wallet as APIWallet;
use crate::blockchain::BlockChain;
use log::{info, debug};
use crate::apis::models::Transaction as APITransaction;
use actix_web::web;
use std::sync::Arc;
use std::sync::Mutex;



/// Map a `BlockchainWallet` to an `APIWallet`.
fn map_blockchain_wallet_to_api_wallet(blockchain_wallet: &BlockchainWallet) -> APIWallet {
    APIWallet {
        address: Some(blockchain_wallet.get_address()),
        public_key: Some(blockchain_wallet.public_key_str()),
        private_key: Some(blockchain_wallet.private_key_str()),
    }
}

/// Wallet response schema
#[utoipa::path(
    get,
    path = "/wallet",
    responses(
        (status = 200, description = "Wallet information retrieved successfully", body = APIWallet)
    )
)]
#[get("/wallet")]
async fn get_wallet_data() -> impl Responder {
    // Create a new blockchain wallet instance
    let blockchain_wallet = BlockchainWallet::new();

    // Map the blockchain wallet to the API wallet structure
    let api_wallet = map_blockchain_wallet_to_api_wallet(&blockchain_wallet);

    HttpResponse::Ok().json(api_wallet)
}

/// Handle transactions by adding them to the blockchain
#[utoipa::path(
    post,
    path = "/transaction",
    request_body = APITransaction,
    responses(
        (status = 200, description = "Transaction added successfully"),
        (status = 500, description = "Failed to add transaction to blockchain")
    )
)]
#[post("/transaction")]
pub async fn get_transaction_handler(
    data: web::Data<Arc<Mutex<BlockChain>>>,
    transaction: web::Json<APITransaction>,
) -> impl Responder {
    let tx = transaction.into_inner();

    // Parse and validate the amount
    let amount = match tx.amount {
        Some(value) => value,
        None => return HttpResponse::BadRequest().body("Amount is missing"),
    };

    // Create wallet instance from transaction details
    let wallet = BlockchainWallet::new_from(
        &tx.public_key.clone().unwrap_or_default(),
        &tx.private_key.clone().unwrap_or_default(),
        &tx.sender.clone().unwrap_or_default(),
    );

    // Sign the transaction
    let amount: u64 = amount as u64; // Convert f64 to u64
    let wallet_tx = wallet.sign_transaction(&tx.recipient.clone().unwrap_or_default(), amount);

    let wallet_tx = wallet.sign_transaction(&tx.recipient.clone().unwrap_or_default(), amount);

    // Access the blockchain from the shared state
    let mut blockchain = data.lock().unwrap();
    let add_result = blockchain.add_transaction(&wallet_tx);

    if !add_result {
        info!("Failed to add transaction to blockchain");
        return HttpResponse::InternalServerError().body("Failed to add transaction to blockchain");
    }

    info!("Transaction added successfully to blockchain");
    HttpResponse::Ok().body("Transaction added successfully")
}

/// Configure the `wallet_api` routes
pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_wallet_data);
    cfg.service(get_transaction_handler);
}
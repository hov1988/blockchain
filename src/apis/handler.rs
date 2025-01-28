use crate::apis::dto::query_amount_response::QueryAmountResponse;
use crate::apis::dto::Transaction as APITransaction;
use crate::apis::dto::Wallet;
use crate::core::blockchain::BlockChain;
use crate::core::wallet::Wallet as BlockchainWallet;
use actix_web::web;
use actix_web::{get, post, HttpResponse, Responder};
use log::info;
use std::sync::Arc;
use std::sync::Mutex;

/// Wallet response schema
#[utoipa::path(
    get,
    path = "/wallet",
    responses(
        (status = 200, description = "Wallet information retrieved successfully", body = Wallet)
    )
)]
#[get("/wallet")]
async fn get_wallet_data() -> impl Responder {
    // Create a new blockchain wallet instance
    let blockchain_wallet = BlockchainWallet::new();

    // Map the blockchain wallet to the API wallet structure
    let api_wallet = Wallet::new_from(
        &blockchain_wallet.get_address(),
        &blockchain_wallet.public_key_str(),
        &blockchain_wallet.private_key_str(),
    );

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

/// Amount retrieval handler
#[utoipa::path(
    get,
    path = "/amount/{address}",
    responses(
        (status = 200, description = "Amount retrieved successfully", body = crate::apis::dto::query_amount::QueryAmount)
    )
)]
#[get("/amount/{address}")]
pub async fn get_amount(
    data: web::Data<Arc<Mutex<BlockChain>>>,
    path: web::Path<String>,
) -> HttpResponse {
    let address = path.into_inner();

    // Logic to get the amount for the given address
    let blockchain = data.lock().unwrap();
    let amount = blockchain.calculate_total_amount(address); // Assuming this method exists in BlockChain

    let response = QueryAmountResponse {
        amount: Some(amount as f32),
    };
    HttpResponse::Ok().json(response)
}

/// Mining process handler
#[utoipa::path(
    get,
    path = "/mining",
    responses(
        (status = 200, description = "Mining started successfully")
    )
)]
#[get("/mining")]
pub async fn mining(data: web::Data<Arc<Mutex<BlockChain>>>) -> impl Responder {
    let mut blockchain = data.lock().unwrap();
    let is_mined = blockchain.mining();
    if !is_mined {
        return HttpResponse::InternalServerError().body("Mining failed");
    }
    HttpResponse::Ok().body("Mining started successfully")
}

/// Show transactions handler
#[utoipa::path(
    get,
    path = "/show_transactions",
    responses(
        (status = 200, description = "Transactions shown successfully", body = crate::apis::dto::transactions_in_block_chain_response::TransactionsInBlockChainResponse)
    )
)]
#[get("/show_transactions")]
pub async fn show_transaction(_data: web::Data<Arc<Mutex<BlockChain>>>) -> HttpResponse {
    // Simulating a response, you can implement the actual logic here
    // let blockchain = data.lock().unwrap();
    // let transactions = blockchain.search_block();  // Assuming this method exists in BlockChain
    // let transaction_count = transactions.len();
    //
    // let response = TransactionsInBlockChainResponse {
    //     transaction_count,
    //     transactions,
    // };

    HttpResponse::Ok().json("")
}

/// Configure the `wallet_api` routes
pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_wallet_data);
    cfg.service(get_transaction_handler);
    cfg.service(get_amount);
    cfg.service(mining);
    cfg.service(show_transaction);
}

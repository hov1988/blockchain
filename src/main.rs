mod apis;
use apis::dto;
use apis::server::Server;
pub mod blockchain;
pub mod config;
pub mod wallet;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Run the API server
    Server::run().await
}

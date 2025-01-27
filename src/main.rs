mod apis;
use apis::server::Server;
use apis::models;
pub mod wallet;
pub mod blockchain;
pub mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Run the API server
    Server::run().await
}

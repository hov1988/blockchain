mod apis;
use apis::server::Server;
pub mod config;
pub mod core;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Run the API server
    Server::run().await
}

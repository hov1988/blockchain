mod apis;
use apis::server::Server;
use apis::models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Run the API server
    Server::run().await
}

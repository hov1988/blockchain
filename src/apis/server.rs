use crate::apis;
use actix_files as fs;
use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Define the OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        apis::handler::get_wallet_data,
        apis::handler::get_transaction_handler,
        apis::handler::get_amount,
        apis::handler::show_transaction,
        apis::handler::mining
    ),
    components(schemas(apis::dto::Wallet, apis::dto::Transaction, crate::apis::dto::query_amount::QueryAmount,
        crate::apis::dto::transactions_in_block_chain_response::TransactionsInBlockChainResponse)),
    info(title = "Blockchain API", version = "1.0.0")
)]
pub struct ApiDoc;

pub struct Server;

impl Server {
    /// Runs the server
    pub async fn run() -> std::io::Result<()> {
        HttpServer::new(move || {
            App::new()
                // Serve the pre-defined YAML file at /api-docs/yaml
                .service(
                    fs::Files::new("/api-docs/yaml", "./api") // Adjust path to your `service.yml`
                        .index_file("service.yml"),
                )
                // Serve `utoipa`-generated Swagger UI at /swagger-ui
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url("/api-doc/openapi.json", ApiDoc::openapi()),
                )
                // Configure your APIs
                .configure(apis::handler::configure)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }
}

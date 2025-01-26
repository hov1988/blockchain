use actix_web::{web, App, HttpServer};
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

mod apis;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            apis::wallet_api::get_wallet,
            apis::blockchain_api::add_transaction
        ),
        components(
            schemas(apis::models::Wallet, apis::models::Transaction)
        ),
        info(
            title = "Blockchain API",
            version = "1.0.0"
        )
    )]
    struct ApiDoc;

    HttpServer::new(move || {
        App::new()
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()))
            .configure(apis::configure_wallet_api)
            .configure(apis::configure_blockchain_api)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod apis;
use apis::models;
#[derive(OpenApi)]
#[openapi(
    paths(
        apis::wallet_api::get_wallet
    ),
    components(schemas(models::Wallet, models::Transaction)),
    info(
        title = "Blockchain API",
        version = "1.0.0"
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // Serve the pre-defined YAML file at /api-docs/yaml
            .service(
                fs::Files::new("/api-docs/yaml", "./api") // Adjust path to your `service.yml`
                    .index_file("service.yml")
            )
            // Serve `utoipa`-generated Swagger UI at /swagger-ui
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi())
            )
            // Configure your APIs
            .configure(apis::wallet_api::configure)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
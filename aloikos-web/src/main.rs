use aloikos_web::{auth::AuthApiDoc, db::connect_db, routes::auth};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use axum::{routing::get, Router, serve};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::{openapi::{InfoBuilder, OpenApiBuilder}, OpenApi};
fn create_openapi_spec() -> utoipa::openapi::OpenApi {
    let mut openapi = OpenApiBuilder::new()
        .info(InfoBuilder::new()
            .title("Aloikos API")
            .version("0.1.0")
            .build())
        .build();
    
    // Register auth endpoints
    openapi.merge(AuthApiDoc::openapi());    
    openapi
}
#[tokio::main]
async fn main() {
    println!("🚀 Starting Aloikos Web Application");
    
    // Initialize tracing first
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info,aloikos_web=info,tower_http=info"))
                .unwrap(),
        )
        .init();
    
    // Connect to database
    connect_db().await.unwrap_or_else(|e| {
        tracing::error!("❌ Database connection failed: {}", e);
        std::process::exit(1);
    });
    
    tracing::info!("✅ Database connected successfully");
    let openapi_spec = create_openapi_spec();
    
    let app = Router::new()
        .nest("/api/v1", auth::api_routes())
        .merge(SwaggerUi::new("/api/v1/swagger")
            .url("/api/v1/openapi.json", openapi_spec.clone()))
        .route("/", get(|| async { "Welcome to AlOikos" }))
        .layer(TraceLayer::new_for_http());
    tracing::info!("🌐 Starting server on http://localhost:8000");
    tracing::info!("📚 Swagger UI available at: http://localhost:8000/api/v1/swagger/");
    tracing::info!("🔧 OpenAPI JSON at: http://localhost:8000/api/v1/openapi.json");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    serve(listener, app).await.unwrap();
}

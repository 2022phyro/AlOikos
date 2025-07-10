use aloikos_web::config::CONFIG;
use aloikos_web::db::connect_db;
use tracing::log::info;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use axum::{routing::get, Router, serve};
#[tokio::main]
async fn main() {
    println!("🚀 Starting Aloikos Web Application");
    connect_db().await.unwrap_or_else(|e| eprintln!("❌ Database connection failed: {}", e));
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("axum_tracing_example=error,tower_http=warn"))
                .unwrap(),
        )
        .init();
    let app = Router::new().route("/", get(|| async { "Hello, World!" })).layer(TraceLayer::new_for_http());
    info!("Starting server on port: 8000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    serve(listener, app).await.unwrap();
}

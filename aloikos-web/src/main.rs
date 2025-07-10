use aloikos_web::config::CONFIG;
use aloikos_web::db::connect_db;
use tracing::log::info;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use axum::{routing::get, Router, serve};
#[tokio::main]
async fn main() {
    println!("🚀 Starting Aloikos Web Application");

    // Display configuration
    println!("\n⚙️  Database Configuration:");
    println!("   DB URI: {}", CONFIG.db_uri);
    println!("   DB Name: {}", CONFIG.db_name);

    // Test database connection
    println!("\n🔗 Testing database connection...");
    connect_db().await.unwrap_or_else(|e| eprintln!("❌ Database connection failed: {}", e));
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
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

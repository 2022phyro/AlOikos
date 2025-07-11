use axum::Router;
use crate::auth::handlers::auth;
use axum::routing::post;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/signup", post(auth::signup))
}
pub fn api_routes() -> Router {
    Router::new()
        .nest("/auth", auth_routes())
}
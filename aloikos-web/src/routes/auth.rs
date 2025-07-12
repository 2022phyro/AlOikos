use axum::{middleware::from_fn, Router};
use crate::auth::{handlers::auth::{self, logout_view}, middleware::auth_middleware};
use axum::routing::post;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(auth::login_view))
        .route("/signup", post(auth::signup_view))
}
pub fn auth_protected_routes() -> Router {
    Router::new()
    .route(
        "/logout", post(logout_view)
    )
    .layer(from_fn(auth_middleware))
}

pub fn api_routes() -> Router {
    let combined = auth_routes().merge(auth_protected_routes());
    Router::new()
        .nest("/auth", combined)

}
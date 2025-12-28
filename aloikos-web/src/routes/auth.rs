use crate::auth::middleware::auth_middleware;
use crate::users::handlers::auth::{
    change_password_view, login_view, logout_view, me_view, otp_request_view, otp_verify_view,
    refresh_token_view, signup_view, verify_account_view,
};
use axum::routing::post;
use axum::{middleware::from_fn, Router};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(login_view))
        .route("/signup", post(signup_view))
        .route("/otp/request", post(otp_request_view))
        .route("/otp/verify", post(otp_verify_view))
        .route("/refresh", post(refresh_token_view))
        .route("/password/change", post(change_password_view))
        .route("/account/verify", post(verify_account_view))
}
pub fn auth_protected_routes() -> Router {
    Router::new()
        .route("/logout", post(logout_view))
        .route("/me", post(me_view))
        .layer(from_fn(auth_middleware))
}
pub fn api_routes() -> Router {
    let combined = auth_routes().merge(auth_protected_routes());
    Router::new().nest("/auth", combined)
}

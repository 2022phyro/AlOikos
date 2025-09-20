use utoipa::OpenApi;

pub mod models;
pub mod dto;
pub mod services;
pub mod handlers;
pub mod middleware;
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::auth::handlers::auth::login_view,
        crate::auth::handlers::auth::signup_view,
        crate::auth::handlers::auth::logout_view,
        crate::auth::handlers::auth::otp_request_view,
        crate::auth::handlers::auth::otp_verify_view,
        crate::auth::handlers::auth::refresh_token_view,
        crate::auth::handlers::auth::me_view,
        // crate::auth::handlers::auth::change_password_view,
        // crate::auth::handlers::auth::verify_account_view,

    ),
    components(
        schemas(
            crate::auth::dto::LoginRequestDto,
            crate::auth::dto::UserCreateDto,
            crate::auth::dto::LoginResponse,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints")
    )
)]
pub struct AuthApiDoc;
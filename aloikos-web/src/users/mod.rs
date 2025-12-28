pub mod models;
pub mod dto;
// pub mod services;
pub mod handlers;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::users::handlers::auth::login_view,
        crate::users::handlers::auth::signup_view,
        crate::users::handlers::auth::logout_view,
        crate::users::handlers::auth::otp_request_view,
        crate::users::handlers::auth::otp_verify_view,
        crate::users::handlers::auth::refresh_token_view,
        crate::users::handlers::auth::me_view,
        // crate::users::handlers::auth::change_password_view,
        // crate::users::handlers::auth::verify_account_view,

    ),
    components(
        schemas(
            crate::users::dto::LoginRequestDto,
            crate::users::dto::UserCreateDto,
            crate::users::dto::LoginResponse,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints")
    )
)]
pub struct AuthApiDoc;
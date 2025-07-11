use utoipa::OpenApi;

pub mod models;
pub mod dto;
pub mod services;
pub mod handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::auth::handlers::auth::login,
        crate::auth::handlers::auth::signup,
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
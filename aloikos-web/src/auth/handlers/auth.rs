//login
use crate::auth::{
    dto::{LoginRequestDto, UserCreateDto, LoginResponse},
    services::{authentication, user},
};
use utoipa;
use crate::config::CONFIG;
use axum::response::{IntoResponse};
use axum::{
    http::{
        header::{HeaderMap, SET_COOKIE},
        StatusCode,
    },
    response::Json,
};

/// Handles user login by validating credentials and issuing an access token.
///
/// On successful authentication, sets an HTTP-only refresh token cookie and returns
/// a JSON response containing the access token, its expiry, and the user ID.
/// Returns an error response with status 401 if authentication fails.
///
/// # Arguments
/// * `Json(login_request_dto)` - The login request payload containing email and password.
///
/// # Returns
/// An HTTP response with appropriate status, headers, and JSON body.
///

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequestDto,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "auth"
)]

pub async fn login(Json(login_request_dto): Json<LoginRequestDto>) -> impl IntoResponse {
    match authentication::login(&login_request_dto.email, login_request_dto.password).await {
        Ok(login_result) => {
            // Create HTTP-only cookie for refresh token
            let cookie_value = format!(
                "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
                login_result.refresh, CONFIG.jwt_refresh_duration
            );
            let mut headers = HeaderMap::new();
            headers.insert(SET_COOKIE, cookie_value.parse().unwrap());
            let response_body = LoginResponse {
                access: login_result.access,
                access_expiry: login_result.access_expiry,
                user_id: login_result.user_id,
            };

            (
                StatusCode::OK,
                headers,
                Json(serde_json::json!({
                    "success": true,
                    "data": response_body
                })),
            )
        }
        Err(err) => {
            let error_message = match err {
                sea_orm::DbErr::RecordNotFound(msg) => msg,
                sea_orm::DbErr::Custom(msg) => msg,
                _ => "An unexpected error occurred".to_string(),
            };

            (
                StatusCode::UNAUTHORIZED,
                HeaderMap::new(),
                Json(serde_json::json!({
                    "success": false,
                    "error": error_message
                })),
            )
        }
    }
}


/// Signs up a new user with the relevant details
#[utoipa::path(
    post,
    path = "/api/v1/auth/signup",
    request_body = UserCreateDto,
    responses(
        (status = 201, description = "User created successfully"),
        (status = 400, description = "Invalid request data")
    ),
    tag = "auth"
)]
pub async fn signup(Json(signup_request_dto): Json<UserCreateDto>) -> impl IntoResponse {
    match user::create(signup_request_dto).await {
        Ok(_) => (
            StatusCode::CREATED,
            HeaderMap::new(),
            Json(serde_json::json!({
                "success": true,
                "data": {
                    "message": "User successfully created"
                }
            })),
        ),
        Err(err) => {
            let error_message = match err {
                sea_orm::DbErr::RecordNotFound(msg) => msg,
                sea_orm::DbErr::Custom(msg) => msg,
                _ => "An unexpected error occurred".to_string(),
            };

            (
                StatusCode::BAD_REQUEST,
                HeaderMap::new(),
                Json(serde_json::json!({
                    "success": false,
                    "error": error_message
                })),
            )
        }
    }
}

//request otp
// verify otp
//refresh token
// change password
//view profile
// sign up
// resend verification email
// verify email

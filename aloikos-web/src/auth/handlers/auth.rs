//login
use crate::auth::models::prelude::UserActiveModel;
use crate::authentication::jwt::JwtRefreshToken;
use crate::config::CONFIG;
use crate::{
    auth::{
        dto::{LoginRequestDto, LoginResponse, LogoutDto, UserCreateDto},
        extractors::AuthContext,
        services,
    },
    authentication::jwt::{Blacklist, JwtAccessToken, Token},
    db::db,
    utils::errors::ApiError,
};
use axum::response::IntoResponse;
use axum::{
    http::{
        header::{HeaderMap, SET_COOKIE},
        StatusCode,
    },
    response::Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde_json::json;
use tower_cookies::Cookies;
use utoipa;

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

pub async fn login_view(Json(login_request_dto): Json<LoginRequestDto>) -> impl IntoResponse {
    match services::authentication::login(&login_request_dto.email, login_request_dto.password)
        .await
    {
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
pub async fn signup_view(Json(signup_request_dto): Json<UserCreateDto>) -> impl IntoResponse {
    match services::user::create(signup_request_dto).await {
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

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    request_body = LogoutDto,
    responses(
        (status = 200, description = "User logged out successfully"),
        (status = 400, description = "Something went wrong")
    ),
    tag = "auth"
)]
pub async fn logout_view(
    cookies: Cookies,
    AuthContext { user, token }: AuthContext,
    Json(logout_dto): Json<LogoutDto>,
) -> impl IntoResponse {
    let mut msg = "User successfully logged out";
    if logout_dto.all {
        let mut user: UserActiveModel = user.into();
        user.auth_change = Set(Some(Utc::now()));
        match user.update(db()).await {
            Ok(_) => {}
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!(ApiError::new(
                        "Something went wrong logging out from all devices"
                    ))),
                )
            }
        }
        msg = "User successfully logged out from all devices";
    }
    let access_token = JwtAccessToken::from_token(token);
    match access_token.blacklist().await {
        Ok(_) => {}
        Err(e) => {
            return (StatusCode::BAD_REQUEST, Json(json!(ApiError::new(e))));
        }
    }
    if let Some(cookie) = cookies.get("refresh") {
        let val = cookie.value();
        let refresh_token = JwtRefreshToken::from_token(val.to_string());
        match refresh_token.blacklist().await {
            Ok(_) => {}
            Err(e) => {
                return (StatusCode::BAD_REQUEST, Json(json!(ApiError::new(e))));
            }
        }
    } else {
        tracing::error!("Cookie not found in logout request")
    }
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "message": msg
        })),
    )
}
pub async fn request_otp_view() {}
pub async fn verify_otp_view() {}
pub async fn refresh_view() {}
pub async fn password_change_view() {}
pub async fn verify_account_view() {}
pub async fn resend_verification_view() {}
pub async fn me() {}

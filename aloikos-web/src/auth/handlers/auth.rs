use crate::auth::dto::{OtpRequestDto, OtpVerifyDto};
use crate::auth::models::prelude::UserActiveModel;
use crate::authentication::{jwt::JwtRefreshToken, otp::Otp};
use chrono::{Duration as ChronoDuration, Utc};
use crate::config::CONFIG;
use crate::{
    auth::{
        dto::{LoginRequestDto, LoginResponse, LogoutDto, UserCreateDto},
        services,
    },
    authentication::{
        extractors::AuthContext,
        jwt::{Blacklist, JwtAccessToken, Token},
    },
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
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde_json::json;
use tower_cookies::cookie::time::Duration;
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
    if let Some(mut cookie) = cookies.get("refresh") {
        let val = cookie.value();
        let refresh_token = JwtRefreshToken::from_token(val.to_string());
        cookie.set_max_age(Duration::seconds(0));
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

#[utoipa::path(
    post,
    path = "/api/v1/auth/otp/request",
    request_body = OtpRequestDto,
    responses(
        (status = 200, description = "Otp sent successfully"),
        (status = 400, description = "Something went wrong")
    ),
    tag = "auth"
)]
pub async fn otp_request_view(Json(otp_request_dto): Json<OtpRequestDto>) -> impl IntoResponse {
    let new_otp = Otp::new(
        &otp_request_dto.device_id,
        &otp_request_dto.email,
        otp_request_dto.otp_type,
    );
    match new_otp {
        Ok(otp) => {
            let code = otp.generate_otp();
            match code {
                Ok(_) => {
                    tracing::info!("OTP {:?} sent successfully to {}", &code, &otp.user_email);
                    return (
                        StatusCode::OK,
                        Json(serde_json::json!({
                            "success": true,
                            "message": "OTP sent successfully"
                        })),
                    );
                }
                Err(e) => {
                    tracing::error!("Failed to send OTP: {}", e);
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({
                            "success": false,
                            "error": "Failed to send OTP"
                        })),
                    );
                }
            }
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "error": err
            })),
        ),
    }
}
#[utoipa::path(
    post,
    path = "/api/v1/auth/otp/verify",
    request_body = OtpVerifyDto,
    responses(
        (status = 200, description = "Otp verified successfully"),
        (status = 400, description = "Something went wrong")
    ),
    tag = "auth"
)]
pub async fn otp_verify_view(Json(body): Json<OtpVerifyDto>) -> impl IntoResponse {
    let otp = Otp::new(&body.device_id, &body.email, body.otp_type);
    let mut headers: HeaderMap = HeaderMap::new();
    match otp {
        Ok(otp_instance) => {
            let status = otp_instance.verify_otp(&body.otp);
            match status {
                Ok(valid) => {
                    if valid {
                        tracing::info!("OTP verified successfully for {}", &body.email);
                        // Add a cookie stating otp is verified for this device
                        let cookie_value = format!(
                "otp_verified=true; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/", CONFIG.otp_expiry);
                        headers.insert(SET_COOKIE, cookie_value.parse().unwrap());
                        (
                            StatusCode::OK,
                            headers,
                            Json(serde_json::json!({
                                "success": true,
                                "message": "OTP verified successfully"
                            })),
                        )
                    } else {
                        (
                            StatusCode::BAD_REQUEST,
                            headers,
                            Json(serde_json::json!({
                                "success": false,
                                "error": "Invalid OTP"
                            })),
                        )
                    }
                }
                Err(e) => (
                    StatusCode::BAD_REQUEST,
                    headers,
                    Json(serde_json::json!({
                        "success": false,
                        "error": e
                    })),
                ),
            }
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            headers,
            Json(serde_json::json!({
                "success": false,
                "error": err
            })),
        ),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    responses(
        (status = 200, description = "Token refreshed successfully"),
        (status = 400, description = "Invalid request data")
    ),
    tag = "auth"
)]
pub async fn refresh_token_view(cookies: Cookies) -> impl IntoResponse {
    if let Some(cookie) = cookies.get("refresh") {
        let val = cookie.value();
        let refresh_token = JwtRefreshToken::from_token(val.to_string());
        match refresh_token.full_verify().await {
            Ok(token_data) => {
                let access = JwtAccessToken::new(
                    token_data.claims.sub.clone(),
                    token_data.claims.auth_change.clone(),
                );
                let refresh = JwtRefreshToken::new(
                    token_data.claims.sub.clone(),
                    token_data.claims.auth_change.clone(),
                );
                let access_expiry =
                    Utc::now() + ChronoDuration::seconds(CONFIG.jwt_access_duration as i64);
                let cookie_value = format!(
                    "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
                    refresh.token, CONFIG.jwt_refresh_duration
                );
                let mut headers = HeaderMap::new();
                headers.insert(SET_COOKIE, cookie_value.parse().unwrap());
                return Ok((
                    StatusCode::OK,
                    headers,
                    Json(json!({
                        "access": access.token,
                        "access_expiry": access_expiry,
                        "user_id": token_data.claims.sub,
                    })),
                ));
            }
            Err(e) => Err((
                StatusCode::UNAUTHORIZED,
                HeaderMap::new(),
                Json(json!(ApiError::new(format!(
                    "Invalid refresh token: {}",
                    e
                )))),
            )),
        }
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            HeaderMap::new(),
            Json(json!(ApiError::new("Refresh token cookie not found"))),
        ))
    }
}



pub async fn change_password_view(
    
) {}
pub async fn verify_account_view() {}


#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    responses(
        (status = 200, description = "User details fetched successfully"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth"
)]
pub async fn me_view(
    AuthContext { user, token }: AuthContext,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "user": user,
                "token": token
            }
        })),
    )
}

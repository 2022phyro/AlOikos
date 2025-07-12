use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::{
    http::StatusCode,
    middleware::{Next},
};
use serde_json::json;

use crate::auth::services::authentication::{verify_token, TokenType};
use crate::auth::services::user;
use crate::utils::errors::ApiError;

pub fn extract_token_from_headers(req: &Request) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok()) // Convert to &str
        .and_then(|s| s.strip_prefix("JWT "))
        .map(|s| s.to_string())
}
pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, impl IntoResponse> {
    let token = extract_token_from_headers(&req).ok_or((
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!(ApiError::new("Token could not be found"))),
    ))?;
    let claims = verify_token(TokenType::Access(token.clone()))
        .await
        .map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!(ApiError::new(format!("Token verification failed: {}", e)))),
            )
        })?;
    let user_id = claims.sub.parse::<i64>().map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!(ApiError::new("Invalid user ID in token"))),
        )
    })?;
    let user = user::get(user_id).await.map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!(ApiError::new(format!(
                "User lookup failed: {}",
                e
            )))),
        )
    })?;
    if let Some(auth_change) = user.auth_change {
        if claims.auth_change < auth_change {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!(ApiError::new(
                    "There was a recent change in authentication"
                ))),
            ));
        }
    }
    req.extensions_mut().insert(user);
    req.extensions_mut().insert(token);
    Ok(next.run(req).await)
}

// authentication middleware, returns user, and access_token, or return none for the auth system. Return false i
// Required Permission # Handled in the handler and not in the request
//

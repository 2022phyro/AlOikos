use crate::auth::models::prelude::UserModel;
use crate::utils::errors::ApiError;
use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
use axum::Json;
use tower_cookies::cookie::time::Duration;
use tower_cookies::{Cookie, Cookies};


#[derive(Clone)]
pub struct AuthContext {
    pub user: UserModel,
    pub token: String,
}


impl<S> FromRequestParts<S> for AuthContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ApiError>);
    
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = parts.extensions.get::<UserModel>().cloned().ok_or((
            StatusCode::UNAUTHORIZED,
            Json(ApiError::new("Authentication required")),
        ))?;
        
        let token = parts.extensions.get::<String>().cloned().ok_or((
            StatusCode::UNAUTHORIZED,
            Json(ApiError::new("Token not found")),
        ))?;
        
        Ok(Self { user, token })
    }
}

pub struct OtpRequiredContext {
    pub otp_verified: bool,
    pub otp_email: String,
}


impl<S> FromRequestParts<S> for OtpRequiredContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ApiError>);
    
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {

        let cookies = Cookies::from_request_parts(parts, _state).await.unwrap();
        
        let result = if let Some(cookie) = cookies.get("otp_verified") {
            if cookie.value() == "true" {
                if let Some(email_cookie) = cookies.get("otp_email") {
                    Ok(OtpRequiredContext {
                        otp_verified: true,
                        otp_email: email_cookie.value().to_string(),
                    })
                } else {
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(ApiError::new("OTP email not found")),
                    ))
                }
            } else {
                Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ApiError::new("OTP not verified")),
                ))
            }
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError::new("OTP required")),
            ))
        };
        
        // Clear both cookies regardless of the result
        // Create expired cookies to remove them
        let mut otp_verified_cookie = Cookie::new("otp_verified", "");
        otp_verified_cookie.set_max_age(Duration::seconds(0));
        otp_verified_cookie.set_path("/");
        
        let mut otp_email_cookie = Cookie::new("otp_email", "");
        otp_email_cookie.set_max_age(Duration::seconds(0));
        otp_email_cookie.set_path("/");
        
        cookies.add(otp_verified_cookie);
        cookies.add(otp_email_cookie);
        
        result
    }
}


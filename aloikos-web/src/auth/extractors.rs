use super::models::prelude::UserModel;
use crate::utils::errors::ApiError;
use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
use axum::Json;

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
        // If using middleware, the user should already be in extensions
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

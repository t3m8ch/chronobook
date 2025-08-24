use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::{AppState, models::error::ApiError};

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub organization_id: Option<Uuid>,
}

impl AuthUser {
    pub fn get_organization_id(&self) -> Option<Uuid> {
        self.organization_id
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        // Get authorization header
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| ApiError::unauthorized("Missing authorization header".to_string()))?;

        // Check Bearer prefix
        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            ApiError::unauthorized("Invalid authorization header format".to_string())
        })?;

        // Verify token
        let token_data = app_state
            .jwt_manager
            .verify_access_token(token)
            .map_err(|_| ApiError::unauthorized("Invalid or expired token".to_string()))?;

        Ok(AuthUser {
            user_id: token_data.claims.sub,
            organization_id: token_data.claims.org,
        })
    }
}

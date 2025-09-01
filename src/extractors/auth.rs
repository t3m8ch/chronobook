use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::{
    AppState,
    models::error::ApiError,
    services::jwt::{UserRole, UserType},
};

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub organization_id: Option<Uuid>,
    pub user_types: Vec<UserType>,
}

impl AuthUser {
    pub fn get_organization_id(&self) -> Option<Uuid> {
        self.organization_id
    }

    /// Check if user has access to a specific organization
    pub fn has_access_to_organization(&self, org_id: Uuid) -> bool {
        // Check if user is a customer of this organization
        if let Some(user_org_id) = self.organization_id {
            if user_org_id == org_id {
                return true;
            }
        }

        // Check if user is an employee of this organization
        for user_type in &self.user_types {
            match user_type {
                UserType::Employee {
                    org_id: employee_org_id,
                    ..
                } => {
                    if *employee_org_id == org_id {
                        return true;
                    }
                }
                UserType::Customer {
                    org_id: customer_org_id,
                    ..
                } => {
                    if *customer_org_id == org_id {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Check if user has specific role in organization
    pub fn has_role_in_organization(&self, org_id: Uuid, required_roles: &[UserRole]) -> bool {
        for user_type in &self.user_types {
            if let UserType::Employee {
                org_id: employee_org_id,
                roles,
                ..
            } = user_type
            {
                if *employee_org_id == org_id {
                    for role in roles {
                        if required_roles.contains(role) {
                            return true;
                        }
                    }
                }
            }
        }
        false
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
            user_types: token_data.claims.user_types,
        })
    }
}

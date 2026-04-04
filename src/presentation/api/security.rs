use std::collections::HashMap;

use axum::{
    extract::{FromRequestParts, Path},
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use chrono::Utc;
use jsonwebtoken::{Algorithm, Validation, decode};
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};

use crate::{
    domain::repositories::profile_repo::ProfileRepository,
    presentation::api::{service::AppState, utils::AppErrorResponse},
};

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, EnumString, Display, AsRefStr,
)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    #[strum(serialize = "profile:admin")]
    #[serde(rename = "profile:admin")]
    ProfileAdmin,

    #[strum(serialize = "profile:create")]
    #[serde(rename = "profile:create")]
    ProfileCreate,

    #[strum(serialize = "profile:delete")]
    #[serde(rename = "profile:delete")]
    ProfileDelete,

    #[strum(serialize = "profile:read")]
    #[serde(rename = "profile:read")]
    ProfileRead,

    #[strum(serialize = "profile:update")]
    #[serde(rename = "profile:update")]
    ProfileUpdate,

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub aud: Vec<String>,
    pub scopes: Vec<Scope>,
    pub email: String,
}

impl Claims {
    pub fn has_scope(&self, required_scopes: Vec<Scope>) -> bool {
        self.scopes
            .iter()
            .any(|scope| required_scopes.contains(scope))
    }
}

impl<R> FromRequestParts<AppState<R>> for Claims
where
    R: ProfileRepository,
{
    type Rejection = AppErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState<R>,
    ) -> Result<Self, Self::Rejection> {
        // 1. Extrai o header Authorization
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AppErrorResponse {
                message: "Missing token".into(),
                status_code: StatusCode::UNAUTHORIZED.as_u16(),
                timestamp: Utc::now(),
                code: None,
                details: None,
            })?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppErrorResponse {
                message: "Invalid token format".into(),
                status_code: StatusCode::UNAUTHORIZED.as_u16(),
                timestamp: Utc::now(),
                code: None,
                details: None,
            })?;

        // 2. Configura a validação
        let mut validation = Validation::new(Algorithm::EdDSA);

        // Defina qual "ID" este microserviço aceita
        // O token vindo do seu IAM-Service DEVE ter "profile-service" no campo 'aud'
        validation.set_audience(&["profile-service"]);

        // Garante que a validação de expiração e assinatura ocorra
        validation.validate_exp = true;

        let token_data =
            decode::<Claims>(token, &state.decoding_key, &validation).map_err(|e| {
                AppErrorResponse {
                    message: format!("Token validation error: {}", e),
                    status_code: StatusCode::UNAUTHORIZED.as_u16(),
                    timestamp: Utc::now(),
                    code: None,
                    details: None,
                }
            })?;

        // Verifica se o token foi emitido para o Profile Service
        if !token_data
            .claims
            .aud
            .contains(&"profile-service".to_string())
        {
            return Err(AppErrorResponse {
                message: "Invalid token audience".into(),
                status_code: StatusCode::UNAUTHORIZED.as_u16(),
                timestamp: Utc::now(),
                code: None,
                details: None,
            });
        }

        Ok(token_data.claims)
    }
}

pub struct CreateClaims(Claims);

impl<R> FromRequestParts<AppState<R>> for CreateClaims
where
    R: ProfileRepository,
{
    type Rejection = AppErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState<R>,
    ) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;

        if !claims.has_scope(vec![Scope::ProfileAdmin, Scope::ProfileCreate]) {
            return Err(AppErrorResponse {
                message: "Access denied: Requires admin privileges".into(),
                status_code: StatusCode::FORBIDDEN.as_u16(),
                timestamp: Utc::now(),
                code: None,
                details: None,
            });
        }

        Ok(CreateClaims(claims))
    }
}

pub struct UpdateClaims(Claims);

impl<R> FromRequestParts<AppState<R>> for UpdateClaims
where
    R: ProfileRepository,
{
    type Rejection = AppErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState<R>,
    ) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;

        let Path(path_params): Path<HashMap<String, String>> =
            Path::from_request_parts(parts, state)
                .await
                .map_err(|_| AppErrorResponse {
                    message: "Missing path parameters".into(),
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    timestamp: Utc::now(),
                    code: None,
                    details: None,
                })?;

        let target_id = path_params.get("id").ok_or(AppErrorResponse {
            message: "Missing 'id' parameter".into(),
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            timestamp: Utc::now(),
            code: None,
            details: None,
        })?;

        let is_owner = &claims.sub == target_id;
        let has_scope = claims.has_scope(vec![Scope::ProfileAdmin, Scope::ProfileUpdate]);

        if !(is_owner || has_scope) {
            return Err(AppErrorResponse {
                message: "Access denied: Requires update privileges".into(),
                status_code: StatusCode::FORBIDDEN.as_u16(),
                timestamp: Utc::now(),
                code: None,
                details: None,
            });
        }

        Ok(UpdateClaims(claims))
    }
}

pub struct ReadClaims(Claims);

impl<R> FromRequestParts<AppState<R>> for ReadClaims
where
    R: ProfileRepository,
{
    type Rejection = AppErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState<R>,
    ) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;

        let Path(path_params): Path<HashMap<String, String>> =
            Path::from_request_parts(parts, state)
                .await
                .map_err(|_| AppErrorResponse {
                    message: "Missing path parameters".into(),
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    timestamp: Utc::now(),
                    code: None,
                    details: None,
                })?;

        let target_id = path_params.get("id").ok_or(AppErrorResponse {
            message: "Missing 'id' parameter".into(),
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            timestamp: Utc::now(),
            code: None,
            details: None,
        })?;

        let is_owner = &claims.sub == target_id;
        let has_scope = claims.has_scope(vec![Scope::ProfileAdmin, Scope::ProfileRead]);

        if !(is_owner || has_scope) {
            return Err(AppErrorResponse {
                message: "Access denied: Requires read privileges".into(),
                status_code: StatusCode::FORBIDDEN.as_u16(),
                timestamp: Utc::now(),
                code: None,
                details: None,
            });
        }

        Ok(ReadClaims(claims))
    }
}

pub struct DeleteClaims(Claims);

impl<R> FromRequestParts<AppState<R>> for DeleteClaims
where
    R: ProfileRepository,
{
    type Rejection = AppErrorResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState<R>,
    ) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;

        if !claims.has_scope(vec![Scope::ProfileAdmin, Scope::ProfileDelete]) {
            return Err(AppErrorResponse {
                message: "Access denied: Requires delete privileges".into(),
                status_code: StatusCode::FORBIDDEN.as_u16(),
                timestamp: Utc::now(),
                code: None,
                details: None,
            });
        }

        Ok(DeleteClaims(claims))
    }
}

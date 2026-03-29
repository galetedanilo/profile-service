use axum::{
    Json,
    body::Body,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde::de::DeserializeOwned;
use validify::Validify;

use crate::domain::models::profile::ProfileError;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppErrorResponse {
    pub message: String,
    pub status_code: u16,
    pub timestamp: DateTime<Utc>,
    pub code: Option<String>,
    pub details: Option<String>,
}

impl IntoResponse for AppErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code =
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status_code, axum::Json(self)).into_response()
    }
}

impl From<ProfileError> for AppErrorResponse {
    fn from(error: ProfileError) -> Self {
        let (message, status_code, details) = match error {
            ProfileError::AlreadyExists(details) => (
                "Profile already exists".to_string(),
                409,
                Some(details.to_string()),
            ),
            ProfileError::VersionConflict(details) => (
                "Version conflict".to_string(),
                409,
                Some(details.to_string()),
            ),
            ProfileError::InvalidData(details) => (
                "Validation error".to_string(),
                400,
                Some(details.to_string()),
            ),
            ProfileError::NotFound(details) => (
                "Profile not found".to_string(),
                404,
                Some(details.to_string()),
            ),
            ProfileError::Unknown(details) => {
                ("Database error".to_string(), 500, Some(details.to_string()))
            }
        };

        Self {
            message,
            status_code,
            timestamp: Utc::now(),
            code: None,
            details,
        }
    }
}

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validify,
{
    type Rejection = AppErrorResponse;

    async fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) =
            Json::<T>::from_request(req, _state)
                .await
                .map_err(|err| AppErrorResponse {
                    message: "Invalid JSON".to_string(),
                    status_code: 400,
                    timestamp: Utc::now(),
                    code: None,
                    details: Some(err.to_string()),
                })?;

        value.validate().map_err(|e| {
            let error_messages: Vec<String> = e
                .errors()
                .into_iter()
                .map(|err| {
                    format!(
                        "'{}': '{}'",
                        err.field_name().unwrap_or_default(),
                        err.message().unwrap_or_default()
                    )
                })
                .collect();

            AppErrorResponse {
                message: "Validation error".to_string(),
                status_code: 400,
                timestamp: Utc::now(),
                code: None,
                details: Some(error_messages.join(", ")),
            }
        })?;

        Ok(ValidatedJson(value))
    }
}

use axum::{http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::Serialize;

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
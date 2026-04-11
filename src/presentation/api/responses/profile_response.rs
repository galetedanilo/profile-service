use axum::response::IntoResponse;
use serde::Serialize;

use crate::domain::models::profile::Profile;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub version: u64,
}

impl From<Profile> for ProfileResponse {
    fn from(profile: Profile) -> Self {
        Self {
            id: profile.id().to_string(),
            email: profile.email().to_string(),
            first_name: profile.first_name().map(|f| f.to_string()),
            last_name: profile.last_name().map(|l| l.to_string()),
            bio: profile.bio().map(|b| b.to_string()),
            profile_image_url: profile.profile_image_url().map(|url| url.to_string()),
            created_at: profile.created_at().to_rfc3339(),
            updated_at: profile.updated_at().map(|u| u.to_rfc3339()),
            version: profile.version(),
        }
    }
}

impl IntoResponse for ProfileResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

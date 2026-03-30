use axum::response::IntoResponse;
use serde::Serialize;

use crate::domain::models::profile::Profile;

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
    pub version: u64,
}

impl From<Profile> for ProfileResponse {
    fn from(profile: Profile) -> Self {
        Self {
            id: profile.id().to_string(),
            first_name: profile.first_name().map(|f| f.to_string()),
            last_name: profile.last_name().map(|l| l.to_string()),
            bio: profile.bio().map(|b| b.to_string()),
            profile_image_url: profile.profile_image_url().map(|url| url.to_string()),
            version: profile.version(),
        }
    }
}

impl IntoResponse for ProfileResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

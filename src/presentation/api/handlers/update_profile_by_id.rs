use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    application::dtos::update_profile_input::UpdateProfileInput,
    domain::repositories::profile_repo::ProfileRepository,
    presentation::api::{
        handlers::requests::UpdateProfileRequest, service::AppState, utils::AppErrorResponse,
    },
};

pub async fn update_profile_by_id_handler<R: ProfileRepository>(
    State(state): State<AppState<R>>,
    Path(id): Path<String>,
    Json(input): Json<UpdateProfileRequest>,
) -> Result<StatusCode, AppErrorResponse> {
    let command = UpdateProfileInput::try_new(
        id,
        input.first_name,
        input.last_name,
        input.bio,
        input.profile_image_url,
        input.version,
    )?;

    state
        .update_profile_use_case
        .execute(command)
        .await
        .map_err(AppErrorResponse::from)
        .map(|_| StatusCode::OK)
}

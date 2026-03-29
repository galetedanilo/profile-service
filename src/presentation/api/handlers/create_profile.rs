use axum::{extract::State, http::StatusCode};

use crate::{
    application::dtos::create_profile_input::CreateProfileInput,
    domain::repositories::profile_repo::ProfileRepository,
    presentation::api::{
        handlers::requests::CreateProfileRequest,
        service::AppState,
        utils::{AppErrorResponse, ValidatedJson},
    },
};

pub async fn create_profile_handler<R: ProfileRepository>(
    State(state): State<AppState<R>>,
    ValidatedJson(input): ValidatedJson<CreateProfileRequest>,
) -> Result<StatusCode, AppErrorResponse> {
    let command = CreateProfileInput::try_new(input.id, input.email)?;

    state
        .create_profile_use_case
        .execute(command)
        .await
        .map_err(AppErrorResponse::from)
        .map(|_| StatusCode::CREATED)
}

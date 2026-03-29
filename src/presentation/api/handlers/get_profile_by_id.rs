use axum::extract::{Path, State};

use crate::{
    application::dtos::get_profile_by_id_input::GetProfileByIdInput,
    domain::repositories::profile_repo::ProfileRepository,
    presentation::api::{
        handlers::responses::ProfileResponse, service::AppState, utils::AppErrorResponse,
    },
};

pub async fn get_profile_by_id_handler<R: ProfileRepository>(
    State(state): State<AppState<R>>,
    Path(id): Path<String>,
) -> Result<ProfileResponse, AppErrorResponse> {
    let command = GetProfileByIdInput::try_new(id)?;

    state
        .get_profile_by_id_use_case
        .execute(command)
        .await
        .map_err(AppErrorResponse::from)
        .map(|p| ProfileResponse::from(p))
}

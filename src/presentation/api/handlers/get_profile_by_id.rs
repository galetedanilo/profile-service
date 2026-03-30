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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{Router, body::Body, extract::Request, http::StatusCode, routing::get};
    use fake::{Fake, faker::internet::en::FreeEmail};
    use tower::ServiceExt;

    use crate::{
        domain::{
            models::profile::Profile, object_values::email::Email,
            repositories::profile_repo::MockProfileRepository,
        },
        presentation::api::handlers::tests::SharedMockRepository,
    };

    use super::*;

    #[tokio::test]
    async fn when_profile_exists_should_return_profile() {
        let mut mock_repo = MockProfileRepository::new();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .returning(|id| {
                Ok(Some(Profile::new_from(
                    id.clone(),
                    Email::try_from(FreeEmail().fake::<String>()).unwrap(),
                    None,
                    None,
                    None,
                    None,
                    2,
                )))
            });

        let shared_repo = SharedMockRepository(Arc::new(mock_repo));

        let app_state = AppState::new(Arc::new(shared_repo));

        let app = Router::new()
            .route("/profiles/{id}", get(get_profile_by_id_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("GET")
            .uri("/profiles/123e4567-e89b-12d3-a456-426614174000")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn when_profile_does_not_exist_should_return_not_found() {
        let mut mock_repo = MockProfileRepository::new();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .returning(|_| Ok(None));

        let shared_repo = SharedMockRepository(Arc::new(mock_repo));

        let app_state = AppState::new(Arc::new(shared_repo));

        let app = Router::new()
            .route("/profiles/{id}", get(get_profile_by_id_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("GET")
            .uri("/profiles/123e4567-e89b-12d3-a456-426614174000")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}

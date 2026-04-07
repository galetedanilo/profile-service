use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    application::dtos::update_profile_input::UpdateProfileInput,
    domain::repositories::profile_repo::ProfileRepository,
    presentation::api::{
        handlers::requests::UpdateProfileRequest, security::UpdateClaims, service::AppState,
        utils::AppErrorResponse,
    },
};

pub async fn update_profile_by_id_handler<R: ProfileRepository>(
    _: UpdateClaims,
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
    )?;

    state
        .update_profile_use_case
        .execute(command)
        .await
        .map_err(AppErrorResponse::from)
        .map(|_| StatusCode::OK)
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use axum::{Router, body::Body, http::Request, routing::put};
    use fake::Fake;
    use fake::faker::name::en::{FirstName, LastName};
    use serde_json::json;
    use tower::ServiceExt;

    use crate::domain::models::profile::Profile;
    use crate::domain::object_values::email::Email;
    use crate::presentation::api::handlers::tests::{create_test_token, get_test_decoding_key};
    use crate::{
        domain::repositories::profile_repo::MockProfileRepository,
        presentation::api::{handlers::tests::SharedMockRepository, service::AppState},
    };

    use super::*;

    #[tokio::test]
    pub async fn when_profile_does_not_exist_shoul_return_not_found() {
        let mut mock_repo = MockProfileRepository::new();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(None));

        let shared_repo = SharedMockRepository(Arc::new(mock_repo));

        let decoding_key = get_test_decoding_key();

        let app_state = AppState::new(Arc::new(shared_repo), Arc::new(decoding_key));
        let token = create_test_token();

        let app = Router::new()
            .route("/profiles/{id}", put(update_profile_by_id_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("PUT")
            .uri("/profiles/123e4567-e89b-12d3-a456-426614174000")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::from(
                json!({
                    "first_name": FirstName().fake::<String>(),
                    "last_name": LastName().fake::<String>(),
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    pub async fn when_profile_exists_should_return_ok() {
        let mut mock_repo = MockProfileRepository::new();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .returning(|id| {
                Ok(Some(Profile::from_parts(
                    id.clone(),
                    Email::try_from("john.doe@example.com").unwrap(),
                    None,
                    None,
                    None,
                    None,
                    chrono::Utc::now(),
                    None,
                    2,
                )))
            });

        mock_repo.expect_save().times(1).returning(|_| Ok(()));

        let shared_repo = SharedMockRepository(Arc::new(mock_repo));

        let decoding_key = get_test_decoding_key();

        let app_state = AppState::new(Arc::new(shared_repo), Arc::new(decoding_key));
        let token = create_test_token();

        let app = Router::new()
            .route("/profiles/{id}", put(update_profile_by_id_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("PUT")
            .uri("/profiles/123e4567-e89b-12d3-a456-426614174000")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::from(
                json!({
                    "first_name": FirstName().fake::<String>(),
                    "last_name": LastName().fake::<String>(),
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn when_version_mismatch_should_return_conflict() {
        let mut mock_repo = MockProfileRepository::new();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .returning(|id| {
                Ok(Some(Profile::from_parts(
                    id.clone(),
                    Email::try_from("john.doe@example.com").unwrap(),
                    None,
                    None,
                    None,
                    None,
                    chrono::Utc::now(),
                    Some(chrono::Utc::now() + chrono::Duration::hours(1)),
                    2,
                )))
            });

        let shared_repo = SharedMockRepository(Arc::new(mock_repo));

        let decoding_key = get_test_decoding_key();

        let app_state = AppState::new(Arc::new(shared_repo), Arc::new(decoding_key));
        let token = create_test_token();

        let app = Router::new()
            .route("/profiles/{id}", put(update_profile_by_id_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("PUT")
            .uri("/profiles/123e4567-e89b-12d3-a456-426614174000")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::from(
                json!({
                    "first_name": FirstName().fake::<String>(),
                    "last_name": LastName().fake::<String>(),
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
}

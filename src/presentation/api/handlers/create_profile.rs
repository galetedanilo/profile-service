use axum::{extract::State, http::StatusCode};

use crate::{
    application::inputs::create_profile_input::CreateProfileInput,
    domain::repositories::profile_repo::ProfileRepository,
    presentation::api::{
        helpers::{
            security::CreateClaims,
            utils::{AppErrorResponse, ValidatedJson},
        },
        requests::create_profile_request::CreateProfileRequest,
        service::AppState,
    },
};

pub async fn create_profile_handler<R: ProfileRepository>(
    _: CreateClaims,
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{Router, body::Body, http::Request, routing::post};
    use serde_json::json;
    use tower::ServiceExt;
    use uuid::Uuid;

    use super::*;

    use crate::{
        domain::{
            models::profile::Profile,
            object_values::{email::Email, id::Id},
            repositories::profile_repo::MockProfileRepository,
        },
        presentation::api::handlers::tests::{
            SharedMockRepository, create_test_token, get_test_decoding_key,
        },
    };

    #[tokio::test]
    async fn when_valid_profile_data_is_provided_then_profile_is_created() {
        let mut mock_repo = MockProfileRepository::new();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .returning(|_| Ok(None));

        mock_repo.expect_save().times(1).returning(|_| Ok(()));

        // Envolve o mock no Wrapper clonável
        let shared_repo = SharedMockRepository(Arc::new(mock_repo));

        let decoding_key = get_test_decoding_key();

        let app_state = AppState::new(Arc::new(shared_repo), Arc::new(decoding_key));

        let app = Router::new()
            .route("/profiles", post(create_profile_handler))
            .with_state(app_state);

        let token = create_test_token();

        // Constrói a requisição HTTP simulada
        let request = Request::builder()
            .method("POST")
            .uri("/profiles")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::from(
                json!({
                    "id": Uuid::now_v7().to_string(),
                    "email": "teste@email.com"
                })
                .to_string(),
            ))
            .unwrap();

        // EXECUÇÃO: Envia a requisição para o App
        // O método oneshot() consome o app e retorna a resposta
        let response = app.oneshot(request).await.unwrap();

        // 5. Assertions
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn when_profile_already_exists_then_returns_conflict() {
        let mut mock_repo = MockProfileRepository::new();

        let existing_profile =
            Profile::new(Id::generate(), Email::try_from("teste@email.com").unwrap());
        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .returning(move |_| Ok(Some(existing_profile.clone())));

        let shared_repo = SharedMockRepository(Arc::new(mock_repo));

        let decoding_key = get_test_decoding_key();

        let app_state = AppState::new(Arc::new(shared_repo), Arc::new(decoding_key));

        let profile_id = Uuid::now_v7().to_string();
        let token = create_test_token();

        let app = Router::new()
            .route("/profiles", post(create_profile_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("POST")
            .uri("/profiles")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", token))
            .body(Body::from(
                json!({
                    "id": profile_id,
                    "email": "teste@email.com"
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
}

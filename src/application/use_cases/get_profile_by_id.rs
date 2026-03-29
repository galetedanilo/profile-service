use std::sync::Arc;

use crate::{
    application::dtos::get_profile_by_id_input::GetProfileByIdInput,
    domain::{
        models::profile::{Profile, ProfileError},
        repositories::profile_repo::ProfileRepository,
    },
};

#[derive(Clone)]
pub struct GetProfileByIdUseCase<R: ProfileRepository + Send + Sync> {
    repository: Arc<R>,
}

impl<R: ProfileRepository + Send + Sync> GetProfileByIdUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, input: GetProfileByIdInput) -> Result<Profile, ProfileError> {
        self.repository
            .get_profile_by_id(&input.id)
            .await
            .map_err(ProfileError::from)
            .and_then(|profile_opt| {
                profile_opt.ok_or_else(|| ProfileError::NotFound(input.id.to_string()))
            })
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, faker::internet::en::FreeEmail};
    use uuid::Uuid;

    use super::*;
    use crate::domain::{
        object_values::email::Email,
        repositories::profile_repo::{MockProfileRepository, ProfileRepositoryError},
    };

    #[tokio::test]
    async fn when_profile_not_found_should_return_not_found_error() {
        let mut mock_repo = MockProfileRepository::new();

        let input = GetProfileByIdInput::try_new(Uuid::now_v7().to_string()).unwrap();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(None));

        let use_case = GetProfileByIdUseCase::new(Arc::new(mock_repo));

        let result = use_case.execute(input).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(ProfileError::NotFound(_))));
    }

    #[tokio::test]
    async fn when_profile_found_should_return_profile() {
        let mut mock_repo = MockProfileRepository::new();

        let input = GetProfileByIdInput::try_new(Uuid::now_v7().to_string()).unwrap();

        let email = FreeEmail().fake::<String>();

        let profile = Profile::new(input.id.clone(), Email::try_from(email).unwrap());

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(Some(profile.clone())));

        let use_case = GetProfileByIdUseCase::new(Arc::new(mock_repo));

        let result = use_case.execute(input).await;

        assert!(result.is_ok());

        let profile_result = result.unwrap();

        assert_eq!(profile_result, profile);
    }

    #[tokio::test]
    async fn when_repository_error_should_return_repository_error() {
        let mut mock_repo = MockProfileRepository::new();

        let input = GetProfileByIdInput::try_new(Uuid::now_v7().to_string()).unwrap();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Err(ProfileRepositoryError::Unknown("mock error".into())));

        let use_case = GetProfileByIdUseCase::new(Arc::new(mock_repo));

        let result = use_case.execute(input).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(ProfileError::Unknown(_))));
    }
}

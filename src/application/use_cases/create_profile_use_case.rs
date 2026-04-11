use std::sync::Arc;

use crate::{
    application::inputs::create_profile_input::CreateProfileInput,
    domain::{
        models::profile::{Profile, ProfileError},
        repositories::profile_repo::ProfileRepository,
    },
};

#[derive(Clone)]
pub struct CreateProfileUseCase<R: ProfileRepository + Send + Sync> {
    repository: Arc<R>,
}

impl<R: ProfileRepository + Send + Sync> CreateProfileUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, input: CreateProfileInput) -> Result<(), ProfileError> {
        if let Ok(Some(_)) = self.repository.get_profile_by_id(&input.id).await {
            return Err(ProfileError::AlreadyExists(input.id.to_string()));
        }

        let profile = Profile::new(input.id, input.email);

        self.repository.save(&profile).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, faker::internet::en::FreeEmail};

    use super::*;
    use crate::domain::{
        object_values::{email::Email, id::Id},
        repositories::profile_repo::MockProfileRepository,
    };

    #[tokio::test]
    async fn when_input_exist_profile_should_return_already_exists_error() {
        let mut mock_repo = MockProfileRepository::new();

        let input = CreateProfileInput {
            id: Id::generate(),
            email: Email::try_new(FreeEmail().fake()).unwrap(),
        };

        let profile = Profile::new(Id::generate(), Email::try_new(FreeEmail().fake()).unwrap());

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(Some(profile)));

        let use_case = CreateProfileUseCase::new(Arc::new(mock_repo));

        let result = use_case.execute(input).await;

        assert!(matches!(result, Err(ProfileError::AlreadyExists(_))));
    }

    #[tokio::test]
    async fn when_input_valid_and_not_exist_should_return_ok() {
        let mut mock_repo = MockProfileRepository::new();

        let input = CreateProfileInput {
            id: Id::generate(),
            email: Email::try_new(FreeEmail().fake()).unwrap(),
        };

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(None));

        mock_repo.expect_save().times(1).return_const(Ok(()));

        let use_case = CreateProfileUseCase::new(Arc::new(mock_repo));

        let result = use_case.execute(input).await;

        assert!(result.is_ok())
    }
}

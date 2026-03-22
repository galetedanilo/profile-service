use crate::{
    application::dtos::create_profile_input::CreateProfileInput,
    domain::{
        models::profile::{Profile, ProfileError},
        object_values::{email::Email, id::Id},
        repositories::profile_repo::ProfileRepository,
    },
};
use validify::Validify;

pub struct CreateProfileUseCase<R: ProfileRepository + Send + Sync> {
    repository: R,
}

impl<R: ProfileRepository + Send + Sync> CreateProfileUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, mut input: CreateProfileInput) -> Result<(), ProfileError> {
        input
            .validify()
            .map_err(|e| ProfileError::InvalidData(e.to_string()))?;

        let id = Id::try_from(input.id.clone())?;
        let email = Email::try_from(input.email.clone())?;

        if let Ok(Some(_)) = self.repository.get_profile_by_id(&id).await {
            return Err(ProfileError::AlreadyExists(input.id));
        }

        let profile = Profile::new(id, email);

        self.repository.save(&profile).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, faker::internet::en::FreeEmail};
    use uuid::Uuid;

    use super::*;
    use crate::domain::repositories::profile_repo::MockProfileRepository;

    #[tokio::test]
    async fn when_input_invalid_should_return_invalid_data_error() {
        let mock_repo = MockProfileRepository::new();

        let use_case = CreateProfileUseCase::new(mock_repo);

        let input = CreateProfileInput {
            id: "".to_string(),
            email: "".to_string(),
        };
        let result = use_case.execute(input).await;

        assert!(matches!(result, Err(ProfileError::InvalidData(_))));
    }

    #[tokio::test]
    async fn when_input_id_invalid_should_return_invalid_data_error() {
        let mock_repo = MockProfileRepository::new();

        let use_case = CreateProfileUseCase::new(mock_repo);

        let input = CreateProfileInput {
            id: "".to_string(),
            email: FreeEmail().fake(),
        };

        let result = use_case.execute(input).await;

        assert!(matches!(result, Err(ProfileError::InvalidData(_))));
    }

    #[tokio::test]
    async fn when_input_email_invalid_should_return_invalid_data_error() {
        let mock_repo = MockProfileRepository::new();

        let use_case = CreateProfileUseCase::new(mock_repo);

        let input = CreateProfileInput {
            id: Uuid::now_v7().to_string(),
            email: "".to_string(),
        };

        let result = use_case.execute(input).await;

        assert!(matches!(result, Err(ProfileError::InvalidData(_))));
    }

    #[tokio::test]
    async fn when_input_exist_profile_should_return_already_exists_error() {
        let mut mock_repo = MockProfileRepository::new();

        let input = CreateProfileInput {
            id: Uuid::now_v7().to_string(),
            email: FreeEmail().fake(),
        };

        let profile = Profile::new(Id::generate(), Email::try_new(FreeEmail().fake()).unwrap());

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(Some(profile)));

        let use_case = CreateProfileUseCase::new(mock_repo);

        let result = use_case.execute(input).await;

        assert!(matches!(result, Err(ProfileError::AlreadyExists(_))));
    }

    #[tokio::test]
    async fn when_input_valid_and_not_exist_should_return_ok() {
        let mut mock_repo = MockProfileRepository::new();

        let input = CreateProfileInput {
            id: Uuid::now_v7().to_string(),
            email: FreeEmail().fake(),
        };

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(None));

        mock_repo.expect_save().times(1).return_const(Ok(()));

        let use_case = CreateProfileUseCase::new(mock_repo);

        let result = use_case.execute(input).await;

        assert!(result.is_ok())
    }
}

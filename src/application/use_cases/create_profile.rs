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

        let id =
            Id::try_from(input.id.clone()).map_err(|e| ProfileError::InvalidData(e.to_string()))?;
        let email = Email::try_from(input.email.clone())
            .map_err(|e| ProfileError::InvalidData(e.to_string()))?;

        if let Ok(Some(_)) = self.repository.get_profile_by_id(&id).await {
            return Err(ProfileError::AlreadyExists(input.id));
        }

        let profile = Profile::new(id, email, None, None, None, None);
        self.repository
            .save(&profile)
            .await
            .map_err(|e| ProfileError::InvalidData(e.to_string()))?;

        Ok(())
    }
}

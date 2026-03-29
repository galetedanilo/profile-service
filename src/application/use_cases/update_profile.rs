use std::sync::Arc;

use crate::{
    application::dtos::update_profile_input::UpdateProfileInput,
    domain::{
        models::profile::{Profile, ProfileError},
        repositories::profile_repo::ProfileRepository,
    },
};

#[derive(Clone)]
pub struct UpdateProfileUseCase<R: ProfileRepository + Send + Sync> {
    repository: Arc<R>,
}

impl<R: ProfileRepository + Send + Sync> UpdateProfileUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: UpdateProfileInput,
    ) -> Result<Option<Profile>, ProfileError> {
        let profile = self.repository.get_profile_by_id(&input.id).await?;

        if let Some(mut profile) = profile {
            if input.version != profile.version() {
                return Err(ProfileError::VersionConflict(
                    "Version mismatch".to_string(),
                ));
            }

            profile.update_profile(
                input.first_name,
                input.last_name,
                input.bio,
                input.profile_image_url,
            );

            self.repository.save(&profile).await?;

            Ok(Some(profile))
        } else {
            Err(ProfileError::NotFound(input.id.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {

    use uuid::Uuid;

    use super::*;

    use crate::domain::{
        object_values::{
            bio::Bio, email::Email, first_name::FirstName, id::Id, image_url::ImageUrl,
            last_name::LastName,
        },
        repositories::profile_repo::MockProfileRepository,
    };

    #[tokio::test]
    pub async fn when_version_mismatch_should_return_version_conflict_error() {
        let mut mock_repo = MockProfileRepository::new();

        let id = Uuid::now_v7().to_string();

        let existing_profile = Profile::new_from(
            Id::try_from(id.clone()).unwrap(),
            Email::try_from("existing@example.com").unwrap(),
            None,
            None,
            None,
            None,
            3,
        );

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(Some(existing_profile)));

        let use_case = UpdateProfileUseCase::new(Arc::new(mock_repo));

        let input = UpdateProfileInput::try_new(id.clone(), None, None, None, None, 2).unwrap();

        let result = use_case.execute(input).await;

        assert!(matches!(result, Err(ProfileError::VersionConflict(_))));
    }

    #[tokio::test]
    pub async fn when_profile_does_not_exist_should_return_not_found_error() {
        let mut mock_repo = MockProfileRepository::new();

        let non_existent_id = Uuid::now_v7().to_string();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(None));

        let use_case = UpdateProfileUseCase::new(Arc::new(mock_repo));

        let input = UpdateProfileInput::try_new(non_existent_id.clone(), None, None, None, None, 2)
            .unwrap();

        let result = use_case.execute(input).await;

        assert!(matches!(result, Err(ProfileError::NotFound(_))));
    }

    #[tokio::test]
    pub async fn when_input_is_valid_and_profile_exists_should_update_profile() {
        let mut mock_repo = MockProfileRepository::new();

        let existing_id = Uuid::now_v7().to_string();

        let fake_email = "jane.smith@example.com".to_string();
        let fake_first_name = "Jane".to_string();
        let fake_last_name = "Smith".to_string();
        let fake_bio = "Hi, I'm Jane!".to_string();
        let fake_profile_image_url = "http://example.com/new_profile.jpg".to_string();

        let id = Id::try_from(existing_id.clone()).unwrap();
        let email = Email::try_from(fake_email.clone()).unwrap();
        let first_name = FirstName::try_from(fake_first_name.clone()).unwrap();
        let last_name = LastName::try_from(fake_last_name.clone()).unwrap();
        let bio = Bio::try_from(fake_bio.clone()).unwrap();
        let profile_image_url = ImageUrl::try_from(fake_profile_image_url.clone()).unwrap();

        let existing_profile = Profile::new_from(
            id,
            email,
            Some(first_name),
            Some(last_name),
            Some(bio),
            Some(profile_image_url),
            1,
        );

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(Some(existing_profile)));

        mock_repo.expect_save().times(1).return_const(Ok(()));

        let use_case = UpdateProfileUseCase::new(Arc::new(mock_repo));

        let input = UpdateProfileInput::try_new(
            existing_id.clone(),
            Some(fake_first_name.clone()),
            Some(fake_last_name.clone()),
            Some(fake_bio.clone()),
            Some(fake_profile_image_url.clone()),
            1,
        )
        .unwrap();

        let result = use_case.execute(input).await;

        assert!(result.is_ok());

        let updated_profile = result.unwrap().unwrap();

        assert_eq!(
            updated_profile.first_name().map(|e| e.to_string()).unwrap(),
            fake_first_name
        );
        assert_eq!(
            updated_profile.last_name().map(|e| e.to_string()).unwrap(),
            fake_last_name
        );
        assert_eq!(
            updated_profile.bio().map(|e| e.to_string()).unwrap(),
            fake_bio
        );
        assert_eq!(
            updated_profile
                .profile_image_url()
                .map(|e| e.to_string())
                .unwrap(),
            fake_profile_image_url
        );
    }
}

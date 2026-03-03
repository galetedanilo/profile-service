use crate::{
    application::dtos::update_profile_input::UpdateProfileInput,
    domain::{
        models::profile::{Profile, ProfileError},
        object_values::{
            bio::Bio, first_name::FirstName, id::Id, image_url::ImageUrl, last_name::LastName,
        },
        repositories::profile_repo::ProfileRepository,
    },
};
use validify::Validify;

pub struct UpdateProfileUseCase<R: ProfileRepository + Send + Sync> {
    repository: R,
}

impl<R: ProfileRepository + Send + Sync> UpdateProfileUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: String,
        mut input: UpdateProfileInput,
    ) -> Result<Option<Profile>, ProfileError> {
        input
            .validify()
            .map_err(|e| ProfileError::InvalidData(e.to_string()))?;

        let id = Id::try_from(id).map_err(|e| ProfileError::InvalidData(e.to_string()))?;

        let profile = self
            .repository
            .get_profile_by_id(&id)
            .await
            .map_err(|e| ProfileError::RepositoryError(e.to_string()))?;

        if let Some(mut profile) = profile {
            if let Some(first_name) = input.first_name {
                let first_name = FirstName::try_from(first_name)
                    .map_err(|e| ProfileError::InvalidData(e.to_string()))?;
                profile.update_first_name(first_name);
            }
            if let Some(last_name) = input.last_name {
                let last_name = LastName::try_from(last_name)
                    .map_err(|e| ProfileError::InvalidData(e.to_string()))?;
                profile.update_last_name(last_name);
            }
            if let Some(bio) = input.bio {
                let bio =
                    Bio::try_from(bio).map_err(|e| ProfileError::InvalidData(e.to_string()))?;
                profile.update_bio(bio);
            }
            if let Some(profile_image_url) = input.profile_image_url {
                let profile_image_url = ImageUrl::try_from(profile_image_url)
                    .map_err(|e| ProfileError::InvalidData(e.to_string()))?;
                profile.update_profile_image_url(profile_image_url);
            }

            self.repository
                .save(&profile)
                .await
                .map_err(|e| ProfileError::RepositoryError(e.to_string()))?;

            Ok(Some(profile))
        } else {
            Err(ProfileError::NotFound(id.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::{
        Fake,
        faker::name::en::{FirstName, LastName},
    };
    use uuid::Uuid;

    use super::*;

    use crate::domain::{
        object_values::email::Email, repositories::profile_repo::MockProfileRepository,
    };

    #[tokio::test]
    pub async fn when_id_is_invalid_should_return_invalid_data_error() {
        let mock_repo = MockProfileRepository::new();

        let use_case = UpdateProfileUseCase::new(mock_repo);

        let invalid_id = String::from("invalid id");

        let input = UpdateProfileInput {
            first_name: Some(FirstName().fake::<String>()),
            last_name: Some(LastName().fake::<String>()),
            bio: None,
            profile_image_url: None,
        };

        let result = use_case.execute(invalid_id, input).await;

        assert!(matches!(result, Err(ProfileError::InvalidData(_))));
    }

    #[tokio::test]
    pub async fn when_first_name_is_invalid_should_return_invalid_data_error() {
        let mock_repo = MockProfileRepository::new();

        let use_case = UpdateProfileUseCase::new(mock_repo);

        let invalid_id = Uuid::now_v7().to_string();

        let input = UpdateProfileInput {
            first_name: Some("".to_string()), // Invalid first name
            last_name: Some(LastName().fake::<String>()),
            bio: None,
            profile_image_url: None,
        };

        let result = use_case.execute(invalid_id, input).await;

        assert!(matches!(result, Err(ProfileError::InvalidData(_))));
    }

    #[tokio::test]
    pub async fn when_last_name_is_invalid_should_return_invalid_data_error() {
        let mock_repo = MockProfileRepository::new();

        let use_case = UpdateProfileUseCase::new(mock_repo);

        let invalid_id = Uuid::now_v7().to_string();

        let input = UpdateProfileInput {
            first_name: None,
            last_name: Some("".to_string()), // Invalid last name
            bio: None,
            profile_image_url: None,
        };

        let result = use_case.execute(invalid_id, input).await;

        assert!(matches!(result, Err(ProfileError::InvalidData(_))));
    }

    #[tokio::test]
    pub async fn when_bio_is_invalid_should_return_invalid_data_error() {
        let mock_repo = MockProfileRepository::new();

        let use_case = UpdateProfileUseCase::new(mock_repo);

        let invalid_id = Uuid::now_v7().to_string();

        let input = UpdateProfileInput {
            first_name: None,
            last_name: None,
            bio: Some("".to_string()), // Invalid bio
            profile_image_url: None,
        };

        let result = use_case.execute(invalid_id, input).await;

        assert!(matches!(result, Err(ProfileError::InvalidData(_))));
    }

    #[tokio::test]
    pub async fn when_profile_image_url_is_invalid_should_return_invalid_data_error() {
        let mock_repo = MockProfileRepository::new();

        let use_case = UpdateProfileUseCase::new(mock_repo);

        let invalid_id = Uuid::now_v7().to_string();

        let input = UpdateProfileInput {
            first_name: None,
            last_name: None,
            bio: None,
            profile_image_url: Some("".to_string()), // Invalid profile image URL
        };

        let result = use_case.execute(invalid_id, input).await;

        assert!(matches!(result, Err(ProfileError::InvalidData(_))));
    }

    #[tokio::test]
    pub async fn when_profile_does_not_exist_should_return_not_found_error() {
        let mut mock_repo = MockProfileRepository::new();

        let non_existent_id = Uuid::now_v7().to_string();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(None));

        let use_case = UpdateProfileUseCase::new(mock_repo);

        let input = UpdateProfileInput {
            first_name: Some(FirstName().fake::<String>()),
            last_name: Some(LastName().fake::<String>()),
            bio: None,
            profile_image_url: None,
        };

        let result = use_case.execute(non_existent_id, input).await;

        assert!(matches!(result, Err(ProfileError::NotFound(_))));
    }

    #[tokio::test]
    pub async fn when_input_is_valid_and_profile_exists_should_update_profile() {
        let mut mock_repo = MockProfileRepository::new();

        let existing_id = Uuid::now_v7().to_string();

        let existing_profile = Profile::new_from(
            Id::try_from(existing_id.clone()).unwrap(),
            Email::try_from("john.doe@example.com".to_string()).unwrap(),
            Some(FirstName::try_from("John".to_string()).unwrap()),
            Some(LastName::try_from("Doe".to_string()).unwrap()),
            Some(Bio::try_from("Hello, I'm John!".to_string()).unwrap()),
            Some(ImageUrl::try_from("http://example.com/profile.jpg".to_string()).unwrap()),
        );

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(Some(existing_profile)));

        mock_repo.expect_save().times(1).return_const(Ok(()));

        let use_case = UpdateProfileUseCase::new(mock_repo);

        let input = UpdateProfileInput {
            first_name: Some("Jane".to_string()),
            last_name: Some("Smith".to_string()),
            bio: Some("Hi, I'm Jane!".to_string()),
            profile_image_url: Some("http://example.com/new_profile.jpg".to_string()),
        };

        let result = use_case.execute(existing_id, input).await;

        assert!(result.is_ok());

        let updated_profile = result.unwrap().unwrap();

        assert_eq!(
            updated_profile.first_name().map(|e| e.to_string()).unwrap(),
            "Jane"
        );
        assert_eq!(
            updated_profile.last_name().map(|e| e.to_string()).unwrap(),
            "Smith"
        );
        assert_eq!(
            updated_profile.bio().map(|e| e.to_string()).unwrap(),
            "Hi, I'm Jane!"
        );
        assert_eq!(
            updated_profile
                .profile_image_url()
                .map(|e| e.to_string())
                .unwrap(),
            "http://example.com/new_profile.jpg"
        );
    }
}

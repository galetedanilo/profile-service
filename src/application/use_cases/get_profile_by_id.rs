use crate::domain::{
    models::profile::{Profile, ProfileError},
    object_values::id::Id,
    repositories::profile_repo::ProfileRepository,
};

pub struct GetProfileById<R: ProfileRepository + Send + Sync> {
    repository: R,
}

impl<R: ProfileRepository + Send + Sync> GetProfileById<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: String) -> Result<Option<Profile>, ProfileError> {
        let id = Id::try_from(id)?;

        self.repository
            .get_profile_by_id(&id)
            .await
            .map_err(ProfileError::from)
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
    async fn when_profile_not_found_should_return_none() {
        let mut mock_repo = MockProfileRepository::new();

        let id = Uuid::now_v7().to_string();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(None));

        let use_case = GetProfileById::new(mock_repo);

        let result = use_case.execute(id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn when_profile_found_should_return_profile() {
        let mut mock_repo = MockProfileRepository::new();

        let id = Uuid::now_v7().to_string();

        let email = FreeEmail().fake::<String>();
        let profile = Profile::new(
            Id::try_from(id.clone()).unwrap(),
            Email::try_from(email).unwrap(),
        );

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Ok(Some(profile.clone())));

        let use_case = GetProfileById::new(mock_repo);

        let result = use_case.execute(id).await;

        assert!(result.is_ok());

        let profile_result = result.unwrap();

        assert!(profile_result.is_some());
        assert_eq!(profile_result.unwrap(), profile);
    }

    #[tokio::test]
    async fn when_repository_error_should_return_repository_error() {
        let mut mock_repo = MockProfileRepository::new();

        mock_repo
            .expect_get_profile_by_id()
            .times(1)
            .return_const(Err(ProfileRepositoryError::Unknown("mock error".into())));

        let use_case = GetProfileById::new(mock_repo);

        let result = use_case.execute(Uuid::now_v7().to_string()).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(ProfileError::Unknown(_))));
    }
}

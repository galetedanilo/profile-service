pub mod create_profile;
pub mod get_profile_by_id;
pub mod requests;
pub mod responses;
pub mod update_profile_by_id;

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use crate::domain::{
        models::profile::Profile,
        object_values::id::Id,
        repositories::profile_repo::{
            MockProfileRepository, ProfileRepository, ProfileRepositoryError,
        },
    };

    // Este é o "truque" para o Axum:
    // Um wrapper que é Clone, mas aponta para o mesmo Mock
    #[derive(Clone)]
    pub struct SharedMockRepository(pub Arc<MockProfileRepository>);

    #[async_trait::async_trait]
    impl ProfileRepository for SharedMockRepository {
        async fn get_profile_by_id(
            &self,
            id: &Id,
        ) -> Result<Option<Profile>, ProfileRepositoryError> {
            self.0.get_profile_by_id(id).await
        }

        async fn save(&self, profile: &Profile) -> Result<(), ProfileRepositoryError> {
            self.0.save(profile).await
        }
    }
}

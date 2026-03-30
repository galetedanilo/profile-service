use thiserror::Error;

#[cfg(test)]
use mockall::automock;

use crate::domain::{
    models::profile::{Profile, ProfileError},
    object_values::id::Id,
};

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait ProfileRepository: Send + Sync + 'static {
    async fn save(&self, profile: &Profile) -> Result<(), ProfileRepositoryError>;

    async fn get_profile_by_id(&self, id: &Id) -> Result<Option<Profile>, ProfileRepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum ProfileRepositoryError {
    #[error("Version conflict for profile with id: {0}")]
    VersionConflict(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<ProfileRepositoryError> for ProfileError {
    fn from(error: ProfileRepositoryError) -> Self {
        match error {
            ProfileRepositoryError::VersionConflict(id) => ProfileError::VersionConflict(id),
            ProfileRepositoryError::InvalidData(msg) => ProfileError::InvalidData(msg),
            ProfileRepositoryError::Unknown(msg) => ProfileError::Unknown(msg),
        }
    }
}

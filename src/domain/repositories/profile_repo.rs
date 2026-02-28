use thiserror::Error;

use crate::domain::{models::profile::Profile, object_values::id::Id};

#[async_trait::async_trait]
pub trait ProfileRepository {
    async fn save(&self, profile: &Profile) -> Result<(), ProfileRepositoryError>;

    async fn get_profile_by_id(&self, id: &Id) -> Result<Option<Profile>, ProfileRepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum ProfileRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Profile not found with id: {0}")]
    NotFound(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

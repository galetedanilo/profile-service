use crate::domain::{models::profile::ProfileError, object_values::id::Id};

#[derive(Debug, Clone)]
pub struct GetProfileByIdInput {
    pub id: Id,
}

impl GetProfileByIdInput {
    pub fn try_new(id: String) -> Result<Self, ProfileError> {
        let id = Id::try_from(id)?;

        Ok(Self { id })
    }
}

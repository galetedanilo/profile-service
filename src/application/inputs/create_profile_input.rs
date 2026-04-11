use crate::domain::{
    models::profile::ProfileError,
    object_values::{email::Email, id::Id},
};

#[derive(Debug, Clone)]
pub struct CreateProfileInput {
    pub id: Id,
    pub email: Email,
}

impl CreateProfileInput {
    pub fn try_new(id: String, email: String) -> Result<Self, ProfileError> {
        let id = Id::try_from(id)?;
        let email = Email::try_from(email)?;

        Ok(Self { id, email })
    }
}

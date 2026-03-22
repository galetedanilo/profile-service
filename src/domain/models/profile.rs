use thiserror::Error;

use crate::domain::object_values::{
    bio::Bio, email::Email, first_name::FirstName, id::Id, image_url::ImageUrl, last_name::LastName,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Profile {
    id: Id,
    email: Email,
    first_name: Option<FirstName>,
    last_name: Option<LastName>,
    bio: Option<Bio>,
    profile_image_url: Option<ImageUrl>,
    version: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum ProfileError {
    #[error("Profile with id {0} already exists")]
    AlreadyExists(String),

    #[error("Invalid profile data: {0}")]
    InvalidData(String),

    #[error("Profile not found with id: {0}")]
    NotFound(String),

    #[error("Version conflict for profile with id: {0}")]
    VersionConflict(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Profile {
    pub fn new(id: Id, email: Email) -> Self {
        let version = 0;
        Self {
            id,
            email,
            first_name: None,
            last_name: None,
            bio: None,
            profile_image_url: None,
            version,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn first_name(&self) -> Option<&FirstName> {
        self.first_name.as_ref()
    }

    pub fn last_name(&self) -> Option<&LastName> {
        self.last_name.as_ref()
    }

    pub fn bio(&self) -> Option<&Bio> {
        self.bio.as_ref()
    }

    pub fn profile_image_url(&self) -> Option<&ImageUrl> {
        self.profile_image_url.as_ref()
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn update_profile(
        &mut self,
        first_name: Option<FirstName>,
        last_name: Option<LastName>,
        bio: Option<Bio>,
        profile_image_url: Option<ImageUrl>,
    ) {
        if let Some(first_name) = first_name {
            self.first_name = Some(first_name);
        }
        if let Some(last_name) = last_name {
            self.last_name = Some(last_name);
        }
        if let Some(bio) = bio {
            self.bio = Some(bio);
        }
        if let Some(profile_image_url) = profile_image_url {
            self.profile_image_url = Some(profile_image_url);
        }
        self.version += 1;
    }

    pub fn new_from(
        id: Id,
        email: Email,
        first_name: Option<FirstName>,
        last_name: Option<LastName>,
        bio: Option<Bio>,
        profile_image_url: Option<ImageUrl>,
        version: u64,
    ) -> Self {
        Self {
            id,
            email,
            first_name,
            last_name,
            bio,
            profile_image_url,
            version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_create_profile_should_have_version_zero() {
        let id = Id::try_from("123e4567-e89b-12d3-a456-426614174000".to_string()).unwrap();
        let email = Email::try_from("test@example.com".to_string()).unwrap();
        let profile = Profile::new(id, email);
        assert_eq!(profile.version(), 0);
    }

    #[test]
    fn when_create_profile_should_have_correct_id() {
        let id = Id::try_from("123e4567-e89b-12d3-a456-426614174000".to_string()).unwrap();
        let email = Email::try_from("test@example.com".to_string()).unwrap();
        let profile = Profile::new(id, email);
        assert_eq!(
            profile.id(),
            &Id::try_from("123e4567-e89b-12d3-a456-426614174000".to_string()).unwrap()
        );
    }

    #[test]
    fn when_create_profile_should_have_correct_email() {
        let id = Id::try_from("123e4567-e89b-12d3-a456-426614174000".to_string()).unwrap();
        let email = Email::try_from("test@example.com".to_string()).unwrap();
        let profile = Profile::new(id, email);
        assert_eq!(
            profile.email(),
            &Email::try_from("test@example.com".to_string()).unwrap()
        );
    }

    #[test]
    fn when_update_profile_should_update_version() {
        let id = Id::try_from("123e4567-e89b-12d3-a456-426614174000".to_string()).unwrap();
        let email = Email::try_from("test@example.com".to_string()).unwrap();
        let mut profile = Profile::new(id, email);
        let initial_version = profile.version();
        profile.update_profile(
            Some(FirstName::try_from("John".to_string()).unwrap()),
            None,
            None,
            None,
        );
        assert_eq!(profile.version(), initial_version + 1);
    }
}

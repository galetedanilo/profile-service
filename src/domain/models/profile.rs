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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum ProfileError {
    #[error("Profile with id {0} already exists")]
    AlreadyExists(String),

    #[error("Invalid profile data: {0}")]
    InvalidData(String),

    #[error("Profile not found with id: {0}")]
    NotFound(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Profile {
    pub fn new(id: Id, email: Email) -> Self {
        Self {
            id,
            email,
            first_name: None,
            last_name: None,
            bio: None,
            profile_image_url: None,
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

    pub fn update_first_name(&mut self, first_name: FirstName) {
        self.first_name = Some(first_name);
    }

    pub fn update_last_name(&mut self, last_name: LastName) {
        self.last_name = Some(last_name);
    }

    pub fn update_bio(&mut self, bio: Bio) {
        self.bio = Some(bio);
    }

    pub fn update_profile_image_url(&mut self, profile_image_url: ImageUrl) {
        self.profile_image_url = Some(profile_image_url);
    }

    pub fn new_from(
        id: Id,
        email: Email,
        first_name: Option<FirstName>,
        last_name: Option<LastName>,
        bio: Option<Bio>,
        profile_image_url: Option<ImageUrl>,
    ) -> Self {
        Self {
            id,
            email,
            first_name,
            last_name,
            bio,
            profile_image_url,
        }
    }
}

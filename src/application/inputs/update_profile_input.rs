use crate::domain::{
    models::profile::ProfileError,
    object_values::{
        bio::Bio, first_name::FirstName, id::Id, image_url::ImageUrl, last_name::LastName,
    },
};

#[derive(Debug, Clone)]
pub struct UpdateProfileInput {
    pub id: Id,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub bio: Option<Bio>,
    pub profile_image_url: Option<ImageUrl>,
}

impl UpdateProfileInput {
    pub fn try_new(
        id: String,
        first_name: Option<String>,
        last_name: Option<String>,
        bio: Option<String>,
        profile_image_url: Option<String>,
    ) -> Result<Self, ProfileError> {
        let id = Id::try_from(id)?;

        let first_name = first_name.map(FirstName::try_from).transpose()?;
        let last_name = last_name.map(LastName::try_from).transpose()?;
        let bio = bio.map(Bio::try_from).transpose()?;
        let profile_image_url = profile_image_url.map(ImageUrl::try_from).transpose()?;

        Ok(Self {
            id,
            first_name,
            last_name,
            bio,
            profile_image_url,
        })
    }
}

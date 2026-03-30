use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        models::profile::Profile,
        object_values::{
            bio::Bio, email::Email, first_name::FirstName, id::Id, image_url::ImageUrl,
            last_name::LastName,
        },
        repositories::profile_repo::{ProfileRepository, ProfileRepositoryError},
    },
    infrastructure::repositories::mongo_service::MongoService,
};
use mongodb::{Collection, bson::doc};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileDocument {
    #[serde(rename = "_id")]
    id: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<String>,
    profile_image_url: Option<String>,
    version: u64,
}

#[derive(Clone)]
pub struct MongoProfileRepository {
    collection: Collection<ProfileDocument>,
}

impl MongoProfileRepository {
    pub fn new(mongo_service: Arc<MongoService>) -> Self {
        let collection = mongo_service
            .get_database()
            .collection::<ProfileDocument>("profiles");

        MongoProfileRepository { collection }
    }
}

#[async_trait::async_trait]
impl ProfileRepository for MongoProfileRepository {
    async fn save(&self, profile: &Profile) -> Result<(), ProfileRepositoryError> {
        let doc: ProfileDocument = profile.clone().into();

        if doc.version == 0 {
            return match self
                .collection
                .insert_one(doc)
                .await
                .map_err(|e| ProfileRepositoryError::Unknown(e.to_string()))
            {
                Ok(_) => Ok(()),
                Err(e) => Err(ProfileRepositoryError::Unknown(e.to_string())),
            };
        }

        let filter = doc! {"_id": doc.id.clone(), "version": doc.version as i64 - 1};

        let result = self
            .collection
            .replace_one(filter, doc)
            .await
            .map_err(|e| ProfileRepositoryError::Unknown(e.to_string()))?;

        if result.matched_count == 0 {
            return Err(ProfileRepositoryError::VersionConflict(
                profile.id().to_string(),
            ));
        }

        Ok(())
    }

    async fn get_profile_by_id(&self, id: &Id) -> Result<Option<Profile>, ProfileRepositoryError> {
        let document = self
            .collection
            .find_one(doc! {"_id": id.to_string()})
            .await
            .map_err(|e| ProfileRepositoryError::Unknown(e.to_string()))?;

        Ok(document.map(|doc| Profile::try_from(doc)).transpose()?)
    }
}

impl From<Profile> for ProfileDocument {
    fn from(profile: Profile) -> Self {
        ProfileDocument {
            id: profile.id().to_string(),
            email: profile.email().to_string(),
            first_name: profile.first_name().map(|f| f.to_string()),
            last_name: profile.last_name().map(|l| l.to_string()),
            bio: profile.bio().map(|b| b.to_string()),
            profile_image_url: profile.profile_image_url().map(|u| u.to_string()),
            version: profile.version(),
        }
    }
}

impl TryFrom<ProfileDocument> for Profile {
    type Error = ProfileRepositoryError;

    fn try_from(doc: ProfileDocument) -> Result<Self, Self::Error> {
        let id: Id =
            Id::try_from(doc.id).map_err(|e| ProfileRepositoryError::InvalidData(e.to_string()))?;
        let email = Email::try_from(doc.email)
            .map_err(|e| ProfileRepositoryError::InvalidData(e.to_string()))?;
        let first_name = doc
            .first_name
            .map(|f| FirstName::try_from(f))
            .transpose()
            .map_err(|e| ProfileRepositoryError::InvalidData(e.to_string()))?;
        let last_name = doc
            .last_name
            .map(|f| LastName::try_from(f))
            .transpose()
            .map_err(|e| ProfileRepositoryError::InvalidData(e.to_string()))?;
        let bio = doc
            .bio
            .map(|b| Bio::try_from(b))
            .transpose()
            .map_err(|e| ProfileRepositoryError::InvalidData(e.to_string()))?;
        let profile_image_url = doc
            .profile_image_url
            .map(|f| ImageUrl::try_from(f))
            .transpose()
            .map_err(|e| ProfileRepositoryError::InvalidData(e.to_string()))?;

        Ok(Profile::new_from(
            id,
            email,
            first_name,
            last_name,
            bio,
            profile_image_url,
            doc.version,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_create_profile_document_should_have_correct_fields() {
        let id = Id::try_from("123e4567-e89b-12d3-a456-426614174000".to_string()).unwrap();
        let email = Email::try_from("test@example.com".to_string()).unwrap();
        let profile = Profile::new(id, email);
        let doc: ProfileDocument = profile.into();
        assert_eq!(doc.id, "123e4567-e89b-12d3-a456-426614174000");
        assert_eq!(doc.email, "test@example.com");
        assert_eq!(doc.version, 0);
    }

    #[test]
    fn when_convert_profile_document_to_profile_should_have_correct_fields() {
        let doc = ProfileDocument {
            id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
            email: "test@example.com".to_string(),
            first_name: None,
            last_name: None,
            bio: None,
            profile_image_url: None,
            version: 0,
        };
        let profile = Profile::try_from(doc).unwrap();
        assert_eq!(
            profile.id(),
            &Id::try_from("123e4567-e89b-12d3-a456-426614174000".to_string()).unwrap()
        );
        assert_eq!(
            profile.email(),
            &Email::try_from("test@example.com".to_string()).unwrap()
        );
        assert_eq!(profile.version(), 0);
        assert_eq!(profile.first_name(), None);
        assert_eq!(profile.last_name(), None);
        assert_eq!(profile.bio(), None);
        assert_eq!(profile.profile_image_url(), None);
    }

    #[test]
    fn when_convert_profile_document_to_profile_should_handle_missing_optional_fields() {
        let doc = ProfileDocument {
            id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
            email: "test@example.com".to_string(),
            first_name: None,
            last_name: None,
            bio: None,
            profile_image_url: None,
            version: 0,
        };
        let profile = Profile::try_from(doc).unwrap();
        assert_eq!(profile.first_name(), None);
        assert_eq!(profile.last_name(), None);
        assert_eq!(profile.bio(), None);
        assert_eq!(profile.profile_image_url(), None);
    }
}

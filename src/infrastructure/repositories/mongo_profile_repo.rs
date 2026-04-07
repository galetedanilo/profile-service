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
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
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

        if doc.version == 1 {
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

        Ok(document.map(Profile::try_from).transpose()?)
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
            created_at: profile.created_at(),
            updated_at: profile.updated_at(),
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
            .map(FirstName::try_from)
            .transpose()
            .map_err(|e| ProfileRepositoryError::InvalidData(e.to_string()))?;
        let last_name = doc
            .last_name
            .map(LastName::try_from)
            .transpose()
            .map_err(|e| ProfileRepositoryError::InvalidData(e.to_string()))?;
        let bio = doc
            .bio
            .map(Bio::try_from)
            .transpose()
            .map_err(|e| ProfileRepositoryError::InvalidData(e.to_string()))?;
        let profile_image_url = doc
            .profile_image_url
            .map(ImageUrl::try_from)
            .transpose()
            .map_err(|e| ProfileRepositoryError::InvalidData(e.to_string()))?;

        Ok(Profile::from_parts(
            id,
            email,
            first_name,
            last_name,
            bio,
            profile_image_url,
            doc.created_at,
            doc.updated_at,
            doc.version,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_profile_to_document() {
        let profile = Profile::from_parts(
            Id::generate(),
            Email::try_from("test@example.com").unwrap(),
            None,
            None,
            None,
            None,
            chrono::Utc::now(),
            None,
            0,
        );

        let doc: ProfileDocument = profile.clone().into();

        assert_eq!(doc.id, profile.id().to_string());
        assert_eq!(doc.email, profile.email().to_string());
        assert_eq!(doc.version, profile.version());
    }
}

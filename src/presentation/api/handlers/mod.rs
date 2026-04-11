pub mod create_profile;
pub mod get_profile_by_id;
pub mod update_profile_by_id;

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use jsonwebtoken::DecodingKey;
    use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
    use lazy_static::lazy_static;
    use ring::signature::Ed25519KeyPair;
    use ring::signature::KeyPair;
    use serde_json::json;
    use uuid::Uuid;

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

    lazy_static! {
        // Gera o par de chaves DER/PKCS#8 uma única vez para toda a suíte de testes
        static ref TEST_KEYS: (Vec<u8>, Vec<u8>) = {
            let rng = ring::rand::SystemRandom::new();
            let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
            let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
            let public_key = key_pair.public_key().as_ref().to_vec();
            (pkcs8.as_ref().to_vec(), public_key)
        };
    }

    pub fn get_test_decoding_key() -> DecodingKey {
        DecodingKey::from_ed_der(&TEST_KEYS.1)
    }

    pub fn create_test_token() -> String {
        let profile_id = Uuid::now_v7().to_string();

        let claims = json!({
            "sub": profile_id,
            "iat": 1700000000u64,
            "exp": 9999999999u64,
            "aud": ["profile-service"],
            "scopes": ["profile:admin", "profile:create", "profile:read", "profile:update", "profile:delete"],
            "email": "test@example.com"
        });

        let encoding_key = EncodingKey::from_ed_der(&TEST_KEYS.0);
        encode(&Header::new(Algorithm::EdDSA), &claims, &encoding_key).unwrap()
    }
}

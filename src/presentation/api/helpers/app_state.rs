use std::sync::Arc;

use jsonwebtoken::DecodingKey;

use crate::{
    application::use_cases::{
        create_profile_use_case::CreateProfileUseCase,
        get_profile_by_id_use_case::GetProfileByIdUseCase,
        update_profile_use_case::UpdateProfileUseCase,
    },
    domain::repositories::profile_repo::ProfileRepository,
};

#[derive(Clone)]
pub struct AppState<R: ProfileRepository> {
    pub create_profile_use_case: Arc<CreateProfileUseCase<R>>,
    pub get_profile_by_id_use_case: Arc<GetProfileByIdUseCase<R>>,
    pub update_profile_use_case: Arc<UpdateProfileUseCase<R>>,
    pub decoding_key: Arc<DecodingKey>,
}

impl<R: ProfileRepository> AppState<R> {
    pub fn new(repository: Arc<R>, decoding_key: Arc<DecodingKey>) -> Self {
        Self {
            create_profile_use_case: Arc::new(CreateProfileUseCase::new(Arc::clone(&repository))),
            get_profile_by_id_use_case: Arc::new(GetProfileByIdUseCase::new(Arc::clone(
                &repository,
            ))),
            update_profile_use_case: Arc::new(UpdateProfileUseCase::new(Arc::clone(&repository))),
            decoding_key,
        }
    }
}

use std::sync::Arc;

use crate::application::use_cases::create_profile::CreateProfileUseCase;

pub struct AppState {
    pub create_profile_use_case: Arc<CreateProfileUseCase>,
}

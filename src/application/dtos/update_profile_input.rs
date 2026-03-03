use serde::Deserialize;
use validify::Validify;

#[derive(Debug, Clone, Deserialize, Validify)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileInput {
    #[modify(trim)]
    #[validate(length(min = 2, max = 15))]
    pub first_name: Option<String>,

    #[modify(trim)]
    #[validate(length(min = 2, max = 25))]
    pub last_name: Option<String>,

    #[modify(trim)]
    #[validate(length(min = 10, max = 160))]
    pub bio: Option<String>,

    #[modify(trim)]
    #[validate(length(min = 5, max = 2048))]
    pub profile_image_url: Option<String>,
}

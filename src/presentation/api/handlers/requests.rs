use serde::Deserialize;
use validify::Validify;

#[derive(Debug, Clone, Deserialize, Validify)]
pub struct CreateProfileRequest {
    #[modify(lowercase, trim)]
    #[validate(length(
        min = 3,
        max = 100,
        message = "ID must be between 3 and 100 characters"
    ))]
    pub id: String,

    #[modify(lowercase, trim)]
    #[validate(
        email(message = "Email must be a valid email address"),
        length(
            min = 3,
            max = 255,
            message = "Email must be a valid email address between 3 and 255 characters"
        )
    )]
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Validify)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    #[modify(trim)]
    #[validate(length(
        min = 2,
        max = 15,
        message = "First name must be between 2 and 15 characters"
    ))]
    pub first_name: Option<String>,

    #[modify(trim)]
    #[validate(length(
        min = 2,
        max = 25,
        message = "Last name must be between 2 and 25 characters"
    ))]
    pub last_name: Option<String>,

    #[modify(trim)]
    #[validate(length(
        min = 10,
        max = 160,
        message = "Bio must be between 10 and 160 characters"
    ))]
    pub bio: Option<String>,

    #[modify(trim)]
    #[validate(length(
        min = 5,
        max = 2048,
        message = "Profile image URL must be between 5 and 2048 characters"
    ))]
    pub profile_image_url: Option<String>,
}

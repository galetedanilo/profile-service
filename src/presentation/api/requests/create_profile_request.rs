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
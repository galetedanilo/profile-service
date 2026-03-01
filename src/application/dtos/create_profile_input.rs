use serde::Deserialize;
use validify::Validify;

#[derive(Debug, Clone, Deserialize, Validify)]
pub struct CreateProfileInput {
    #[modify(lowercase, trim)]
    #[validate(length(min = 3, max = 100))]
    pub id: String,

    #[modify(lowercase, trim)]
    #[validate(email, length(min = 3, max = 255))]
    pub email: String,
}

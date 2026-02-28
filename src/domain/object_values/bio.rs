use std::fmt::Display;

use thiserror::Error;

use crate::domain::helpers::BIO_VALID_CHARS_REGEX;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bio(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum BioError {
    #[error("Bio cannot be empty")]
    Empty,

    #[error("Bio is too long (maximum {0} characters)")]
    TooLong(usize),

    #[error("Bio is too short (minimum {0} characters)")]
    TooShort(usize),

    #[error(
        "Bio contains invalid characters (only letters, numbers, underscores, and dots are allowed)"
    )]
    InvalidCharacters,
}

impl Bio {
    const MIN_LENGTH: usize = 10;
    const MAX_LENGTH: usize = 160;

    pub fn try_new(value: String) -> Result<Self, BioError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(BioError::Empty);
        }

        if trimmed.len() < Self::MIN_LENGTH {
            return Err(BioError::TooShort(Self::MIN_LENGTH));
        }

        if trimmed.len() > Self::MAX_LENGTH {
            return Err(BioError::TooLong(Self::MAX_LENGTH));
        }

        if !BIO_VALID_CHARS_REGEX.is_match(trimmed) {
            return Err(BioError::InvalidCharacters);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Bio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Bio {
    type Error = BioError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value.to_string())
    }
}

impl TryFrom<String> for Bio {
    type Error = BioError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_bio_too_short_should_min_length_error() {
        let bio = Bio::try_new("Too short".to_string());
        assert!(matches!(bio, Err(BioError::TooShort(_))));
    }

    #[test]
    fn when_bio_too_long_should_max_length_error() {
        let long_bio = "a".repeat(Bio::MAX_LENGTH + 1);
        let bio = Bio::try_new(long_bio);
        assert!(matches!(bio, Err(BioError::TooLong(_))));
    }

    #[test]
    fn when_bio_empty_should_empty_error() {
        let bio = Bio::try_new("".to_string());
        assert!(matches!(bio, Err(BioError::Empty)));
    }

    #[test]
    fn when_bio_just_white_space_should_empty_error() {
        let bio = Bio::try_new("  ".to_string());
        assert!(matches!(bio, Err(BioError::Empty)));
    }

    #[test]
    fn when_bio_contains_invalid_characters_should_invalid_characters_error() {
        let bio = Bio::try_new("Invalid bio with #".to_string());
        assert!(matches!(bio, Err(BioError::InvalidCharacters)));
    }

    #[test]
    fn when_bio_valid_should_create_bio() {
        let bio_str = "This is a valid bio.";
        let bio = Bio::try_new(bio_str.to_string()).unwrap();
        println!("Bio: {}, {}", bio.to_string(), bio_str);
        assert_eq!(bio.to_string(), bio_str);
    }

    #[test]
    fn when_bio_valid_should_create_bio_from_str() {
        let bio_str = "This is a valid bio.";
        let bio = Bio::try_from(bio_str).unwrap();
        assert_eq!(bio.to_string(), bio_str);
    }

    #[test]
    fn when_bio_valid_should_create_bio_from_string() {
        let bio_str = "This is a valid bio.";
        let bio = Bio::try_from(bio_str.to_string()).unwrap();
        assert_eq!(bio.to_string(), bio_str);
    }
}

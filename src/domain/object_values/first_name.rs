use std::fmt::Display;

use thiserror::Error;

use crate::domain::helpers::VALID_CHARS_REGEX;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FirstName(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum FirstNameError {
    #[error("First name cannot be emtpy")]
    Empty,

    #[error("First name is too long(maximum {0} characters)")]
    ToLong(usize),

    #[error("Frist name is too short(minimum {0} characters)")]
    ToShort(usize),

    #[error(
        "First name contains invalid characters (only letters, numbers, underscores, and dots are allowed"
    )]
    InvalidCharacters,

    #[error("First name cannot start or end with a special character")]
    InvalidEdgeCharacters,
}

impl FirstName {
    const MIN_LENGTH: usize = 2;
    const MAX_LENGTH: usize = 15;

    pub fn try_new(value: String) -> Result<Self, FirstNameError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(FirstNameError::Empty);
        }

        if trimmed.len() < Self::MIN_LENGTH {
            return Err(FirstNameError::ToShort(Self::MIN_LENGTH));
        }

        if trimmed.len() > Self::MAX_LENGTH {
            return Err(FirstNameError::ToLong(Self::MAX_LENGTH));
        }

        if trimmed.starts_with(|c: char| !c.is_alphanumeric())
            || trimmed.ends_with(|c: char| !c.is_alphanumeric())
        {
            return Err(FirstNameError::InvalidEdgeCharacters);
        }

        if !VALID_CHARS_REGEX.is_match(trimmed) {
            return Err(FirstNameError::InvalidCharacters);
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

impl Display for FirstName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for FirstName {
    type Error = FirstNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value.to_string())
    }
}

impl TryFrom<String> for FirstName {
    type Error = FirstNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_first_name_empty_should_empty_error() {
        let first_name = FirstName::try_new("".to_string());
        assert!(matches!(first_name, Err(FirstNameError::Empty)));
    }

    #[test]
    fn when_first_name_just_white_space_should_empty_error() {
        let first_name = FirstName::try_new("  ".to_string());
        assert!(matches!(first_name, Err(FirstNameError::Empty)));
    }

    #[test]
    fn when_first_name_too_short_should_min_length_error() {
        let first_name = FirstName::try_new("a".to_string());
        assert!(matches!(first_name, Err(FirstNameError::ToShort(_))));
    }

    #[test]
    fn when_first_name_too_long_should_max_length_error() {
        let first_name = FirstName::try_new(
            "Hello, I am a long first name for this history, because I am so so".to_string(),
        );
        assert!(matches!(first_name, Err(FirstNameError::ToLong(_))));
    }

    #[test]
    fn when_first_name_start_with_invalid_characters_should_invalid_edge_characters_error() {
        let first_name = FirstName::try_new("& name".to_string());
        assert!(matches!(
            first_name,
            Err(FirstNameError::InvalidEdgeCharacters)
        ));
    }

    #[test]
    fn when_first_name_end_with_invalid_characters_should_invalid_edge_characters_error() {
        let firs_name = FirstName::try_new("Da &".to_string());
        assert!(matches!(
            firs_name,
            Err(FirstNameError::InvalidEdgeCharacters)
        ));
    }

    #[test]
    fn when_first_name_have_invalid_characters_should_invalid_characters_error() {
        let first_name = FirstName::try_new("Dani&lo".to_string());
        assert!(matches!(first_name, Err(FirstNameError::InvalidCharacters)));
    }
}

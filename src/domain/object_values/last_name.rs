use std::fmt::Display;

use thiserror::Error;

use crate::domain::helpers::VALID_CHARS_REGEX;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LastName(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum LastNameError {
    #[error("Last name cannot be emtpy")]
    Empty,

    #[error("Last name is too long(maximum {0} characters)")]
    ToLong(usize),

    #[error("Last name is too short(minimum {0} characters)")]
    ToShort(usize),

    #[error(
        "Last name contains invalid characters (only letters, numbers, underscores, and dots are allowed"
    )]
    InvalidCharacters,

    #[error("Last name cannot start or end with a special character")]
    InvalidEdgeCharacters,
}

impl LastName {
    const MIN_LENGTH: usize = 2;
    const MAX_LENGTH: usize = 25;

    pub fn try_new(value: String) -> Result<Self, LastNameError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(LastNameError::Empty);
        }

        if trimmed.len() < Self::MIN_LENGTH {
            return Err(LastNameError::ToShort(Self::MIN_LENGTH));
        }

        if trimmed.len() > Self::MAX_LENGTH {
            return Err(LastNameError::ToLong(Self::MAX_LENGTH));
        }

        if trimmed.starts_with(|c: char| !c.is_alphanumeric())
            || trimmed.ends_with(|c: char| !c.is_alphanumeric())
        {
            return Err(LastNameError::InvalidEdgeCharacters);
        }

        if !VALID_CHARS_REGEX.is_match(trimmed) {
            return Err(LastNameError::InvalidCharacters);
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

impl Display for LastName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for LastName {
    type Error = LastNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value.to_string())
    }
}

impl TryFrom<String> for LastName {
    type Error = LastNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_last_name_empty_should_empty_error() {
        let last_name = LastName::try_new("".to_string());
        assert!(matches!(last_name, Err(LastNameError::Empty)));
    }

    #[test]
    fn when_last_name_just_white_space_should_empty_error() {
        let last_name = LastName::try_new("  ".to_string());
        assert!(matches!(last_name, Err(LastNameError::Empty)));
    }

    #[test]
    fn when_last_name_to_short_should_min_length_error() {
        let last_name = LastName::try_new("a".to_string());
        assert!(matches!(last_name, Err(LastNameError::ToShort(_))));
    }

    #[test]
    fn when_last_name_to_long_should_max_length_error() {
        let last_name =
            LastName::try_new("jfi fuodf oufdif oufsli fidojfo fios ufdsoufo".to_string());
        assert!(matches!(last_name, Err(LastNameError::ToLong(_))));
    }

    #[test]
    fn when_last_name_start_with_invalid_characters_should_invalid_edge_characters_error() {
        let last_name = LastName::try_new("& name".to_string());
        assert!(matches!(
            last_name,
            Err(LastNameError::InvalidEdgeCharacters)
        ));
    }

    #[test]
    fn when_last_name_end_with_invalid_characters_should_invalid_edge_characters_error() {
        let last_name = LastName::try_new("Da &".to_string());
        assert!(matches!(
            last_name,
            Err(LastNameError::InvalidEdgeCharacters)
        ));
    }

    #[test]
    fn when_last_name_have_invalid_characters_should_invalid_characters_error() {
        let last_name = LastName::try_new("Test &dog".to_string());
        assert!(matches!(last_name, Err(LastNameError::InvalidCharacters)));
    }

    #[test]
    fn when_last_name_valid_should_create_last_name() {
        let last_name_str = "Smith";
        let last_name = LastName::try_new(last_name_str.to_string()).unwrap();
        assert_eq!(last_name.to_string(), last_name_str);
    }

    #[test]
    fn when_last_name_valid_should_create_last_name_from_str() {
        let last_name_str = "Smith";
        let last_name = LastName::try_from(last_name_str).unwrap();
        assert_eq!(last_name.to_string(), last_name_str);
    }

    #[test]
    fn when_last_name_valid_should_create_last_name_from_string() {
        let last_name_str = "Smith";
        let last_name = LastName::try_from(last_name_str.to_string()).unwrap();
        assert_eq!(last_name.to_string(), last_name_str);
    }
}

use std::fmt::Display;

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(Uuid);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum IdError {
    #[error("invalid id: {0}")]
    Invalid(String),
}

impl Id {
    pub fn from_str(id_str: &str) -> Result<Self, IdError> {
        Uuid::parse_str(id_str)
            .map_err(|e| IdError::Invalid(e.to_string()))
            .map(Id)
    }

    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn into_inner(&self) -> Uuid {
        self.0
    }

    pub fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Id {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::parse_str(&value)
            .map_err(|e| IdError::Invalid(e.to_string()))
            .map(Id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_call_generate_method_shoul_return_valid_uuid() {
        let id1 = Id::generate();
        let id2 = Id::generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn when_str_as_valid_uuid_should_return_id_object_value() {
        let id_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = Id::from_str(id_str).unwrap();
        assert_eq!(id.to_string(), id_str);
    }

    #[test]
    fn when_str_is_not_valid_uuid_should_invalid_error() {
        let id_str = "invalid-uuid";
        let result = Id::from_str(id_str);
        assert!(matches!(
            result,
            Err(IdError::Invalid(_))
        ))
    }
}

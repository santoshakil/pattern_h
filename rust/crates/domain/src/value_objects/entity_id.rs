use errors::DomainError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct EntityId(String);

impl EntityId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn from_value(s: impl Into<String>) -> Result<Self, DomainError> {
        let val = s.into();
        if val.is_empty() {
            return Err(DomainError::Validation("entity id cannot be empty".into()));
        }
        Ok(Self(val))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_unique_ids() {
        let a = EntityId::new();
        let b = EntityId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn from_value_valid() {
        let id = EntityId::from_value("abc-123");
        assert!(id.is_ok());
        let id = match id {
            Ok(v) => v,
            Err(_) => panic!("expected ok"),
        };
        assert_eq!(id.as_str(), "abc-123");
    }

    #[test]
    fn from_value_empty_returns_validation_err() {
        let id = EntityId::from_value("");
        assert!(id.is_err());
        match id {
            Err(DomainError::Validation(msg)) => {
                assert!(msg.contains("empty"));
            }
            _ => panic!("expected Validation error"),
        }
    }

    #[test]
    fn as_str_returns_inner() {
        let id = EntityId::from_value("test-id");
        assert!(id.is_ok());
        let id = match id {
            Ok(v) => v,
            Err(_) => panic!("expected ok"),
        };
        assert_eq!(id.as_str(), "test-id");
    }

    #[test]
    fn default_generates_id() {
        let id = EntityId::default();
        assert!(!id.as_str().is_empty());
    }

    #[test]
    fn display_shows_inner_string() {
        let id = EntityId::from_value("display-me");
        assert!(id.is_ok());
        let id = match id {
            Ok(v) => v,
            Err(_) => panic!("expected ok"),
        };
        assert_eq!(format!("{id}"), "display-me");
    }

    #[test]
    fn from_value_accepts_string_type() {
        let owned = String::from("owned-val");
        let id = EntityId::from_value(owned);
        assert!(id.is_ok());
    }

    #[test]
    fn clone_produces_equal() {
        let a = EntityId::from_value("clone-me");
        assert!(a.is_ok());
        let a = match a {
            Ok(v) => v,
            Err(_) => panic!("expected ok"),
        };
        let b = a.clone();
        assert_eq!(a, b);
    }
}

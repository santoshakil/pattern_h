use std::fmt;

#[derive(Debug, Clone)]
pub enum DomainEvent {
    EntityCreated { entity_type: String, id: String },
    EntityUpdated { entity_type: String, id: String },
    EntityDeleted { entity_type: String, id: String },
    Custom { name: String, payload: Vec<u8> },
}

impl fmt::Display for DomainEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntityCreated { entity_type, id } => {
                write!(f, "created {entity_type}({id})")
            }
            Self::EntityUpdated { entity_type, id } => {
                write!(f, "updated {entity_type}({id})")
            }
            Self::EntityDeleted { entity_type, id } => {
                write!(f, "deleted {entity_type}({id})")
            }
            Self::Custom { name, .. } => write!(f, "custom({name})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_created_display() {
        let e = DomainEvent::EntityCreated {
            entity_type: "Order".into(),
            id: "7".into(),
        };
        assert_eq!(e.to_string(), "created Order(7)");
    }

    #[test]
    fn entity_updated_display() {
        let e = DomainEvent::EntityUpdated {
            entity_type: "Item".into(),
            id: "12".into(),
        };
        assert_eq!(e.to_string(), "updated Item(12)");
    }

    #[test]
    fn entity_deleted_display() {
        let e = DomainEvent::EntityDeleted {
            entity_type: "User".into(),
            id: "3".into(),
        };
        assert_eq!(e.to_string(), "deleted User(3)");
    }

    #[test]
    fn custom_display() {
        let e = DomainEvent::Custom {
            name: "sync_done".into(),
            payload: vec![1, 2, 3],
        };
        assert_eq!(e.to_string(), "custom(sync_done)");
    }

    #[test]
    fn clone_preserves_values() {
        let e = DomainEvent::EntityCreated {
            entity_type: "X".into(),
            id: "1".into(),
        };
        let c = e.clone();
        assert_eq!(e.to_string(), c.to_string());
    }

    #[test]
    fn clone_custom_preserves_payload() {
        let payload = vec![10, 20, 30];
        let e = DomainEvent::Custom {
            name: "test".into(),
            payload: payload.clone(),
        };
        let c = e.clone();
        if let DomainEvent::Custom { name, payload: p } = c {
            assert_eq!(name, "test");
            assert_eq!(p, payload);
        } else {
            panic!("expected Custom variant");
        }
    }
}

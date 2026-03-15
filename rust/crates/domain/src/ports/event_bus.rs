use crate::events::DomainEvent;
use errors::DomainError;

pub trait EventBus: Send + Sync {
    fn publish(&self, event: DomainEvent) -> Result<(), DomainError>;
    fn publish_all(&self, events: Vec<DomainEvent>) -> Result<(), DomainError> {
        for event in events {
            self.publish(event)?;
        }
        Ok(())
    }
}

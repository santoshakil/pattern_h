use errors::DomainError;
use std::any::Any;

pub trait EventBus: Send + Sync {
    fn publish(&self, event: Box<dyn Any + Send>) -> Result<(), DomainError>;
}

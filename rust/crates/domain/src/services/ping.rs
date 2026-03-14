use errors::DomainError;

pub trait PingService: Send + Sync {
    fn process(&self, message: &str) -> Result<String, DomainError>;
}

pub struct DefaultPingService;

impl PingService for DefaultPingService {
    fn process(&self, message: &str) -> Result<String, DomainError> {
        if message.is_empty() {
            return Err(DomainError::Validation("ping message cannot be empty".into()));
        }
        Ok(format!("pong: {}", message))
    }
}

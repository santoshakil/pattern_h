use errors::DomainError;

pub trait PingService: Send + Sync {
    fn process(&self, message: &str) -> Result<String, DomainError>;
}

pub struct DefaultPingService;

impl PingService for DefaultPingService {
    fn process(&self, message: &str) -> Result<String, DomainError> {
        if message.is_empty() {
            return Err(DomainError::Validation(
                "ping message cannot be empty".into(),
            ));
        }
        Ok(format!("pong: {}", message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_valid_msg() -> Result<(), DomainError> {
        let svc = DefaultPingService;
        let res = svc.process("hello")?;
        assert_eq!(res, "pong: hello");
        Ok(())
    }

    #[test]
    fn process_with_special_chars() -> Result<(), DomainError> {
        let svc = DefaultPingService;
        let res = svc.process("hi there! 123")?;
        assert_eq!(res, "pong: hi there! 123");
        Ok(())
    }

    #[test]
    fn process_empty_returns_validation_err() {
        let svc = DefaultPingService;
        let res = svc.process("");
        assert!(res.is_err());
        match res {
            Err(DomainError::Validation(msg)) => {
                assert!(msg.contains("empty"));
            }
            _ => panic!("expected Validation error"),
        }
    }

    #[test]
    fn trait_object_works() {
        let svc: Box<dyn PingService> = Box::new(DefaultPingService);
        let res = svc.process("dyn");
        assert!(res.is_ok());
    }
}

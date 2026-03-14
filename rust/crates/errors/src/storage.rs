#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("connection: {0}")]
    Connection(String),
    #[error("query: {0}")]
    Query(String),
    #[error("migration v{version}: {reason}")]
    Migration { version: String, reason: String },
    #[error("optimistic lock conflict")]
    OptimisticLock,
    #[error("transaction: {0}")]
    Transaction(String),
    #[error("constraint: {0}")]
    Constraint(String),
}

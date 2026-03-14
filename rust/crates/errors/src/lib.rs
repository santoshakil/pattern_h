mod domain;
mod ffi;
mod storage;

pub use domain::DomainError;
pub use ffi::FfiError;
pub use storage::StorageError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Ffi(#[from] FfiError),
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Storage(#[from] StorageError),
}

#[derive(Debug, thiserror::Error)]
pub enum FfiError {
    #[error("null pointer")]
    NullPointer,
    #[error("invalid handle: {0}")]
    InvalidHandle(u64),
    #[error("buffer overflow: need {required}, have {capacity}")]
    BufferOverflow { capacity: usize, required: usize },
    #[error("invalid utf8: {0}")]
    InvalidUtf8(String),
    #[error("decode error: {0}")]
    Decode(String),
    #[error("encode error: {0}")]
    Encode(String),
    #[error("not initialized")]
    NotInitialized,
    #[error("already initialized")]
    AlreadyInitialized,
    #[error("runtime init failed: {0}")]
    RuntimeInit(String),
}

impl FfiError {
    pub fn code(&self) -> i32 {
        match self {
            Self::NullPointer => 1,
            Self::InvalidHandle(_) => 2,
            Self::BufferOverflow { .. } => 3,
            Self::InvalidUtf8(_) => 4,
            Self::Decode(_) => 5,
            Self::Encode(_) => 6,
            Self::NotInitialized => 7,
            Self::AlreadyInitialized => 8,
            Self::RuntimeInit(_) => 9,
        }
    }
}

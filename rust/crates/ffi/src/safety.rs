use crate::FfiResult;
use errors::AppError;

pub fn catch_ffi<F>(f: F) -> FfiResult
where
    F: FnOnce() -> Result<Vec<u8>, AppError> + std::panic::UnwindSafe,
{
    match std::panic::catch_unwind(f) {
        Ok(Ok(data)) => FfiResult::ok(data),
        Ok(Err(e)) => {
            tracing::error!(error = %e, "ffi error");
            let code = match &e {
                AppError::Ffi(ffi) => ffi.code(),
                AppError::Domain(_) => 100,
                AppError::Storage(_) => 200,
            };
            FfiResult::err(code, &e.to_string())
        }
        Err(panic) => {
            let msg = panic
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| panic.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "unknown panic".into());
            tracing::error!(panic = %msg, "ffi panic caught");
            FfiResult::err(-1, &msg)
        }
    }
}

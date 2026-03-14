use errors::FfiError;

const MAX_FFI_BUFFER: usize = 64 * 1024 * 1024;

/// # Safety
/// Caller must ensure ptr is valid for len bytes.
pub unsafe fn validate_ptr<'a>(ptr: *const u8, len: usize) -> Result<&'a [u8], FfiError> {
    if ptr.is_null() {
        return Err(FfiError::NullPointer);
    }
    if len > MAX_FFI_BUFFER {
        return Err(FfiError::BufferOverflow {
            capacity: MAX_FFI_BUFFER,
            required: len,
        });
    }
    Ok(unsafe { std::slice::from_raw_parts(ptr, len) })
}

/// # Safety
/// Caller must ensure ptr points to valid UTF-8 bytes of length len.
pub unsafe fn validate_string<'a>(ptr: *const u8, len: usize) -> Result<&'a str, FfiError> {
    let bytes = unsafe { validate_ptr(ptr, len)? };
    std::str::from_utf8(bytes).map_err(|e| FfiError::InvalidUtf8(e.to_string()))
}

pub fn validate_handle(handle: u64) -> Result<u64, FfiError> {
    if handle == 0 {
        return Err(FfiError::InvalidHandle(0));
    }
    Ok(handle)
}

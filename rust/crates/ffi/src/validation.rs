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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_ptr_null_returns_null_pointer() {
        let res = unsafe { validate_ptr(std::ptr::null(), 10) };
        assert!(res.is_err());
        match res {
            Err(FfiError::NullPointer) => {}
            _ => panic!("expected NullPointer"),
        }
    }

    #[test]
    fn validate_ptr_valid_returns_slice() {
        let data = [1u8, 2, 3, 4];
        let res = unsafe { validate_ptr(data.as_ptr(), data.len()) };
        assert!(res.is_ok());
        let s = match res {
            Ok(v) => v,
            Err(_) => panic!("expected ok"),
        };
        assert_eq!(s, &[1, 2, 3, 4]);
    }

    #[test]
    fn validate_ptr_zero_len() {
        let data = [0u8; 1];
        let res = unsafe { validate_ptr(data.as_ptr(), 0) };
        assert!(res.is_ok());
        let s = match res {
            Ok(v) => v,
            Err(_) => panic!("expected ok"),
        };
        assert!(s.is_empty());
    }

    #[test]
    fn validate_ptr_oversized_returns_buffer_overflow() {
        let data = [0u8; 1];
        let huge = 128 * 1024 * 1024;
        let res = unsafe { validate_ptr(data.as_ptr(), huge) };
        assert!(res.is_err());
        match res {
            Err(FfiError::BufferOverflow { capacity, required }) => {
                assert_eq!(capacity, MAX_FFI_BUFFER);
                assert_eq!(required, huge);
            }
            _ => panic!("expected BufferOverflow"),
        }
    }

    #[test]
    fn validate_string_valid_utf8() {
        let s = "hello";
        let res = unsafe { validate_string(s.as_ptr(), s.len()) };
        assert!(res.is_ok());
        let val = match res {
            Ok(v) => v,
            Err(_) => panic!("expected ok"),
        };
        assert_eq!(val, "hello");
    }

    #[test]
    fn validate_string_empty() {
        let s = "";
        let res = unsafe { validate_string(s.as_ptr(), s.len()) };
        assert!(res.is_ok());
        let val = match res {
            Ok(v) => v,
            Err(_) => panic!("expected ok"),
        };
        assert_eq!(val, "");
    }

    #[test]
    fn validate_string_invalid_utf8() {
        let bad = [0xFF, 0xFE, 0xFD];
        let res = unsafe { validate_string(bad.as_ptr(), bad.len()) };
        assert!(res.is_err());
        match res {
            Err(FfiError::InvalidUtf8(_)) => {}
            _ => panic!("expected InvalidUtf8"),
        }
    }

    #[test]
    fn validate_handle_zero_returns_invalid() {
        let res = validate_handle(0);
        assert!(res.is_err());
        match res {
            Err(FfiError::InvalidHandle(0)) => {}
            _ => panic!("expected InvalidHandle(0)"),
        }
    }

    #[test]
    fn validate_handle_nonzero_returns_ok() {
        let res = validate_handle(42);
        assert!(res.is_ok());
        let val = match res {
            Ok(v) => v,
            Err(_) => panic!("expected ok"),
        };
        assert_eq!(val, 42);
    }

    #[test]
    fn validate_handle_max_returns_ok() {
        let res = validate_handle(u64::MAX);
        assert!(res.is_ok());
        let val = match res {
            Ok(v) => v,
            Err(_) => panic!("expected ok"),
        };
        assert_eq!(val, u64::MAX);
    }
}

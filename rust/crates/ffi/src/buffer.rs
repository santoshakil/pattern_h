#[repr(C)]
pub struct ByteBuffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub capacity: usize,
}

impl ByteBuffer {
    pub fn empty() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn from_vec(v: Vec<u8>) -> Self {
        let mut v = std::mem::ManuallyDrop::new(v);
        Self {
            ptr: v.as_mut_ptr(),
            len: v.len(),
            capacity: v.capacity(),
        }
    }

    /// # Safety
    /// The buffer must have been created by `from_vec` and not yet freed.
    pub unsafe fn into_vec(self) -> Vec<u8> {
        if self.ptr.is_null() {
            return Vec::new();
        }
        unsafe { Vec::from_raw_parts(self.ptr, self.len, self.capacity) }
    }

    /// # Safety
    /// The buffer must be valid and not yet freed.
    pub unsafe fn as_slice(&self) -> &[u8] {
        if self.ptr.is_null() || self.len == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

/// # Safety
/// ByteBuffer owns its allocation exclusively. The raw pointer is not shared
/// across threads without synchronization — callers must ensure the buffer is
/// not accessed concurrently. Send is safe because ownership can transfer
/// between threads. Sync is safe because shared references only read ptr/len/capacity.
unsafe impl Send for ByteBuffer {}
unsafe impl Sync for ByteBuffer {}

#[repr(C)]
pub struct FfiResult {
    pub success: bool,
    pub error_code: i32,
    pub data: ByteBuffer,
}

impl FfiResult {
    pub fn ok(data: Vec<u8>) -> Self {
        Self {
            success: true,
            error_code: 0,
            data: ByteBuffer::from_vec(data),
        }
    }

    pub fn ok_empty() -> Self {
        Self {
            success: true,
            error_code: 0,
            data: ByteBuffer::empty(),
        }
    }

    pub fn err(code: i32, msg: &str) -> Self {
        Self {
            success: false,
            error_code: code,
            data: ByteBuffer::from_vec(msg.as_bytes().to_vec()),
        }
    }
}

pub fn free_buffer(buf: ByteBuffer) {
    if !buf.ptr.is_null() {
        unsafe {
            drop(buf.into_vec());
        }
    }
}

pub fn free_result(result: FfiResult) {
    free_buffer(result.data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_buffer_has_null_ptr_and_zero_len() {
        let buf = ByteBuffer::empty();
        assert!(buf.ptr.is_null());
        assert_eq!(buf.len, 0);
        assert_eq!(buf.capacity, 0);
    }

    #[test]
    fn from_vec_preserves_data() {
        let data = vec![1u8, 2, 3, 4, 5];
        let buf = ByteBuffer::from_vec(data.clone());
        assert!(!buf.ptr.is_null());
        assert_eq!(buf.len, 5);
        let recovered = unsafe { buf.into_vec() };
        assert_eq!(recovered, data);
    }

    #[test]
    fn from_vec_empty_vec() {
        let buf = ByteBuffer::from_vec(Vec::new());
        assert_eq!(buf.len, 0);
        let recovered = unsafe { buf.into_vec() };
        assert!(recovered.is_empty());
    }

    #[test]
    fn as_slice_on_empty() {
        let buf = ByteBuffer::empty();
        let s = unsafe { buf.as_slice() };
        assert!(s.is_empty());
    }

    #[test]
    fn as_slice_on_data() {
        let data = vec![10u8, 20, 30];
        let buf = ByteBuffer::from_vec(data.clone());
        let s = unsafe { buf.as_slice() };
        assert_eq!(s, &[10, 20, 30]);
        unsafe { drop(buf.into_vec()) };
    }

    #[test]
    fn ffi_result_ok() {
        let data = vec![9u8, 8, 7];
        let r = FfiResult::ok(data);
        assert!(r.success);
        assert_eq!(r.error_code, 0);
        assert_eq!(r.data.len, 3);
        free_result(r);
    }

    #[test]
    fn ffi_result_ok_empty() {
        let r = FfiResult::ok_empty();
        assert!(r.success);
        assert_eq!(r.error_code, 0);
        assert!(r.data.ptr.is_null());
        assert_eq!(r.data.len, 0);
    }

    #[test]
    fn ffi_result_err() {
        let r = FfiResult::err(42, "something broke");
        assert!(!r.success);
        assert_eq!(r.error_code, 42);
        let msg = unsafe { r.data.as_slice() };
        assert_eq!(msg, b"something broke");
        free_result(r);
    }

    #[test]
    fn free_buffer_empty_no_panic() {
        let buf = ByteBuffer::empty();
        free_buffer(buf);
    }

    #[test]
    fn free_buffer_with_data() {
        let buf = ByteBuffer::from_vec(vec![1, 2, 3]);
        free_buffer(buf);
    }

    #[test]
    fn free_result_frees_correctly() {
        let r = FfiResult::ok(vec![100; 64]);
        free_result(r);
    }

    #[test]
    fn free_result_err_frees() {
        let r = FfiResult::err(1, "err msg");
        free_result(r);
    }
}

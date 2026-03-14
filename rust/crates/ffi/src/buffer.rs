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

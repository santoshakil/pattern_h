#![allow(unsafe_code)]

use std::sync::atomic::{AtomicBool, Ordering};

ffi::define_runtime!(RUNTIME, "app-core", 4);
ffi::define_event_port!(EVENT_PORT);

/// # Safety
/// Buffer must have been allocated by Rust via `ByteBuffer::from_vec`.
#[unsafe(no_mangle)]
pub extern "C" fn free_buffer(buf: ffi::ByteBuffer) {
    ffi::free_buffer(buf);
}

/// # Safety
/// Result must have been allocated by Rust.
#[unsafe(no_mangle)]
pub extern "C" fn free_result(result: ffi::FfiResult) {
    ffi::free_result(result);
}

static TRACING_INITIALIZED: AtomicBool = AtomicBool::new(false);

mod ffi_exports;

#[unsafe(no_mangle)]
pub extern "C" fn app_version() -> u32 {
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn app_init() -> ffi::FfiResult {
    ffi::catch_ffi(|| {
        if !TRACING_INITIALIZED.swap(true, Ordering::SeqCst) {
            use tracing_subscriber::EnvFilter;
            let filter =
                EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
            if let Err(e) = tracing_subscriber::fmt().with_env_filter(filter).try_init() {
                eprintln!("tracing init failed: {e}");
            }
        }
        RUNTIME.init()?;
        tracing::info!("app initialized");
        Ok(Vec::new())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn app_shutdown() -> ffi::FfiResult {
    ffi::catch_ffi(|| {
        RUNTIME.shutdown()?;
        tracing::info!("app shut down");
        Ok(Vec::new())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn app_is_initialized() -> bool {
    RUNTIME.is_initialized()
}

/// # Safety
/// `data` must be the pointer from `NativeApi.initializeApiDLData`.
#[unsafe(no_mangle)]
pub extern "C" fn app_init_dart_api(data: *mut std::ffi::c_void) {
    ffi::init_dart_api(data);
}

#[unsafe(no_mangle)]
pub extern "C" fn app_set_dart_port(port: i64) {
    EVENT_PORT.set_port(port);
}

#[unsafe(no_mangle)]
pub extern "C" fn app_disconnect_dart_port() {
    EVENT_PORT.disconnect();
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub extern "C" fn app_send_test_event() {
    EVENT_PORT.send_event(1, vec!["test event from Rust".into()]);
}

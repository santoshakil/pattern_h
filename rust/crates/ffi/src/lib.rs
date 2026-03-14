#![allow(unsafe_code)]

mod buffer;
mod dart_port;
mod handle_registry;
mod runtime;
mod safety;
mod validation;

pub use buffer::{ByteBuffer, FfiResult, free_buffer, free_result};
pub use dart_port::{EventValue, NativeEventPort, init_dart_api};
pub use handle_registry::HandleRegistry;
pub use runtime::AppRuntime;
pub use safety::catch_ffi;
pub use validation::{validate_handle, validate_ptr, validate_string};

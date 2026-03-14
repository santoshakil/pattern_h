use irondash_dart_ffi::{DartPort, DartValue};
use std::sync::atomic::{AtomicI64, Ordering};

pub use irondash_dart_ffi::DartValue as EventValue;

pub struct NativeEventPort {
    port: AtomicI64,
}

impl Default for NativeEventPort {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeEventPort {
    pub const fn new() -> Self {
        Self {
            port: AtomicI64::new(0),
        }
    }

    pub fn set_port(&self, port: i64) {
        self.port.store(port, Ordering::Release);
        tracing::info!(port, "dart event port set");
    }

    pub fn get_port(&self) -> Option<DartPort> {
        let port = self.port.load(Ordering::Acquire);
        if port == 0 {
            return None;
        }
        Some(DartPort::new(port))
    }

    pub fn is_connected(&self) -> bool {
        self.port.load(Ordering::Acquire) != 0
    }

    pub fn send(&self, values: Vec<DartValue>) -> bool {
        if let Some(port) = self.get_port() {
            port.send(values)
        } else {
            tracing::warn!("dart port not set, event dropped");
            false
        }
    }

    pub fn send_event(&self, event_id: i32, data: Vec<DartValue>) -> bool {
        let mut values = Vec::with_capacity(data.len() + 1);
        values.push(DartValue::I32(event_id));
        values.extend(data);
        self.send(values)
    }

    pub fn disconnect(&self) {
        self.port.store(0, Ordering::Release);
        tracing::info!("dart event port disconnected");
    }
}

pub fn init_dart_api(data: *mut std::ffi::c_void) {
    irondash_dart_ffi::irondash_init_ffi(data);
}

#[macro_export]
macro_rules! define_event_port {
    ($name:ident) => {
        static $name: $crate::NativeEventPort = $crate::NativeEventPort::new();
    };
}

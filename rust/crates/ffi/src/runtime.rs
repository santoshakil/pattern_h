use errors::{AppError, FfiError};
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::runtime::Runtime;

pub struct AppRuntime {
    rt: RwLock<Option<Runtime>>,
    initialized: AtomicBool,
    name: &'static str,
    workers: usize,
}

impl AppRuntime {
    pub const fn new(name: &'static str, workers: usize) -> Self {
        Self {
            rt: RwLock::new(None),
            initialized: AtomicBool::new(false),
            name,
            workers,
        }
    }

    pub fn init(&self) -> Result<(), AppError> {
        let mut guard = self.rt.write();
        if guard.is_some() {
            return Err(FfiError::AlreadyInitialized.into());
        }
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(self.workers)
            .thread_name(self.name)
            .enable_all()
            .build()
            .map_err(|e| FfiError::RuntimeInit(e.to_string()))?;
        *guard = Some(runtime);
        self.initialized.store(true, Ordering::Release);
        tracing::info!(name = self.name, workers = self.workers, "runtime started");
        Ok(())
    }

    pub fn shutdown(&self) -> Result<(), AppError> {
        let mut guard = self.rt.write();
        let rt = guard.take().ok_or(FfiError::NotInitialized)?;
        self.initialized.store(false, Ordering::Release);
        rt.shutdown_timeout(std::time::Duration::from_secs(5));
        tracing::info!(name = self.name, "runtime stopped");
        Ok(())
    }

    pub fn ensure_init(&self) -> Result<(), AppError> {
        if !self.initialized.load(Ordering::Acquire) {
            return Err(FfiError::NotInitialized.into());
        }
        Ok(())
    }

    pub fn block_on<F: std::future::Future>(&self, f: F) -> Result<F::Output, AppError> {
        if tokio::runtime::Handle::try_current().is_ok() {
            return Err(FfiError::RuntimeInit(
                "cannot call block_on from within a tokio runtime".into(),
            )
            .into());
        }
        let guard = self.rt.read();
        let rt = guard.as_ref().ok_or(FfiError::NotInitialized)?;
        Ok(rt.block_on(f))
    }

    pub fn spawn<F>(&self, f: F) -> Result<tokio::task::JoinHandle<F::Output>, AppError>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let guard = self.rt.read();
        let rt = guard.as_ref().ok_or(FfiError::NotInitialized)?;
        Ok(rt.spawn(f))
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Acquire)
    }
}

#[macro_export]
macro_rules! define_runtime {
    ($name:ident, $label:literal, $workers:expr) => {
        static $name: $crate::AppRuntime = $crate::AppRuntime::new($label, $workers);
    };
}

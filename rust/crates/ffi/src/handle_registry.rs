use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct HandleRegistry<T: Send + Sync> {
    map: DashMap<u64, T>,
    counter: AtomicU64,
}

impl<T: Send + Sync> HandleRegistry<T> {
    pub fn new() -> Self {
        Self {
            map: DashMap::new(),
            counter: AtomicU64::new(1),
        }
    }

    pub fn insert(&self, value: T) -> u64 {
        loop {
            let handle = self.counter.fetch_add(1, Ordering::Relaxed);
            if handle != 0 {
                self.map.insert(handle, value);
                return handle;
            }
        }
    }

    pub fn get(&self, handle: u64) -> Option<dashmap::mapref::one::Ref<'_, u64, T>> {
        self.map.get(&handle)
    }

    pub fn remove(&self, handle: u64) -> Option<T> {
        self.map.remove(&handle).map(|(_, v)| v)
    }

    pub fn contains(&self, handle: u64) -> bool {
        self.map.contains_key(&handle)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<T: Send + Sync> Default for HandleRegistry<T> {
    fn default() -> Self {
        Self::new()
    }
}

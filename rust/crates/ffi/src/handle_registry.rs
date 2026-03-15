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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_returns_nonzero() {
        let reg = HandleRegistry::new();
        let h = reg.insert(42u32);
        assert_ne!(h, 0);
    }

    #[test]
    fn get_returns_inserted() {
        let reg = HandleRegistry::new();
        let h = reg.insert(String::from("val"));
        let got = reg.get(h);
        assert!(got.is_some());
        if let Some(r) = got {
            assert_eq!(r.value(), "val");
        }
    }

    #[test]
    fn get_missing_returns_none() {
        let reg: HandleRegistry<i32> = HandleRegistry::new();
        assert!(reg.get(999).is_none());
    }

    #[test]
    fn remove_returns_value_and_clears() {
        let reg = HandleRegistry::new();
        let h = reg.insert(100i32);
        let removed = reg.remove(h);
        assert_eq!(removed, Some(100));
        assert!(reg.get(h).is_none());
    }

    #[test]
    fn remove_missing_returns_none() {
        let reg: HandleRegistry<i32> = HandleRegistry::new();
        assert!(reg.remove(999).is_none());
    }

    #[test]
    fn contains_existing() {
        let reg = HandleRegistry::new();
        let h = reg.insert("x");
        assert!(reg.contains(h));
    }

    #[test]
    fn contains_missing() {
        let reg: HandleRegistry<i32> = HandleRegistry::new();
        assert!(!reg.contains(999));
    }

    #[test]
    fn len_tracks_inserts() {
        let reg = HandleRegistry::new();
        assert_eq!(reg.len(), 0);
        reg.insert(1u8);
        assert_eq!(reg.len(), 1);
        reg.insert(2u8);
        assert_eq!(reg.len(), 2);
    }

    #[test]
    fn is_empty_initially() {
        let reg: HandleRegistry<i32> = HandleRegistry::new();
        assert!(reg.is_empty());
    }

    #[test]
    fn is_empty_after_insert() {
        let reg = HandleRegistry::new();
        reg.insert(1u32);
        assert!(!reg.is_empty());
    }

    #[test]
    fn len_decreases_on_remove() {
        let reg = HandleRegistry::new();
        let h = reg.insert(1u32);
        reg.insert(2u32);
        assert_eq!(reg.len(), 2);
        reg.remove(h);
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn multiple_inserts_unique_handles() {
        let reg = HandleRegistry::new();
        let h1 = reg.insert("a");
        let h2 = reg.insert("b");
        let h3 = reg.insert("c");
        assert_ne!(h1, h2);
        assert_ne!(h2, h3);
        assert_ne!(h1, h3);
    }

    #[test]
    fn default_creates_empty() {
        let reg: HandleRegistry<i32> = HandleRegistry::default();
        assert!(reg.is_empty());
    }
}

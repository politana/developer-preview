use std::sync::atomic::{AtomicU64, Ordering};

pub struct UniqueCounter {
    next_id: AtomicU64
}

impl UniqueCounter {
    pub const fn new() -> UniqueCounter {
        Self::new_starting_at(1)
    }

    pub const fn new_starting_at(value: u64) -> UniqueCounter {
        UniqueCounter { next_id: AtomicU64::new(value) }
    }

    pub fn next_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }
}

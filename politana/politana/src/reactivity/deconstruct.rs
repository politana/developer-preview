use std::sync::atomic::{AtomicBool, Ordering};


pub static DID_DECONSTRUCT: AtomicBool = AtomicBool::new(false);

pub fn deconstruct() {
    DID_DECONSTRUCT.store(true, Ordering::Relaxed);
}

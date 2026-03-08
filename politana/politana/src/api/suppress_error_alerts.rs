use std::sync::atomic::AtomicBool;

pub static SUPPRESS_ERROR_ALERTS: AtomicBool = AtomicBool::new(false);

use std::sync::atomic::AtomicBool;

thread_local! {
    pub static IS_TEST_SUITE: AtomicBool = AtomicBool::new(false);
}

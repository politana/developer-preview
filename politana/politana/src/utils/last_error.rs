use std::cell::RefCell;

thread_local! {
    pub static LAST_ERROR: RefCell<Option<String>> = Default::default();
}

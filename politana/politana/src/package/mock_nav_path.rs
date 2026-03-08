use std::cell::RefCell;

thread_local! {
    pub static MOCK_NAV_PATH: RefCell<Vec<String>> = RefCell::default();
}

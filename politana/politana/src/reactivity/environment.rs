use std::{cell::RefCell, hash::Hash};

use web_sys::{Document, Window};

use crate::{Closure, reactivity::window_resize::add_resize_observer, utils::{error_messages, report_error::report_error}};

pub struct Environment;

pub struct EnvironmentStorage {
    pub window: Window,
    pub document: Document,
}

thread_local! {
    static STORAGE: RefCell<Option<EnvironmentStorage>> = RefCell::default();
}

impl Environment {
    pub fn window() -> Window {
        EnvironmentStorage::map(|e| e.window.clone())
    }

    pub fn document() -> Document {
        EnvironmentStorage::map(|e| e.document.clone())
    }

    pub fn map_window_size<T: Hash + PartialEq + Eq + Clone + 'static>(
        closure: impl Fn(u32, u32) -> T + 'static,
    ) -> Closure<(), T> {
        let closure = Box::new(add_resize_observer(closure));
        Closure::new(move |_| closure())
    }
}

impl EnvironmentStorage {
    pub fn set_environment(storage: EnvironmentStorage) {
        STORAGE.set(Some(storage));
    }

    pub fn map<T>(closure: impl Fn(&EnvironmentStorage) -> T) -> T {
        STORAGE.with_borrow(|s| {
            closure(
                s.as_ref()
                    .unwrap_or_else(|| report_error(error_messages::EARLY_ENVIRONMENT)),
            )
        })
    }
}

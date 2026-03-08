use std::{cell::RefCell, collections::HashSet};

pub struct HeadResources;

thread_local! {
    static STORAGE: RefCell<HashSet<&'static str>> = Default::default();
}

impl HeadResources {
    #[cfg(target_arch = "wasm32")]
    pub fn use_resource(resource: &'static str) {
        use crate::{Environment, utils::{error_messages::browser_error, unwrap_or_error::UnwrapOrError}};
        let did_store = STORAGE.with_borrow_mut(|s| s.insert(resource));
        if did_store {
            Environment::document().head()
                .unwrap_or_report(&browser_error("Document does not have a head element"))
                .insert_adjacent_html("beforeend", resource)
                .ok().unwrap_or_report(&browser_error("Cannot append a resource to the document's head element"));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn use_resource(resource: &'static str) {
        // Store the resource anyway so we can track it when generating HTML
        STORAGE.with_borrow_mut(|s| s.insert(resource));
    }
}

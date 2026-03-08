use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{reactivity::vdom::Vdom, utils::unique_counter::UniqueCounter};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct VdomRef { id: u64 }

static VDOM_REF_ID_COUNTER: UniqueCounter = UniqueCounter::new_starting_at(1);
thread_local! {
    static VDOM_REF_STORAGE: RefCell<HashMap<u64, Rc<RefCell<Vdom>>>> = Default::default();
}

impl VdomRef {
    pub fn new(value: Vdom) -> Self {
        let id = VDOM_REF_ID_COUNTER.next_id();
        VDOM_REF_STORAGE.with_borrow_mut(
            |s| s.insert(id, Rc::new(RefCell::new(value)))
        );
        Self { id }
    }

    pub fn null() -> Self { Self { id: 0 } }

    #[cfg(target_arch = "wasm32")]
    pub fn exists(self) -> bool {
        VDOM_REF_STORAGE.with_borrow(|s| s.contains_key(&self.id))
    }

    /// Used if you need to disconnect a subset of references to a Vdom
    pub fn ephemeral_copy(self) -> Self {
        let new_id = VDOM_REF_ID_COUNTER.next_id();
        VDOM_REF_STORAGE.with_borrow_mut(|s| {
            if let Some(vdom_rc) = s.get(&self.id) {
                s.insert(new_id, vdom_rc.clone());
            }
        });
        Self { id: new_id }
    }

    /// Returns None if the Vdom has been destroyed
    pub fn map<T>(self, closure: impl FnOnce(&mut Vdom) -> T) -> Option<T> {
        let vdom = VDOM_REF_STORAGE.with_borrow(
            |s| s.get(&self.id).cloned()
        );
        Some(closure(&mut vdom?.borrow_mut()))
    }

    /// If the Vdom has been destroyed, does nothing
    pub fn update(self, closure: impl FnOnce(&mut Vdom)) {
        let vdom = VDOM_REF_STORAGE.with_borrow(
            |s| s.get(&self.id).cloned()
        );
        if let Some(vdom) = vdom {
            closure(&mut vdom.borrow_mut());
        }
    }

    pub fn will_remove_element(self) {
        let vdom = VDOM_REF_STORAGE.with_borrow(
            |s| s.get(&self.id).cloned()
        );
        if let Some(vdom) = vdom {
            let _resources: Vec<_> = vdom.borrow_mut().resources_drop_before.drain(..).collect();
        }
    }

    pub fn destroy(self) {
        // The Vdom is not dropped until with_borrow_mut returns
        VDOM_REF_STORAGE.with_borrow_mut(
            |s| s.remove(&self.id)
        );
    }
}

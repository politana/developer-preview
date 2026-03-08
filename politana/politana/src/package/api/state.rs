use std::{any::Any, cell::RefCell, collections::HashMap, hash::Hash, marker::PhantomData};

use crate::utils::{error_messages, report_error::report_error, scope_guard::DataGuard, unique_counter::UniqueCounter};

pub struct State<T> {
    id: u64,
    phantom: PhantomData<T>
}

struct StateStorage {
    data: Option<Box<dyn Any>>,
}

static STATE_ID_COUNTER: UniqueCounter = UniqueCounter::new();
thread_local! {
    static STORAGE: RefCell<HashMap<u64, StateStorage>> = Default::default();
}

impl <T: 'static> State<T> {
    pub fn new(value: T) -> Self {
        let id = STATE_ID_COUNTER.next_id();
        let storage = StateStorage {
            data: Some(Box::new(value))
        };
        STORAGE.with_borrow_mut(|s| s.insert(id, storage));
        Self { id, phantom: PhantomData }
    }

    pub fn map<U>(self, closure: impl FnOnce(&T) -> U) -> U {
        self.map_once(closure)
    }

    pub fn map_once<U>(self, closure: impl FnOnce(&T) -> U) -> U {
        let value = STORAGE.with_borrow_mut(
            |s| s.get_mut(&self.id)
                .unwrap_or_else(|| report_error(error_messages::STATE_READ_AFTER_FREE))
                .data
                .take()
                .unwrap_or_else(|| report_error(error_messages::SAME_STATE_FUNCTION_NESTING))
                .downcast::<T>()
                .unwrap_or_else(|_| report_error(error_messages::UNEXPECTED))
        );
        let cleanup = DataGuard::new(value, |value| {
            STORAGE.with_borrow_mut(
                |s| if let Some(storage) = s.get_mut(&self.id) {
                    storage.data = Some(value);
                }
            );
        });
        let result = closure(cleanup.value());
        result
    }

    pub fn set(self, closure: impl FnOnce(T) -> T) {
        let Some(old_value) = STORAGE.with_borrow_mut(
            |s| Some(s.get_mut(&self.id)?
                .data
                .take()
                .unwrap_or_else(|| report_error(error_messages::SAME_STATE_FUNCTION_NESTING))
                .downcast::<T>()
                .unwrap_or_else(|_| report_error(error_messages::UNEXPECTED)))
        ) else {
            return
        };
        let new_value = closure(*old_value);
        STORAGE.with_borrow_mut(|s| {
            if let Some(storage) = s.get_mut(&self.id) {
                storage.data = Some(Box::new(new_value));
            }
        });
    }

    pub fn update<U>(self, closure: impl FnOnce(&mut T) -> U) {
        self.set(|mut x| {
            closure(&mut x);
            x
        });
    }

    pub fn put(self, new_value: T) {
        self.set(|_| new_value)
    }

    pub(crate) fn exists(self) -> bool {
        STORAGE.with_borrow(|s| s.contains_key(&self.id))
    }

    #[allow(unused_variables)]
    pub(crate) fn is_inside_effect<R>(inside: bool, closure: impl FnOnce() -> R) -> R {
        closure()
    }
}

impl <T: 'static + Clone> State<T> {
    pub fn get(self) -> T {
        self.map(|x| x.clone())
    }

    pub fn get_once(self) -> T {
        self.map_once(|x| x.clone())
    }
}

impl <T> Clone for State<T> {
    fn clone(&self) -> Self { *self }
}

impl <T> Copy for State<T> {}

impl <T> PartialEq for State<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl <T> Eq for State<T> {}

impl <T> Hash for State<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl <T: Default + 'static> Default for State<T> {
    fn default() -> Self {
        State::new(Default::default())
    }
}

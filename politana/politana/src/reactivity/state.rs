use std::{any::Any, cell::RefCell, collections::{HashMap, HashSet}, hash::Hash, marker::PhantomData, sync::atomic::Ordering};

use crate::{reactivity::{deconstruct::DID_DECONSTRUCT, update_ui::update_ui, vdom::Vdom, vdom_ref::VdomRef}, utils::{error_messages, report_error::report_error, scope_guard::{DataGuard, ScopeGuard}, unique_counter::UniqueCounter, unwrap_or_error::UnwrapOrError}};

pub struct State<T> {
    id: u64,
    phantom: PhantomData<T>
}

struct StateStorage {
    /// If this is None, it means the data is currently being updated
    data: Option<Box<dyn Any>>,
    /// Child states that this state owns
    children: HashSet<u64>,
    /// Parts of the DOM that depend on this state
    consumers: HashSet<VdomRef>,
    /// The nesting level of the VDOM owner (of this or a parent state)
    owner_nesting_level: u64
}

static STATE_ID_COUNTER: UniqueCounter = UniqueCounter::new();
thread_local! {
    static STORAGE: RefCell<HashMap<u64, StateStorage>> = Default::default();
    /// Stack of State IDs. If non-empty, we are inside the set/update closure of the last element.
    static PARENT_STACK: RefCell<Vec<u64>> = Default::default();
    /// Usually, State::set isn't allowed with an active Vdom, but an exception is made within effects.
    static IS_INSIDE_EFFECT_STACK: RefCell<Vec<bool>> = RefCell::new(vec![false]);
}

impl StateStorage {
    fn drop_guard(state_id: u64) -> Box<dyn Any> {
        Box::new(ScopeGuard::new(
            move || { STORAGE.with_borrow_mut(|s| {
                let mut to_remove = vec![state_id];
                while let Some(id) = to_remove.pop() {
                    if let Some(storage) = s.remove(&id) {
                        for child in &storage.children {
                            to_remove.push(*child);
                        }
                    }
                }
            }); }
        ))
    }
}

impl <T: 'static> State<T> {
    // If this function is nested, the states will be assigned to the same parent and will live for the correct amount of time.
    pub fn new(value: T) -> Self {
        let id = STATE_ID_COUNTER.next_id();
        let parent_nesting;
        if let Some(parent_id) = PARENT_STACK.with_borrow_mut(|s| s.last().copied()) {
            // The state's parent takes ownership of it
            parent_nesting = STORAGE.with_borrow_mut(|s| {
                let Some(storage) = s.get_mut(&parent_id) else {
                    report_error(error_messages::NEW_STATE_INSIDE_FREED_PARENT)
                };
                storage.children.insert(id);
                storage.owner_nesting_level
            });
        } else if let Some(current_vdom) = Vdom::current() {
            // The containing HTML element takes ownership of the state
            current_vdom.update(|v| v.resources_drop_after.push(StateStorage::drop_guard(id)));
            parent_nesting = current_vdom.map(|v| v.nesting_level)
                .unwrap_or_else(|| report_error(error_messages::UNEXPECTED));
        } else {
            report_error(error_messages::MISPLACED_STATE_NEW);
        }
        let storage = StateStorage {
            data: Some(Box::new(value)),
            children: HashSet::new(),
            consumers: HashSet::new(),
            owner_nesting_level: parent_nesting
        };
        STORAGE.with_borrow_mut(|s| s.insert(id, storage));
        Self { id, phantom: PhantomData }
    }

    pub fn map<U>(self, closure: impl FnOnce(&T) -> U) -> U {
        if let Some(current_vdom) = Vdom::current() {
            STORAGE.with_borrow_mut(|s| {
                let Some(storage) = s.get_mut(&self.id) else {
                    report_error(error_messages::STATE_READ_AFTER_FREE)
                };
                let vdom_nesting = current_vdom.map(|v| v.nesting_level)
                    .unwrap_or_else(|| report_error(error_messages::UNEXPECTED));
                if vdom_nesting <= storage.owner_nesting_level {
                    // the state is being accessed from a Vdom ABOVE its owner nesting level
                    report_error(error_messages::STATE_READ_ABOVE_CREATION);
                }
                storage.consumers.insert(current_vdom);
            });
        } else {
            // Currently allowed and not a warning so that people can use State::get freely as long as they follow the rules:
            //  * Don't call get at the same level you define it
            //  * Don't let it escape the context
        }
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
        if DID_DECONSTRUCT.load(Ordering::Relaxed) {
            return;
        }
        if Vdom::current().is_some()
            && !IS_INSIDE_EFFECT_STACK
                .with_borrow(|s| s.last().copied())
                .unwrap_or_unexpected()
        {
            report_error(error_messages::STATE_WRITE_IN_UI);
        }
        let new_value;
        {
            let Some(old_value) = STORAGE.with_borrow_mut(
                |s| Some(s.get_mut(&self.id)?
                    .data
                    .take()
                    .unwrap_or_else(|| report_error(error_messages::SAME_STATE_FUNCTION_NESTING))
                    .downcast::<T>()
                    .unwrap_or_else(|_| report_error(error_messages::UNEXPECTED)))
            ) else {
                // If the state has been freed, do nothing
                return
            };
            PARENT_STACK.with_borrow_mut(|s| s.push(self.id));
            let _cleanup = ScopeGuard::new(|| {
                PARENT_STACK.with_borrow_mut(|s| s.pop());
            });
            // TODO: make this panic-safe
            new_value = closure(*old_value);
        } // run _cleanup
        let mut consumers = Vec::new();
        STORAGE.with_borrow_mut(|s| {
            if let Some(storage) = s.get_mut(&self.id) {
                storage.data = Some(Box::new(new_value));
                storage.consumers.retain(|consumer| {
                    if consumer.exists() {
                        consumers.push(*consumer);
                        true
                    } else {
                        false
                    }
                });
            }
        });
        for consumer in consumers {
            Vdom::using(consumer, update_ui);
        }
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

    pub(crate) fn is_inside_effect<R>(inside: bool, closure: impl FnOnce() -> R) -> R {
        IS_INSIDE_EFFECT_STACK.with_borrow_mut(|s| s.push(inside));
        let result = closure();
        IS_INSIDE_EFFECT_STACK.with_borrow_mut(|s| s.pop());
        result
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

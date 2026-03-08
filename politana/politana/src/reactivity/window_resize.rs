use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Event, Window};

use crate::{State, reactivity::hash_eq_clone::HashEq, utils::{error_messages::browser_error, unique_counter::UniqueCounter, unwrap_or_error::UnwrapOrError}};

struct Observer {
    closure: Box<dyn Fn(u32, u32) -> HashEq>,
    last_result: HashEq,
    set_state: Rc<dyn Fn(HashEq)>,
    state_exists: Box<dyn Fn() -> bool>
}

static OBSERVER_COUNTER: UniqueCounter = UniqueCounter::new();
thread_local! {
    static LAST_SIZE: RefCell<(u32, u32)> = RefCell::default();
    static OBSERVERS: RefCell<HashMap<u64, Observer>> = RefCell::default();
}

pub fn add_resize_observer<T: Hash + PartialEq + Eq + Clone + 'static>(
    closure: impl Fn(u32, u32) -> T + 'static
) -> impl Fn() -> T {
    let last_size = LAST_SIZE.with_borrow(|s| *s);
    let initial_value = closure(last_size.0, last_size.1);
    let initial_value_clone = closure(last_size.0, last_size.1);
    let state = State::new(initial_value);
    let observer = Observer {
        closure: Box::new(move |w, h| HashEq::new(closure(w, h))),
        last_result: HashEq::new(initial_value_clone),
        // UNEXPECTED: Only values of type T are ever put into the HashEq instance
        set_state: Rc::new(move |new_value| state.set(
            |_| *new_value.value_owned().downcast::<T>()
                .ok().unwrap_or_unexpected()
        )),
        state_exists: Box::new(move || state.exists())
    };
    OBSERVERS.with_borrow_mut(|o| o.insert(OBSERVER_COUNTER.next_id(), observer));
    move || state.get()
}

pub fn start_observing_resize(window: Window) {
    let window_clone = window.clone();
    let callback = Closure::<dyn FnMut(Event)>::new(move |_| {
        // Ignore if we can't get the size of the window
        (|| {
            let width = window_clone.inner_width().ok()?.as_f64()? as u32;
            let height = window_clone.inner_height().ok()?.as_f64()? as u32;
            on_window_resized(width, height);
            Some(())
        })();
    });
    window
        .add_event_listener_with_callback("resize", callback.as_ref().unchecked_ref())
        .ok().unwrap_or_report(&browser_error("Cannot add \"resize\" event listener to the window object"));
    let error = browser_error("Environment::map_window_size: Cannot access window.innerWidth and window.innerHeight");
    let initial_width = window.inner_width()
        .ok().unwrap_or_report(&error).as_f64().unwrap_or_report(&error) as u32;
    let initial_height = window.inner_height()
        .ok().unwrap_or_report(&error).as_f64().unwrap_or_report(&error) as u32;
    on_window_resized(initial_width, initial_height);
    callback.forget();
}

fn on_window_resized(width: u32, height: u32) {
    LAST_SIZE.set((width, height));
    OBSERVERS.with_borrow_mut(
        |o| o.retain(|_, observer| (observer.state_exists)())
    );
    let mut updates = Vec::new();
    OBSERVERS.with_borrow_mut(|o| {
        for (_, observer) in o.iter_mut() {
            let new_value = (observer.closure)(width, height);
            if new_value != observer.last_result {
                let new_value_clone = (observer.closure)(width, height);
                observer.last_result = new_value_clone;
                updates.push((observer.set_state.clone(), new_value));
            }
        }
    });
    for (set_state, val) in updates {
        set_state(val);
    }
}

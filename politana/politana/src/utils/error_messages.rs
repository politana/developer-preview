// Can't disable them on native architecture, because they are required for tests
#![allow(unused)]

pub const MISPLACED_STATE_NEW: &str = "Cannot call State::new, Closure::new, or State::default here.

You can call State::new (etc.) inside the functions that define your UI structure. These include the functions you pass to run_app and as an element's children.

You can also call State::new inside State::set and State::update (but not State::put).

However, you cannot call State::new anywhere else, such as in event handlers.

These restrictions exist because each State object is associated with a UI element. When that element is removed from the DOM, the data stored by the State object is freed.";

pub const UNEXPECTED: &str = "An unexpected internal error occurred.";

pub const NOT_INPUT_ELEMENT: &str = "Event::input_element called on an event whose target is not an input element.

You should only call this function within an event handler that's attached to an input element.";

pub const NOT_SELECT_ELEMENT: &str = "Event::select_element called on an event whose target is not a select element.

You should only call this function within an event handler that's attached to a select element.";

pub const NOT_TEXT_AREA_ELEMENT: &str = "Event::textarea_element called on an event whose target is not a textarea element.

You should only call this function within an event handler that's attached to a textarea element.";

pub const NOT_HTML_ELEMENT: &str = "Event::opaque_target_element called on an event whose target is not an HTML element.

You should only call this function within an event handler that's attached to an HTML element.";

pub const NEW_STATE_INSIDE_FREED_PARENT: &str = "Trying to create a new nested State instance (by calling State::set or State::update on the parent), but that parent state has already been freed.";

pub const STATE_READ_AFTER_FREE: &str = "Trying to read from a State instance (by calling State::get, State::map, or the \"once\" variants) or a Closure instance (by calling Closure::call), but that instance has already been freed.";

pub const SAME_STATE_FUNCTION_NESTING: &str = "Cannot nest calls to State::get, State::map, State::put, State::set, and State::update within each other when they are called on the same state.";

pub const DUPLICATE_FOR_EACH_KEY: &str = "A ForEach element had two identical keys. When using ForEach, each item must be unique. When using ForEachKeyed, the key function must return a unique key for each item.";

pub const EARLY_ENVIRONMENT: &str = "Cannot use Environment before run_app returns.";

pub const NESTED_USING_PSEUDO: &str = "Cannot nest El::using_pseudo";

pub const MISPLACED_NAV_HOST: &str = "Cannot call NavigationHost here.

NavigationHost is a view, so you must call it inside the functions that define your UI structure. These include the functions you pass to run_app, or as an element's children.

However, you cannot call NavigationHost anywhere else.";

pub const STATE_READ_ABOVE_CREATION: &str = "State::get or State::map called from the same component as State::new, or from a parent.

State::get and State::map must be called from a child of the component where the corresponding State::new is called.

There are three reasons why this might happen:

* Calling State::get right after State::new in the same component, e.g. in an if statement. Solution: wrap your returned element in a div or other container.

* Leaking state out through a view. Views should return El and should not expose internal State instances through any of its mutable parameters (including other State instances).

* Incorrectly nesting state. When dealing with nested State in data structures, you must call State::new inside the State::set or State::update that modifies the parent instance. You cannot use State::new inside State::put.";

pub const STATE_WRITE_IN_UI: &str = "Cannot call State::set, State::update, or State::put directly within the functions that define your UI structure.

Functions that return El are generally expected not to have any side effects. Consider moving the state mutation to El::on_appear, El::effect, or an event handler.";

pub fn browser_error(message: &str) -> String {
    "Browser issue: ".to_string() + message
}

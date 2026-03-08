use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
pub fn current_nav_prefix() -> Option<Vec<Rc<String>>> {
    use crate::reactivity::vdom::Vdom;
    Vdom::current().map(|v| v.map(|v| v.nav_prefix())).flatten()
}

// For the web environment, we would use this to enable nested nav hosts.
// For the compile-time environment, we do this process manually when recursively
// evaluating El instances, so this is the empty path.
#[cfg(not(target_arch = "wasm32"))]
pub fn current_nav_prefix() -> Option<Vec<Rc<String>>> {
    Some(Vec::new())
}

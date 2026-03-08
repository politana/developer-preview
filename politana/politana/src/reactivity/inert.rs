/// Has no impact on state-read tracking
#[cfg(target_arch = "wasm32")]
pub fn inert<T>(closure: impl FnOnce() -> T) {
    use crate::reactivity::vdom::Vdom;
    Vdom::inert(true, closure)
}

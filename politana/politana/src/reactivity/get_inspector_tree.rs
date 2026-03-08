use crate::{debug_bar::inspection::tree::InspectorElement, reactivity::vdom::Vdom};

pub fn get_inspector_tree() -> InspectorElement {
    Vdom::inspector_tree()
}

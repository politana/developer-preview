use crate::{debug_bar::inspection::tree::{InspectorChildren, InspectorElement}, reactivity::{vdom::VirtualElement, vdom_ref::VdomRef}, utils::unwrap_or_error::UnwrapOrError};

pub fn generate_inspector_tree(item: VdomRef) -> InspectorElement {
    item.map(|v| generate(
        // UNEXPECTED: Enclosing function will be called outside of rendering, so nobody will have ownership of the virtual element
        v.virtual_el.as_ref().unwrap_or_unexpected(),
        v.html.tag_name()
    ))
        // UNEXPECTED: VDOM is retrieved from the active tree and should exist.
        .unwrap_or_unexpected()
}

fn generate(virtual_el: &VirtualElement, html_tag: String) -> InspectorElement {
    match virtual_el {
        VirtualElement::ReplaceElement { current_children, .. } =>
            InspectorElement {
                tag: html_tag,
                children: InspectorChildren::Children(
                    current_children.iter()
                        .map(|c| generate_inspector_tree(*c))
                        .collect()
                )
            },
        VirtualElement::ForEach { current_children, content } =>
            InspectorElement {
                tag: html_tag,
                children: InspectorChildren::Children(
                    (content.items)().iter()
                        .map(|i| (content.item_id)(i))
                        // They should all be Some, but just in case
                        .filter_map(|i| current_children.get(&i))
                        .map(|c| generate_inspector_tree(*c))
                        .collect()
                )
            },
        VirtualElement::StaticString(str) =>
            InspectorElement {
                tag: html_tag,
                children: InspectorChildren::String(str().to_string())
            },
        VirtualElement::String(str) =>
            InspectorElement {
                tag: html_tag,
                children: InspectorChildren::String(str())
            },
        VirtualElement::InnerHtml(content) =>
            InspectorElement {
                tag: html_tag,
                children: InspectorChildren::InnerHtml(content().0)
            },
        _ => InspectorElement {
            tag: html_tag,
            children: InspectorChildren::Unexpected
        }
    }
}

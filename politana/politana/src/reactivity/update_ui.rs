use std::{collections::{HashMap, HashSet}, rc::Rc, sync::atomic::Ordering};

use js_sys::Reflect;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::Event;

use crate::{State, api::{el::{Children, ElInner}, property::Property}, reactivity::{create_html_element::CreateHtmlElement, deconstruct::DID_DECONSTRUCT, hash_eq_clone::HashEq, style_manager::{StyleManager, StyleSpec}, vdom::{Vdom, VirtualElement}, vdom_ref::VdomRef}, utils::{error_messages::{self, browser_error}, report_error::report_error, scope_guard::ScopeGuard, unwrap_or_error::UnwrapOrError}};

/// Updates the DOM at Vdom::current
pub fn update_ui() {
    Vdom::inert(false, || {
        State::<()>::is_inside_effect(false, || {
            if let Some(current) = Vdom::current() {
                update(current);
            }
        });
    });
}

struct DeferredWork {
    function: fn(VdomRef, Rc<dyn Fn() -> bool>, VdomRef),
    vdom: VdomRef,
    closure: Rc<dyn Fn() -> bool>,
    last_run_dependencies: VdomRef
}

fn update(vdom: VdomRef) -> Option<()> {
    let mut virtual_el = vdom.map(|v| v.virtual_el.take())?
        .unwrap_or_unexpected();
    let mut deferred_work: Option<DeferredWork> = None;
    // Capture early returns so we can do cleanup
    let mut perform_update = || { match &mut virtual_el {
        VirtualElement::ReplaceElement { current_children, content } => {
            vdom.will_remove_element();
            let _resources = vdom.map(|v| v.resources_drop_after.drain(..).collect::<Vec<_>>());
            let new_el_wrapped = content();
            let run_lifecycle = || {
                let lifecycle = new_el_wrapped.lifecycle;
                let html = vdom.map(|v| v.html.clone())?;
                for mut handler in lifecycle.on_appear {
                    if let Some(on_appear) = handler.take() {
                        Vdom::inert(true, || {
                            on_appear(html.clone());
                        });
                    }
                }
                for mut handler in lifecycle.on_disappear {
                    if let Some(on_disappear) = handler.take() {
                        let html = vdom.map(|v| v.html.clone())?;
                        let guard = ScopeGuard::new(|| {
                            Vdom::inert(true, || {
                                on_disappear(html)
                            });
                        });
                        vdom.update(|v| v.resources_drop_before.push(Box::new(guard)));
                    }
                }
                Some(())
            };
            // An iterator so we don't have to collect it unless it is needed
            let nav_prefix = vdom
                .map(|v| v.nav_prefix.clone())?
                .into_iter()
                .chain(new_el_wrapped.navigation_path.into_iter().map(|x| Rc::new(x)));
            let new_el;
            match new_el_wrapped.inner {
                ElInner::Static(el_data) => {
                    new_el = el_data;
                }
                ElInner::Flattened(view) => {
                    let flattened_vdom = VdomRef::new(Vdom::new(
                        vdom.map(|v| v.html.clone())?,
                        VirtualElement::ReplaceElement {
                            current_children: Vec::new(),
                            content: view
                        },
                        nav_prefix.collect()
                    ));
                    for child in current_children.iter() {
                        child.destroy();
                    }
                    current_children.clear();
                    current_children.push(flattened_vdom);
                    Vdom::using(flattened_vdom, update_ui);
                    // Bring new HTML element up to the parent
                    if let Some(new_html) = flattened_vdom.map(|v| v.html.clone()) {
                        vdom.update(|v| v.html = new_html);
                    }
                    run_lifecycle()?;
                    return Some(());
                }
            }
            let new_html = Vdom::document().create_html_element(new_el.tag);
            vdom.update(|v|
                v.html.replace_with_with_node_1(&new_html)
                    .ok().unwrap_or_report(&browser_error("Element.replaceWith failed when updating the document"))
            );
            vdom.update(|v| v.html = new_html);
            for child in current_children.iter() {
                child.destroy();
            }
            current_children.clear();
            match new_el.children {
                Children::StaticString(str) => {
                    let str_vdom = VdomRef::new(Vdom::new(
                        vdom.map(|v| v.html.clone())?,
                        VirtualElement::StaticString(str),
                        // Can't have any children that would read this
                        Vec::new()
                    ));
                    current_children.push(str_vdom);
                    Vdom::using(str_vdom, update_ui);
                }
                Children::String(str) => {
                    let str_vdom = VdomRef::new(Vdom::new(
                        vdom.map(|v| v.html.clone())?,
                        VirtualElement::String(str),
                        // Can't have any children that would read this
                        Vec::new()
                    ));
                    current_children.push(str_vdom);
                    Vdom::using(str_vdom, update_ui);
                }
                Children::Fixed(items) => {
                    let nav_prefix: Vec<_> = nav_prefix.collect();
                    for item in items {
                        let dummy_element = Vdom::document().create_html_element("div");
                        vdom.update(|v|
                            v.html.append_with_node_1(&dummy_element)
                                .ok().unwrap_or_report(&browser_error("Element.append failed when updating the document)"))
                        );
                        let el_vdom = VdomRef::new(Vdom::new(
                            dummy_element,
                            VirtualElement::ReplaceElement {
                                current_children: Vec::new(),
                                content: item
                            },
                            nav_prefix.clone()
                        ));
                        current_children.push(el_vdom);
                        Vdom::using(el_vdom, update_ui);
                    }
                }
                Children::ForEach(content) => {
                    let fe_vdom = VdomRef::new(Vdom::new(
                        vdom.map(|v| v.html.clone())?,
                        VirtualElement::ForEach {
                            current_children: HashMap::new(),
                            content
                        },
                        nav_prefix.collect()
                    ));
                    current_children.push(fe_vdom);
                    Vdom::using(fe_vdom, update_ui);
                }
                Children::InnerHtml(content) => {
                    let str_vdom = VdomRef::new(Vdom::new(
                        vdom.map(|v| v.html.clone())?,
                        VirtualElement::InnerHtml(content),
                        // Can't have any children that would read this
                        Vec::new()
                    ));
                    current_children.push(str_vdom);
                    Vdom::using(str_vdom, update_ui);
                }
            }
            vdom.update(|v| for property in v.properties.drain(..) {
                // Make sure effects are properly destroyed
                property.map(|p| {
                    if let Some(VirtualElement::Effect { last_run_dependencies, .. }) = &p.virtual_el {
                        Some(*last_run_dependencies)
                    } else {
                        None
                    }
                }).flatten().inspect(|v| v.destroy());
                property.destroy();
            });
            for property in new_el.properties {
                let prop_vdom = VdomRef::new(Vdom::new(
                    vdom.map(|v| v.html.clone())?,
                    VirtualElement::UpdateProperty {
                        key: property.0,
                        value: property.1,
                        current_style: None
                    },
                    // Properties can't create views
                    Vec::new()
                ));
                vdom.update(|v| v.properties.push(prop_vdom));
                Vdom::using(prop_vdom, update_ui);
            }
            {
                let classes_vdom = VdomRef::new(Vdom::new(
                    vdom.map(|v| v.html.clone())?,
                    VirtualElement::UpdateClassList {
                        computed: new_el.classes,
                        current: Vec::new()
                    },
                    // Won't be used from class setters
                    Vec::new()
                ));
                vdom.update(|v| v.properties.push(classes_vdom));
                Vdom::using(classes_vdom, update_ui);
            }
            for event_listener in new_el.event_listeners {
                let f = event_listener.listener.clone();
                let closure: Closure<dyn Fn(Event)> = Closure::wrap(Box::new(
                    move |event: Event| {
                        if !DID_DECONSTRUCT.load(Ordering::Relaxed) {
                            f(event)
                        }
                    }
                ));
                vdom.update(|v|
                    v.html.add_event_listener_with_callback(
                        event_listener.event_type,
                        closure.as_ref().unchecked_ref()
                    ).ok().unwrap_or_report(&browser_error(&format!("Cannot add event listener \"{}\"", event_listener.event_type)))
                );
                vdom.update(|v| v.resources_drop_after.push(Box::new(closure)));
            }
            run_lifecycle()?;
            // effects after on_appear
            for effect in new_el.effects {
                let effect_vdom = VdomRef::new(Vdom::new(
                    vdom.map(|v| v.html.clone())?,
                    VirtualElement::Effect {
                        closure: effect,
                        last_run_dependencies: VdomRef::null()
                    },
                    // Effects can't create views
                    Vec::new()
                ));
                // Use the properties array to hold the effects (they are automatically cleaned up when the element disappears)
                vdom.update(|v| v.properties.push(effect_vdom));
                Vdom::using(effect_vdom, update_ui);
            }
        }
        VirtualElement::ForEach { current_children, content } => {
            let new_items = (content.items)();
            check_for_each_key_duplicates(new_items.iter().map(&*content.item_id).collect());
            let mut used_keys = HashSet::new();
            for item in new_items {
                let item_key = (content.item_id)(&item);
                if let Some(existing_child) = current_children.get(&item_key) {
                    // Move an existing element
                    let existing_child_html = existing_child.map(|c| c.html.clone())?;
                    vdom.update(|v|
                        v.html.append_with_node_1(&existing_child_html)
                            .ok().unwrap_or_report(&browser_error("Element.append failed when moving ForEach elements"))
                    );
                } else {
                    let extra_item_key = (content.item_id)(&item);
                    // Create a new element
                    let dummy_element = Vdom::document().create_html_element("div");
                    vdom.update(|v|
                        v.html.append_with_node_1(&dummy_element)
                            .ok().unwrap_or_report(&browser_error("Element.append failed when moving ForEach elements"))
                    );
                    let element_fn = content.element.clone();
                    let el_vdom = VdomRef::new(Vdom::new(
                        dummy_element,
                        VirtualElement::ReplaceElement {
                            current_children: Vec::new(),
                            content: Rc::new(move || (element_fn)(item.clone()))
                        },
                        // A ForEach is not an El and can't add to the nav prefix
                        vdom.map(|v| v.nav_prefix.clone())?
                    ));
                    current_children.insert(extra_item_key, el_vdom);
                    Vdom::using(el_vdom, update_ui);
                }
                used_keys.insert(item_key);
            }
            current_children.retain(|k, vdom_ref| if used_keys.contains(k) {
                true
            } else {
                vdom_ref.will_remove_element();
                vdom_ref.map(|v| v.html.clone()).map(|el| el.remove());
                vdom_ref.destroy();
                false
            });
        },
        VirtualElement::UpdateProperty {
            key: prop_key, value: property, current_style
        } => {
            match property {
                Property::Style(value) => {
                    let style_value = value();
                    let new_style = StyleSpec {
                        key: *prop_key,
                        value: style_value
                    };
                    let class_name = StyleManager::use_style(&new_style);
                    let old_class_name = current_style.as_ref().map(StyleManager::release_style).flatten();
                    *current_style = Some(new_style);
                    if old_class_name.as_ref() != Some(&class_name) {
                        old_class_name.map(|old_class_name| {
                            vdom.update(|v|
                                v.html.class_list().remove_1(&old_class_name)
                                    .ok().unwrap_or_report(&browser_error("Cannot remove internal style classes from an HTML element"))
                            );
                        });
                        vdom.update(|v|
                            v.html.class_list().add_1(&class_name)
                                .ok().unwrap_or_report(&browser_error("Cannot add internal style classes to an HTML element"))
                        );
                    }
                }
                Property::Attribute(value) => {
                    let new_attribute_value = value();
                    if prop_key.name == "class" {
                        // Should be handled in the El::attr function
                        report_error(error_messages::UNEXPECTED);
                    } else {
                        vdom.update(|v|
                            v.html.set_attribute(prop_key.name, &new_attribute_value)
                                .ok().unwrap_or_report(&browser_error(&format!("Cannot set attribute \"{}\" on an HTML element", prop_key.name)))
                        );
                    }
                }
                Property::BooleanAttribute(value) => {
                    if value() {
                        vdom.update(|v|
                            v.html.set_attribute(prop_key.name, "")
                                .ok().unwrap_or_report(&browser_error(&format!("Cannot set boolean attribute \"{}\" on an HTML element", prop_key.name)))
                        )
                    } else {
                        vdom.update(|v|
                            v.html.remove_attribute(prop_key.name)
                                .ok().unwrap_or_report(&browser_error(&format!("Cannot remove boolean attribute \"{}\" from an HTML element", prop_key.name)))
                        );
                    }
                }
                Property::Value(value) => {
                    let value = value();
                    let value_prop_result = vdom.map(
                        |v| Reflect::set(&v.html, &"value".into(), &value.clone().into())
                    )?;
                    if let Err(_) = value_prop_result {
                        vdom.update(|v|
                            v.html.set_attribute("value", &value)
                                .ok().unwrap_or_report(&browser_error("Cannot set the value property or attribute on an HTML element"))
                        );
                    }
                }
            }
        }
        VirtualElement::UpdateClassList { computed, current } => {
            let new_classes: HashSet<String> = computed
                .into_iter().map(|s| s()).flatten().collect();
            let current_classes: HashSet<String> = current
                .clone().into_iter().collect();
            let classes_to_remove = &current_classes - &new_classes;
            let classes_to_add = &new_classes - &current_classes;
            // TODO: error handling for malformed class names
            for class in classes_to_remove {
                vdom.update(|v|
                    v.html.class_list().remove_1(&class)
                        .ok().unwrap_or_report(&browser_error(&format!("Cannot add class named \"{}\" to an HTML element. The class name might not be valid.", class)))
                );
            }
            for class in classes_to_add {
                vdom.update(|v|
                    v.html.class_list().add_1(&class)
                        .ok().unwrap_or_report(&browser_error(&format!("Cannot remove class named \"{}\" from an HTML element. The class name might not be valid.", class)))
                );
            }
            *current = new_classes.into_iter().collect();
        }
        VirtualElement::StaticString(str) => {
            let str = str();
            vdom.update(|v| v.html.set_text_content(Some(str)));
        }
        VirtualElement::String(str) => {
            let str = str();
            vdom.update(|v| v.html.set_text_content(Some(&str)));
        }
        VirtualElement::InnerHtml(content) => {
            let content = content().0;
            vdom.update(|v| v.html.set_inner_html(&content));
        }
        VirtualElement::Effect { closure, last_run_dependencies } => {
            deferred_work = Some(DeferredWork {
                function: |vdom, closure, last_run_dependencies| {
                    let update_handle = vdom.ephemeral_copy();
                    let new_deps = vdom.ephemeral_copy();
                    let should_retain_references = Vdom::using(new_deps, || {
                        State::<()>::is_inside_effect(true, || {
                            closure()
                        })
                    });
                    if should_retain_references {
                        last_run_dependencies.destroy();
                        update_handle.update(|v| v.virtual_el = Some(VirtualElement::Effect {
                            closure,
                            last_run_dependencies: new_deps
                        }));
                    } else {
                        new_deps.destroy();
                        update_handle.update(|v| v.virtual_el = Some(VirtualElement::Effect {
                            closure, last_run_dependencies }));
                    }
                    update_handle.destroy();
                },
                vdom,
                closure: closure.clone(),
                last_run_dependencies: *last_run_dependencies
            })
        }
    }; Some(()) };
    perform_update();
    vdom.update(|v| v.virtual_el = Some(virtual_el));
    if let Some(deferred_work) = deferred_work {
        (deferred_work.function)(deferred_work.vdom, deferred_work.closure, deferred_work.last_run_dependencies);
    }
    Some(())
}

fn check_for_each_key_duplicates(keys: Vec<HashEq>) {
    let mut set = HashSet::new();
    for key in keys {
        if !set.insert(key) {
            report_error(error_messages::DUPLICATE_FOR_EACH_KEY);
        }
    }
}

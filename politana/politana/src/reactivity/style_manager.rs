use std::{cell::RefCell, collections::HashMap};

use web_sys::HtmlElement;

use crate::{Environment, api::property::PropertyKey, reactivity::create_html_element::CreateHtmlElement, utils::{unique_counter::UniqueCounter, unwrap_or_error::UnwrapOrError}};

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct StyleSpec {
    pub key: PropertyKey,
    pub value: String
}

struct StoredStyle {
    class_name: String,
    html: HtmlElement,
    ref_count: u64
}

#[derive(Hash, PartialEq, Eq)]
enum StyleKey {
    Spec(StyleSpec),
    Global(&'static str)
}

#[derive(Default)]
pub struct StyleManager {
    styles: HashMap<StyleKey, StoredStyle>
}

static CLASS_NAME_COUNTER: UniqueCounter = UniqueCounter::new_starting_at(10000);
thread_local! {
    static STYLE_MANAGER: RefCell<StyleManager> = RefCell::default();
}

impl StyleManager {
    pub fn use_style(style: &StyleSpec) -> String {
        let style_key = StyleKey::Spec(style.clone());
        if let Some(existing_class_name) = STYLE_MANAGER.with_borrow_mut(|s| {
            s.styles.get_mut(&style_key).map(|s| {
                s.ref_count += 1;
                s.class_name.clone()
            })
        }) {
            return existing_class_name;
        }
        let document = Environment::document();
        let html = document.create_html_element("style");
        let class_name = format!("politana-internal-{}", CLASS_NAME_COUNTER.next_id());
        html.set_text_content(Some(
            &style.key.create_style_rule(&class_name, &style.value)
        ));
        document.head().unwrap_or_report("Document has no head element")
            .append_child(&html)
            .ok().unwrap_or_report("Cannot append a style element to the head element");
        let stored_style = StoredStyle {
            class_name: class_name.clone(),
            html,
            ref_count: 1
        };
        STYLE_MANAGER.with_borrow_mut(|s| {
            s.styles.insert(style_key, stored_style)
        });
        class_name
    }

    /// If the style existed, returns the class name (which may no longer be valid)
    pub fn release_style(style: &StyleSpec) -> Option<String> {
        let style_key = StyleKey::Spec(style.clone());
        STYLE_MANAGER.with_borrow_mut(|s| {
            let mut class_name = None;
            let should_remove = if let Some(stored) = s.styles.get_mut(&style_key) {
                stored.ref_count -= 1;
                class_name = Some(stored.class_name.clone());
                stored.ref_count == 0
            } else {
                false
            };
            if should_remove {
                if let Some(stored) = s.styles.remove(&style_key) {
                    stored.html.remove();
                }
            }
            class_name
        })
    }

    pub fn use_global_style(style: &'static str) {
        let style_key = StyleKey::Global(style);
        if let Some(_) = STYLE_MANAGER.with_borrow_mut(|s|
            s.styles.get_mut(&style_key).map(|s| s.ref_count += 1)
        ) {
            return;
        }
        let document = Environment::document();
        let html = document.create_html_element("style");
        html.set_text_content(Some(style));
        document.head().unwrap_or_report("Document has no head element")
            .append_child(&html)
            .ok().unwrap_or_report("Cannot append a style element to the head element");
        let stored_style = StoredStyle {
            class_name: "".to_string(),
            html,
            ref_count: 1
        };
        STYLE_MANAGER.with_borrow_mut(|s| {
            s.styles.insert(style_key, stored_style)
        });
    }

    pub fn release_global_style(style: &'static str) {
        let style_key = StyleKey::Global(style);
        STYLE_MANAGER.with_borrow_mut(|s| {
            let should_remove = if let Some(stored) = s.styles.get_mut(&style_key) {
                stored.ref_count -= 1;
                stored.ref_count == 0
            } else {
                false
            };
            if should_remove {
                if let Some(stored) = s.styles.remove(&style_key) {
                    stored.html.remove();
                }
            }
        })
    }
}

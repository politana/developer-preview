use std::collections::HashMap;

use crate::{reactivity::style_manager::StyleSpec, utils::unique_counter::UniqueCounter};

struct StoredStyle {
    class_name: String
}

#[derive(Hash, PartialEq, Eq)]
enum StyleKey {
    Spec(StyleSpec),
    Global(&'static str)
}

#[derive(Default)]
pub struct MiniStyleManager {
    styles: HashMap<StyleKey, StoredStyle>
}

static CLASS_NAME_COUNTER: UniqueCounter = UniqueCounter::new_starting_at(10000);

impl MiniStyleManager {
    pub fn use_style(&mut self, style: &StyleSpec) -> String {
        let style_key = StyleKey::Spec(style.clone());
        if let Some(existing_class_name) = self.styles.get_mut(&style_key).map(|s| {
            s.class_name.clone()
        }) {
            return existing_class_name;
        }
        let class_name = format!("politana-internal-{}", CLASS_NAME_COUNTER.next_id());
        let stored_style = StoredStyle {
            class_name: class_name.clone()
        };
        self.styles.insert(style_key, stored_style);
        class_name
    }

    pub fn use_global_style(&mut self, style: &'static str) {
        let style_key = StyleKey::Global(style);
        let stored_style = StoredStyle {
            class_name: "".to_string()
        };
        self.styles.insert(style_key, stored_style);
    }

    pub fn make_head_content(&self) -> String {
        let mut classes = String::new();
        let mut global = String::new();
        for (style_key, stored_style) in self.styles.iter() {
            match style_key {
                StyleKey::Spec(style_spec) => {
                    let rule = style_spec.key.create_style_rule(
                        &stored_style.class_name,
                        &style_spec.value
                    );
                    classes.push_str(&format!("<style>{rule}</style>"));
                }
                StyleKey::Global(style) => {
                    global.push_str(style);
                }
            }
        }
        format!("{classes}\n<style>{global}</style>")
    }
}

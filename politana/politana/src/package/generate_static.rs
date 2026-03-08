use std::collections::HashMap;

use crate::{El, api::{computed::Computed, el::{Children, ElInner}, property::{Property, PropertyKey}}, package::{mini_style_manager::MiniStyleManager, mock_nav_path::MOCK_NAV_PATH}, reactivity::style_manager::StyleSpec};

pub struct StaticContent {
    pub head: String,
    pub body: String
}

pub fn generate_static(
    el: impl Fn() -> El,
    nav_path: Vec<&'static str>
) -> StaticContent {
    let mut generator = StaticGenerator {
        style_manager: Default::default(),
        nav_path
    };
    let body = generator.generate_static(Box::new(el));
    let head = generator.style_manager.make_head_content();
    StaticContent { head, body }
}

struct StaticGenerator {
    style_manager: MiniStyleManager,
    nav_path: Vec<&'static str>
}

impl StaticGenerator {
    fn generate_static<'a>(&mut self, el: Box<dyn Fn() -> El + 'a>) -> String {
        MOCK_NAV_PATH.set(self.nav_path.iter().map(|s| s.to_string()).collect());
        let el = el();
        self.nav_path.drain(0..el.navigation_path.len());
        for global_style in &el.global_css {
            self.style_manager.use_global_style(global_style);
        }
        let data = match el.inner {
            ElInner::Static(el_data) => el_data,
            ElInner::Flattened(f) => return self.generate_static(Box::new(|| f()))
        };
        let tag = data.tag;
        let properties = self.properties(data.properties, data.classes);
        if is_void_element(tag) {
            return format!("<{tag}{properties}>");
        }
        let inner = self.inner_html(data.children);
        format!("<{tag}{properties}>{inner}</{tag}>")
    }

    fn inner_html(&mut self, children: Children) -> String {
        match children {
            Children::Fixed(items) => items.iter()
                .map(|i| self.generate_static(Box::new(|| i())))
                .collect::<String>(),
            Children::StaticString(str) => html_escape::encode_text(str()).to_string(),
            Children::String(str) => html_escape::encode_text(&str()).to_string(),
            Children::ForEach(content) => (content.items)().iter()
                .map(|i| || (content.element)(i.clone()))
                .map(|e| self.generate_static(Box::new(e)))
                .collect::<String>(),
            Children::InnerHtml(content) => content().0
        }
    }

    fn properties(
        &mut self,
        properties: HashMap<PropertyKey, Property>,
        classes: Vec<Computed<Vec<String>>>
    ) -> String {
        let mut other_attributes = String::new();
        let mut style_classes = Vec::new();
        for (property_key, property) in properties {
            match property {
                Property::Style(value) => {
                    style_classes.push(self.style_manager.use_style(&StyleSpec {
                        key: property_key,
                        value: value()
                    }));
                }
                Property::Attribute(value) => {
                    let key = property_key.name;
                    let value = value();
                    let value = html_escape::encode_quoted_attribute(&value);
                    other_attributes.push_str(&format!(" {key}=\"{value}\""));
                }
                Property::BooleanAttribute(is_enabled) => {
                    if is_enabled() {
                        other_attributes.push_str(&format!(" {}", property_key.name));
                    }
                }
                Property::Value(value) => {
                    let value = value();
                    let value = html_escape::encode_quoted_attribute(&value);
                    other_attributes.push_str(&format!(" value=\"{value}\""));
                }
            }
        }
        let mut classes = classes.iter().map(|c| c()).flatten().collect::<Vec<_>>();
        classes.extend(style_classes.into_iter());
        let classes_string = if !classes.is_empty() {
            format!(
                " class=\"{}\"",
                html_escape::encode_quoted_attribute(&classes.join(" "))
            )
        } else { "".to_string() };
        classes_string + &other_attributes
    }
}

fn is_void_element(tag: &'static str) -> bool {
    match tag {
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" |
            "link" | "meta" | "param" | "source" | "track" | "wbr" => true,
        _ => false
    }
}

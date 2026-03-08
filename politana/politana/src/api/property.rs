use std::fmt::Write;

use crate::{El, api::computed::Computed, utils::{error_messages, report_error::report_error}};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct PropertyKey {
    pub name: &'static str,
    pub pseudo: Pseudo
}

pub enum Property {
    Style(Computed<String>),
    Attribute(Computed<String>),
    BooleanAttribute(Computed<bool>),
    Value(Computed<String>)
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Pseudo {
    None,
    // elements
    FirstLetter,
    FirstLine,
    Selection,
    Placeholder,
    Marker,
    // classes
    Active,
    Focus,
    Visited,
    Disabled,
    Enabled,
    Checked,
    // class + media
    Hover
}

impl Pseudo {
    fn specificity_boost(&self) -> usize {
        match self {
            Pseudo::None |
                Pseudo::FirstLetter |
                Pseudo::FirstLine |
                Pseudo::Selection |
                Pseudo::Placeholder |
                Pseudo::Marker => 1,
            Pseudo::Visited |
                Pseudo::Enabled |
                Pseudo::Checked => 2,
            Pseudo::Hover => 3,
            Pseudo::Focus => 4,
            Pseudo::Active => 5,
            Pseudo::Disabled => 6,
        }
    }
}

impl PropertyKey {
    pub fn create_style_rule(self, class: &String, value: &String) -> String {
        let pseudoclass_name = match self.pseudo {
            Pseudo::None => "",
            Pseudo::FirstLetter => "::first-letter",
            Pseudo::FirstLine => "::first-line",
            Pseudo::Selection => "::selection",
            Pseudo::Placeholder => "::placeholder",
            Pseudo::Marker => "::marker",
            Pseudo::Active => ":active",
            Pseudo::Focus => ":focus",
            Pseudo::Visited => ":visited",
            Pseudo::Disabled => ":disabled",
            Pseudo::Enabled => ":enabled",
            Pseudo::Checked => ":checked",
            Pseudo::Hover => ":hover"
        };
        // Make sure each class has at least a specificity of 2 so it doesn't interfere with custom classes
        let boost = self.pseudo.specificity_boost() + 1;
        let is_hover = self.pseudo == Pseudo::Hover;
        let selector_len = boost * (class.len() + 1);
        let rule_body_len = pseudoclass_name.len() + self.name.len() + value.len() + 8;
        let media_wrapper_len = if is_hover { 26 } else { 0 };
        let mut result = String::with_capacity(selector_len + rule_body_len + media_wrapper_len);
        if is_hover {
            result.push_str("@media (hover: hover) { ");
        }
        for _ in 0..boost {
            result.push('.');
            result.push_str(class);
        }
        result.push_str(pseudoclass_name);
        let _ = write!(result, " {{ {}: {}; }}", self.name, value);
        if is_hover {
            result.push_str(" }");
        }
        result
    }
}

impl El {
    pub fn pseudo(
        mut self,
        pseudo: Pseudo,
        closure: impl FnOnce(Self) -> Self
    ) -> Self {
        if self.current_pseudo != Pseudo::None {
            report_error(error_messages::NESTED_USING_PSEUDO);
        }
        self.current_pseudo = pseudo;
        self = closure(self);
        self.current_pseudo = Pseudo::None;
        self
    }
}

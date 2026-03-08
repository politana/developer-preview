use crate::{TypedAttributeValue, api::attr_style::{AttributeValue, Style}};

#[derive(Clone, Copy)]
pub enum CssDefaults {
    Initial, Inherit, Unset, Revert, RevertLayer
}

impl <T: Copy> TypedAttributeValue<T> for CssDefaults {
    fn value(self) -> impl AttributeValue {
        CssDefaultsAttribute(self)
    }
}

impl <T: Copy> Style<T> for CssDefaults {
    fn into_function(&self) -> impl Fn() -> String {
        || <CssDefaults as TypedAttributeValue<T>>::value(*self).value().attr_string()
    }
}

#[derive(Clone, Copy)]
struct CssDefaultsAttribute(pub CssDefaults);

impl AttributeValue for CssDefaultsAttribute {
    fn attr_string(self) -> String {
        match self.0 {
            CssDefaults::Initial => "initial",
            CssDefaults::Inherit => "inherit",
            CssDefaults::Unset => "unset",
            CssDefaults::Revert => "revert",
            CssDefaults::RevertLayer => "revert-layer"
        }.to_string()
    }
}

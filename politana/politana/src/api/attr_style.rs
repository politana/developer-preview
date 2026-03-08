use std::rc::Rc;

use crate::{El, api::{computed::Computed, property::{Property, PropertyKey, Pseudo}}};

pub trait StringOrRef {
    fn into_string(self) -> String;
}

impl StringOrRef for String {
    fn into_string(self) -> String { self }
}

impl StringOrRef for &str {
    fn into_string(self) -> String { self.to_string() }
}

pub trait StringProperty {
    fn into_function(&self) -> impl Fn() -> String;
}

impl <F, S> StringProperty for F
where F: Fn() -> S + 'static, S: StringOrRef {
    fn into_function(&self) -> impl Fn() -> String {
        || self().into_string()
    }
}

impl StringProperty for String {
    fn into_function(&self) -> impl Fn() -> String {
        || self.clone()
    }
}

impl StringProperty for &'static str {
    fn into_function(&self) -> impl Fn() -> String {
        || self.to_string()
    }
}

pub trait AttributeValue: Copy {
    fn attr_string(self) -> String;
}

pub trait TypedAttributeValue<T> where T: Copy {
    fn value(self) -> impl AttributeValue;
}

impl <T> TypedAttributeValue<T> for T where T: AttributeValue {
    fn value(self) -> impl AttributeValue { self }
}

pub trait Attribute<T> {
    fn into_function(&self) -> impl Fn() -> String;
}

impl <T, V, F> Attribute<T> for F
where F: Fn() -> V + 'static, V: TypedAttributeValue<T> + Copy, T: Copy {
    fn into_function(&self) -> impl Fn() -> String {
        || self().value().attr_string()
    }
}

pub trait Style<T> {
    fn into_function(&self) -> impl Fn() -> String;
}

impl <T, V, F> Style<T> for F
where F: Fn() -> V + 'static, V: TypedAttributeValue<T> + Copy, T: Copy {
    fn into_function(&self) -> impl Fn() -> String {
        || self().value().attr_string()
    }
}

pub trait BooleanAttribute {
    fn into_function(&self) -> impl Fn() -> bool;
}

impl BooleanAttribute for bool {
    fn into_function(&self) -> impl Fn() -> bool {
        || *self
    }
}

impl <F> BooleanAttribute for F where F: Fn() -> bool {
    fn into_function(&self) -> impl Fn() -> bool {
        self
    }
}

impl El {
    pub fn attr(
        self,
        key: &'static str,
        value: impl StringProperty + 'static
    ) -> Self {
        self.attr_multiple([key], value)
    }

    pub(crate) fn attr_multiple<const SIZE: usize>(
        mut self,
        keys: [&'static str; SIZE],
        value: impl StringProperty + 'static
    ) -> Self {
        let value_function = Rc::new(
            move || value.into_function()().into_string()
        ) as Computed<String>;
        for key in keys {
            let value_function = value_function.clone();
            if key == "class" {
                self = self.class(move || value_function());
            } else {
                self.update(move |d| d.properties.insert(
                    PropertyKey {
                        name: key,
                        pseudo: Pseudo::None
                    },
                    Property::Attribute(value_function.clone())
                ));
            }
        }
        self
    }

    pub fn bool_attr(
        mut self,
        key: &'static str,
        value: impl BooleanAttribute + 'static
    ) -> Self {
        let value_function = Rc::new(move || value.into_function()()) as Computed<bool>;
        self.update(move |d| d.properties.insert(
            PropertyKey {
                name: key,
                pseudo: Pseudo::None
            },
            Property::BooleanAttribute(value_function.clone())
        ));
        self
    }

    pub fn style(
        self,
        key: &'static str,
        value: impl StringProperty + 'static
    ) -> Self {
        self.style_multiple([key], value)
    }

    pub(crate) fn style_multiple<const SIZE: usize>(
        mut self,
        keys: [&'static str; SIZE],
        value: impl StringProperty + 'static
    ) -> Self {
        let value_function = Rc::new(
            move || value.into_function()().into_string()
        ) as Computed<String>;
        for key in keys {
            let value_function = value_function.clone();
            self.update(move |d| d.properties.insert(
                PropertyKey {
                    name: key,
                    pseudo: self.current_pseudo
                },
                Property::Style(value_function.clone())
            ));
        }
        self
    }
}

macro_rules! define_attributes {
    ($( $struct_name:ident => [ $($method_name:ident $css_lit:literal)* ] )*) => {
        $(
            impl Attribute<$struct_name> for $struct_name {
                fn into_function(&self) -> impl Fn() -> String {
                    || self.value().attr_string()
                }
            }
            impl El {
                $(
                    pub fn $method_name(
                        self,
                        value: impl Attribute<$struct_name> + 'static
                    ) -> Self {
                        self.attr($css_lit, move || value.into_function()())
                    }
                )*
            }
        )*
    };
}
pub(crate) use define_attributes;

macro_rules! define_string_attributes {
    ($($method_name:ident $css_lit:literal)*) => {
        $(
            impl El {
                pub fn $method_name(
                    self,
                    value: impl StringProperty + 'static
                ) -> Self {
                    self.attr($css_lit, move || value.into_function()())
                }
            }
        )*
    };
}
pub(crate) use define_string_attributes;

macro_rules! define_boolean_attributes {
    ($($method_name:ident $css_lit:literal)*) => {
        $(
            impl El {
                pub fn $method_name(
                    self,
                    value: impl BooleanAttribute + 'static
                ) -> Self {
                    self.bool_attr($css_lit, move || value.into_function()())
                }
            }
        )*
    };
}
pub(crate) use define_boolean_attributes;

macro_rules! define_css_properties {
    ($( $struct_name:ident => [ $($method_name:ident $css_lit:literal)* ] )*) => {
        $(
            impl Style<$struct_name> for $struct_name {
                fn into_function(&self) -> impl Fn() -> String {
                    || self.value().attr_string()
                }
            }
            impl El {
                $(
                    pub fn $method_name(
                        self,
                        value: impl Style<$struct_name> + 'static
                    ) -> Self {
                        self.style($css_lit, move || value.into_function()())
                    }
                )*
            }
        )*
    };
}
pub(crate) use define_css_properties;

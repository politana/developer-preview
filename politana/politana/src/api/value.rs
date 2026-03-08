use std::rc::Rc;

use crate::{El, api::{attr_style::{StringOrRef, StringProperty}, computed::Computed, property::{Property, PropertyKey, Pseudo}}};

impl El {
    pub fn value(self, value: impl StringProperty + 'static) -> Self {
        let value_function = Rc::new(
            move || value.into_function()().into_string()
        ) as Computed<String>;
        let mut result = self;
        result.update(move |d| d.properties.insert(
            PropertyKey {
                name: "value",
                pseudo: Pseudo::None
            },
            Property::Value(value_function.clone())
        ));
        result
    }
}

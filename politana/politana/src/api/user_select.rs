use crate::{El, UserSelect, api::attr_style::{AttributeValue, Style, TypedAttributeValue}};

impl El {
    pub fn user_select(self, value: impl Style<UserSelect> + 'static) -> Self {
        self.style_multiple(
            ["user-select", "-webkit-user-select"],
            move || value.into_function()()
        )
    }
}

impl Style<UserSelect> for UserSelect {
    fn into_function(&self) -> impl Fn() -> String {
        || self.value().attr_string()
    }
}

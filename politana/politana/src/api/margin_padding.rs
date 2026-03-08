use crate::{El, Length, api::attr_style::Style};

impl El {
    pub fn padding_horizontal(
        self,
        value: impl Style<Length> + 'static
    ) -> Self {
        self.style_multiple(
            ["padding-left", "padding-right"],
            move || value.into_function()()
        )
    }

    pub fn padding_vertical(
        self,
        value: impl Style<Length> + 'static
    ) -> Self {
        self.style_multiple(
            ["padding-top", "padding-bottom"],
            move || value.into_function()()
        )
    }

    pub fn padding(self, value: impl Style<Length> + 'static) -> Self {
        self.style_multiple(
            ["padding-top", "padding-left", "padding-right", "padding-bottom"],
            move || value.into_function()()
        )
    }

    pub fn margin_horizontal(
        self,
        value: impl Style<Length> + 'static
    ) -> Self {
        self.style_multiple(
            ["margin-left", "margin-right"],
            move || value.into_function()()
        )
    }

    pub fn margin_vertical(
        self,
        value: impl Style<Length> + 'static
    ) -> Self {
        self.style_multiple(
            ["margin-top", "margin-bottom"],
            move || value.into_function()()
        )
    }

    pub fn margin(self, value: impl Style<Length> + 'static) -> Self {
        self.style_multiple(
            ["margin-top", "margin-left", "margin-right", "margin-bottom"],
            move || value.into_function()()
        )
    }
}

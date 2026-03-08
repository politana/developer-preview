use crate::{El, Attribute, api::attr_style::AttributeValue};

pub type FontVariation = (&'static str, f64);

fn font_variation_attr_string(v: FontVariation) -> String {
    format!("\"{}\" {}", v.0, v.1)
}

impl <const SIZE: usize> AttributeValue for [FontVariation; SIZE] {
    fn attr_string(self) -> String {
        self.map(|f| font_variation_attr_string(f)).join(", ")
    }
}

impl <const SIZE: usize> Attribute<[FontVariation; SIZE]> for [FontVariation; SIZE] {
    fn into_function(&self) -> impl Fn() -> String {
        || self.attr_string()
    }
}

impl El {
    pub fn font_variation_settings<const SIZE: usize>(
        self,
        value: impl Attribute<[FontVariation; SIZE]> + 'static
    ) -> Self {
        self.style("font-variation-settings", move || value.into_function()())
    }
}

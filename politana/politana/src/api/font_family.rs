use crate::{El, Attribute, api::attr_style::AttributeValue};

#[derive(Clone, Copy)]
pub enum FontFamily {
    Named(&'static str),
    Serif, SansSerif, Monospace, Cursive, SystemUi
}

impl FontFamily {
    fn font_attr_string(self) -> String {
        match self {
            FontFamily::Named(name) => format!("\"{}\"", name),
            FontFamily::Serif => "serif".to_string(),
            FontFamily::SansSerif => "sans-serif".to_string(),
            FontFamily::Monospace => "monospace".to_string(),
            FontFamily::Cursive => "cursive".to_string(),
            FontFamily::SystemUi => "system-ui".to_string()
        }
    }
}

impl <const SIZE: usize> AttributeValue for [FontFamily; SIZE] {
    fn attr_string(self) -> String {
        self.map(|f| f.font_attr_string()).join(", ")
    }
}

impl <const SIZE: usize> Attribute<[FontFamily; SIZE]> for [FontFamily; SIZE] {
    fn into_function(&self) -> impl Fn() -> String {
        || self.attr_string()
    }
}

impl El {
    pub fn font_family<const SIZE: usize>(
        self,
        value: impl Attribute<[FontFamily; SIZE]> + 'static
    ) -> Self {
        self.style("font-family", move || value.into_function()())
    }
}

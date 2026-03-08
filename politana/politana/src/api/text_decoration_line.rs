use crate::{El, api::attr_style::{Attribute, AttributeValue}};

#[derive(Clone, Copy)]
pub enum TextDecorationLine {
    None, Underline, Overline, LineThrough, Blink
}

impl <const SIZE: usize> AttributeValue for [TextDecorationLine; SIZE] {
    fn attr_string(self) -> String {
        self.map(|d| match d {
            TextDecorationLine::None => "none",
            TextDecorationLine::Underline => "underline",
            TextDecorationLine::Overline => "overline",
            TextDecorationLine::LineThrough => "line-through",
            TextDecorationLine::Blink => "blink",
        }).join(" ")
    }
}

impl <const SIZE: usize> Attribute<[TextDecorationLine; SIZE]> for [TextDecorationLine; SIZE] {
    fn into_function(&self) -> impl Fn() -> String {
        || self.attr_string()
    }
}

impl El {
    pub fn text_decoration_line<const SIZE: usize>(
        self,
        value: impl Attribute<[TextDecorationLine; SIZE]> + 'static
    ) -> Self {
        self.style("text-decoration-line", move || value.into_function()())
    }
}

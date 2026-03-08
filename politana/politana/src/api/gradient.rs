use crate::{Angle, Color, El, api::attr_style::{AttributeValue, Style, TypedAttributeValue}};

#[derive(Clone, Copy)]
pub enum Gradient<const SIZE: usize> {
    Linear(Angle, [(Color, f64); SIZE])
}

impl <const SIZE: usize> AttributeValue for Gradient<SIZE> {
    fn attr_string(self) -> String {
        match self {
            Gradient::Linear(angle, color_stops) => {
                let mut result = String::new();
                result.push_str("linear-gradient(");
                result.push_str(&angle.attr_string());
                for (color, position) in color_stops {
                    result.push_str(",");
                    result.push_str(&color.attr_string());
                    result.push_str(" ");
                    result.push_str(&(position * 100.0).to_string());
                    result.push_str("%");
                }
                result.push_str(")");
                result
            }
        }
    }
}

impl <const SIZE: usize> Style<Gradient<SIZE>> for Gradient<SIZE> {
    fn into_function(&self) -> impl Fn() -> String {
        || self.value().attr_string()
    }
}

impl El {
    pub fn background_gradient<const SIZE: usize>(
        self, value: impl Style<Gradient<SIZE>> + 'static
    ) -> Self {
        self.style("background", move || value.into_function()())
    }
}

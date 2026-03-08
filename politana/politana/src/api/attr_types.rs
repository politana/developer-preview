use crate::api::attr_style::AttributeValue;

#[derive(Clone, Copy)]
pub enum ContentEditable {
    False, True, PlaintextOnly
}

impl AttributeValue for ContentEditable {
    fn attr_string(self) -> String {
        match self {
            ContentEditable::False => "false",
            ContentEditable::True => "true",
            ContentEditable::PlaintextOnly => "plaintext-only",
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum InputType {
    Checkbox, Color, Date, DateTimeLocal, Email, File, Hidden, Number,
    Password, Radio, Range, Search, Tel, Text, Time, Url
}

impl AttributeValue for InputType {
    fn attr_string(self) -> String {
        match self {
            InputType::Checkbox => "checkbox",
            InputType::Color => "color",
            InputType::Date => "date",
            InputType::DateTimeLocal => "datetime-local",
            InputType::Email => "email",
            InputType::File => "file",
            InputType::Hidden => "hidden",
            InputType::Number => "number",
            InputType::Password => "password",
            InputType::Radio => "radio",
            InputType::Range => "range",
            InputType::Search => "search",
            InputType::Tel => "tel",
            InputType::Text => "text",
            InputType::Time => "time",
            InputType::Url => "url"
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum Step {
    Interval(f64), Any
}

impl AttributeValue for Step {
    fn attr_string(self) -> String {
        match self {
            Step::Interval(interval) => interval.to_string(),
            Step::Any => "any".to_string(),
        }
    }
}

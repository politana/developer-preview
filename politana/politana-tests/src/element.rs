use crate::test_failure::TestFailure;

pub struct Element {
    html: web_sys::HtmlElement
}

impl Element {
    pub fn new(html: web_sys::HtmlElement) -> Self {
        Self { html }
    }

    pub fn html(&self) -> web_sys::HtmlElement { self.html.clone() }

    #[track_caller]
    pub fn has_text(&self, text: impl Into<String>) -> Result<(), TestFailure> {
        let text = text.into();
        let Some(text_content) = self.html.text_content() else {
            return Err(TestFailure::new(format!("Element does not have any text\nExpected to have \"{}\"", text)))
        };
        if text_content != text {
            return Err(TestFailure::new(format!("Element expected to have text \"{}\"\nActually has text \"{}\"", text, text_content)))
        }
        Ok(())
    }

    #[track_caller]
    pub fn has_style(&self, key: &str, value: &str) -> Result<(), TestFailure> {
        let window = web_sys::window().expect("Document doesn't have a window");
        let computed_style = window.get_computed_style(&self.html)
            .expect("Can't get element computed style")
            .expect("Element doesn't have a computed style");
        let Ok(prop_value) = computed_style.get_property_value(key) else {
            return Err(TestFailure::new(format!("Element does not have a value for style property \"{}\"\nExpected to have value \"{}\"", key, value)))
        };
        if value != prop_value {
            return Err(TestFailure::new(format!("Element expected to have value \"{}\" for style property \"{}\"\nActually has value \"{}\"", value, key, prop_value)))
        }
        Ok(())
    }

    #[track_caller]
    pub fn has_attribute(&self, key: &str, value: &str) -> Result<(), TestFailure> {
        let Some(attr) = self.html.get_attribute(key) else {
            return Err(TestFailure::new(format!("Element does not have an attribute for key \"{}\"", key)))
        };
        if attr != value {
            return Err(TestFailure::new(format!("Element expected to have attribute value \"{}\" for key \"{}\"\nActually has value \"{}\"", value, key, attr)))
        }
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    #[track_caller]
    pub fn has_child_count(&self, count: u32) -> Result<(), TestFailure> {
        let actual_count = self.html.child_nodes().length();
        if actual_count != count {
            return Err(TestFailure::new(format!("Element expected to have {} children, but actually has {}", count, actual_count)))
        }
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[allow(unused_variables)]
    pub fn has_child_count(&self, count: u32) -> Result<(), TestFailure> {
        Ok(())
    }
}

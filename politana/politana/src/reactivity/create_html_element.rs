use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

use crate::utils::unwrap_or_error::UnwrapOrError;

pub trait CreateHtmlElement {
    fn create_html_element(&self, tag: &str) -> HtmlElement;
}

impl CreateHtmlElement for Document {
    fn create_html_element(&self, tag: &str) -> HtmlElement {
        let error = format!("Cannot create an HTML element with tag name \"{}\"", tag);
        self.create_element(tag)
            .ok().unwrap_or_report(&error)
            .dyn_into::<HtmlElement>()
            .ok().unwrap_or_report(&error)
    }
}

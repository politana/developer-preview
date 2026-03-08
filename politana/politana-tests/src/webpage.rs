use web_sys::{Document, HtmlElement, Window, wasm_bindgen::JsCast};

use crate::{element::Element, test_failure::TestFailure};

pub struct Webpage {
    document: Document,
    window: Window
}

impl Webpage {
    pub fn new(document: Document, window: Window) -> Self {
        Self { document, window }
    }

    #[track_caller]
    pub fn element_by_id(&self, id: &'static str) -> Result<Element, TestFailure> {
        let Some(element) = self.document.get_element_by_id(id) else {
            return Err(TestFailure::new(format!("Can't find element with id \"{}\"", id)))
        };
        let Ok(element) = element.dyn_into::<HtmlElement>() else {
            return Err(TestFailure::new(format!("Element with id \"{}\" is not an HtmlElement", id)))
        };
        Ok(Element::new(element))
    }

    pub fn window(&self) -> Window { self.window.clone() }
}

use web_sys::{Event, HtmlElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, wasm_bindgen::JsCast};

use crate::utils::{error_messages, report_error::report_error};

pub trait TypedEventTargets {
    fn input_target(&self) -> HtmlInputElement;
    fn select_target(&self) -> HtmlSelectElement;
    fn textarea_target(&self) -> HtmlTextAreaElement;
    fn opaque_target_element(&self) -> HtmlElement;
}

impl TypedEventTargets for Event {
    #[track_caller]
    fn input_target(&self) -> HtmlInputElement {
        let Some(target) = self.target() else {
            report_error(error_messages::NOT_INPUT_ELEMENT);
        };
        let Ok(element) = target.dyn_into::<HtmlInputElement>() else {
            report_error(error_messages::NOT_INPUT_ELEMENT);
        };
        element
    }

    #[track_caller]
    fn select_target(&self) -> HtmlSelectElement {
        let Some(target) = self.target() else {
            report_error(error_messages::NOT_SELECT_ELEMENT);
        };
        let Ok(element) = target.dyn_into::<HtmlSelectElement>() else {
            report_error(error_messages::NOT_SELECT_ELEMENT);
        };
        element
    }

    #[track_caller]
    fn textarea_target(&self) -> HtmlTextAreaElement {
        let Some(target) = self.target() else {
            report_error(error_messages::NOT_TEXT_AREA_ELEMENT);
        };
        let Ok(element) = target.dyn_into::<HtmlTextAreaElement>() else {
            report_error(error_messages::NOT_TEXT_AREA_ELEMENT);
        };
        element
    }

    #[track_caller]
    fn opaque_target_element(&self) -> HtmlElement {
        let Some(target) = self.target() else {
            report_error(error_messages::NOT_HTML_ELEMENT);
        };
        let Ok(element) = target.dyn_into::<HtmlElement>() else {
            report_error(error_messages::NOT_HTML_ELEMENT);
        };
        element
    }
}

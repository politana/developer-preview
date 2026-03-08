#![allow(non_snake_case)]

use std::rc::Rc;
use crate::api::{attr_style::{Attribute, AttributeValue, BooleanAttribute, StringProperty, Style, TypedAttributeValue, define_attributes, define_boolean_attributes, define_css_properties, define_string_attributes}, el::EventListener, element_children::ElementChildren, elements::{define_elements, define_void_elements}, event_listeners::define_event_listeners};
use web_sys::{Event, PointerEvent, wasm_bindgen::JsCast};

mod api;
mod debug_bar;
pub mod library;
mod reactivity;
mod utils;

#[cfg(not(target_arch = "wasm32"))]
mod package;

pub use api::attr_types::{ContentEditable, InputType, Step};
pub use api::css_defaults::CssDefaults;
pub use api::css_types::{AlignItems, Angle, AngleUnit, BorderStyle, BoxSizing, Color, Cursor, Display, FlexDirection, FontOpticalSizing, FontStyle, IntoAngle, IntoLength, JustifyContent, Length, LengthUnit, Overflow, Position, TextAlign, TextDecorationStyle, UserSelect};
pub use api::el::{El, InnerHtml};
pub use api::font_family::FontFamily;
pub use api::font_variation::FontVariation;
pub use api::for_each::{ForEach, ForEachKeyed};
pub use api::gradient::Gradient;
pub use api::property::Pseudo;
pub use api::politana::{Politana, RunOptions};
pub use api::text_decoration_line::TextDecorationLine;
pub use api::typed_event_targets::TypedEventTargets;
pub use api::unique::UniqueId;
pub use politana_view_macro::View;
pub use reactivity::{closure::Closure, environment::Environment};

// Some APIs must be swapped out for compile-time reflection

#[cfg(target_arch = "wasm32")]
pub use reactivity::state::State;

#[cfg(not(target_arch = "wasm32"))]
pub use package::api::state::State;

// Expose private APIs for internal testing

#[cfg(feature = "internal-testing")]
pub use utils::{error_messages, is_test_suite, last_error};

define_elements!(
    A "a"
    Button "button"
    Div "div"
    Em "em"
    H1 "h1"
    H2 "h2"
    Label "label"
    Option "option"
    P "p"
    Pre "pre"
    Select "select"
    Span "span"
    Strong "strong"
);

define_void_elements!(
    Br "br"
    Hr "hr"
    Img "img"
    Input "input"
    Textarea "textarea"
);

define_attributes!(
    f64 => []
    ContentEditable => [content_editable "contenteditable"]
    InputType => [input_type "type"]
    Step => []
);

define_string_attributes!(
    alt "alt"
    href "href"
    id "id"
    input_name "name"
    label_for "for"
    src "src"
);

define_boolean_attributes!(
    checked "checked"
    disabled "disabled"
    hidden "hidden"
    selected "selected"
);

define_css_properties!(
    f64 => [
        flex_grow "flex-grow"
        flex_shrink "flex-shrink"
        font_weight "font-weight"
        line_height "line-height"
        opacity "opacity"
    ]
    AlignItems => [align_items "align-items"]
    BorderStyle => [
        border_bottom_style "border-bottom-style"
        border_left_style "border-left-style"
        border_right_style "border-right-style"
        border_style "border-style"
        border_top_style "border-top-style"
    ]
    BoxSizing => [box_sizing "box-sizing"]
    Color => [
        background_color "background-color"
        border_bottom_color "border-bottom-color"
        border_color "border-color"
        border_left_color "border-left-color"
        border_right_color "border-right-color"
        border_top_color "border-top-color"
        color "color"
        text_decoration_color "text-decoration-color"
    ]
    Cursor => [cursor "cursor"]
    Display => [display "display"]
    FlexDirection => [flex_direction "flex-direction"]
    FontOpticalSizing => [font_optical_sizing "font-optical-sizing"]
    FontStyle => [font_style "font-style"]
    JustifyContent => [justify_content "justify-content"]
    Length => [
        border_bottom_width "border-bottom-width"
        border_left_width "border-left-width"
        border_radius "border-radius"
        border_right_width "border-right-width"
        border_top_width "border-top-width"
        border_width "border-width"
        bottom "bottom"
        flex_basis "flex-basis"
        font_size "font-size"
        gap "gap"
        height "height"
        left "left"
        margin_bottom "margin-bottom"
        margin_left "margin-left"
        margin_right "margin-right"
        margin_top "margin-top"
        max_height "max-height"
        max_width "max-width"
        min_height "min-height"
        min_width "min-width"
        padding_bottom "padding-bottom"
        padding_left "padding-left"
        padding_right "padding-right"
        padding_top "padding-top"
        right "right"
        top "top"
        width "width"
    ]
    Overflow => [
        overflow_x "overflow-x"
        overflow_y "overflow-y"
    ]
    Position => [
        position "position"
    ]
    TextAlign => [text_align "text-align"]
    TextDecorationStyle => [text_decoration_style "text-decoration-style"]
);

define_event_listeners!(
    on_change "change" Event
    on_click "click" PointerEvent
    on_input "input" Event
);

use politana_view_macro::View;

use crate::{BorderStyle, Display, Div, El, FlexDirection, IntoLength, debug_bar::ui::debug_bar::DebugBar, library::{NavigationHost, Routes}};

#[View]
pub fn AddDebugBar(app: fn() -> El) -> El {
    NavigationHost(
        Routes::new()
            .route("debug", |_| app()),
        |_| Div((
            || DebugBar(),
            || El("iframe", ())
                .src("./debug")
                .width(100.vw())
                .flex_grow(1.0)
                .border_style(BorderStyle::None)
        ))
            .height(100.vh())
            .display(Display::Flex)
            .flex_direction(FlexDirection::Column)
            .global_css("
                body {
                    padding: 0;
                    margin: 0;
                }
            ")
    )
}

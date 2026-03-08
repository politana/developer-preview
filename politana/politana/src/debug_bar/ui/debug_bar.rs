use politana_view_macro::View;

use crate::{Color, Display, Div, El, FlexDirection, FontFamily, IntoLength, P, Span};

#[View]
pub fn DebugBar() -> El {
    Div((
        || P((
            || Span("Politana")
                .font_weight(250.0),
            || Span(" Debug Bar")
                .font_weight(375.0)
        ))
            .margin(0.px())
            .margin_left(16.px()),
        || P("(hidden in release builds)")
            .margin(0.px())
            .margin_left(10.px())
            .font_weight(300.0)
            .opacity(0.7)
    ))
        .display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .flex_shrink(0.0)
        .margin(0.px())
        .padding_vertical(12.px())
        .background_color(Color::Rgba(186.0, 100.0, 111.0, 1.0))
        .color(Color::White)
        .use_resource(r#"
            <link rel="preconnect" href="https://fonts.googleapis.com">
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
            <link href="https://fonts.googleapis.com/css2?family=Google+Sans+Flex:opsz,wght@6..144,1..1000&display=swap" rel="stylesheet">
        "#)
        .font_family([FontFamily::Named("Google Sans Flex")])
}

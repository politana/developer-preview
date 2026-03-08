use politana::{AlignItems, BoxSizing, Closure, Color, Display, Div, El, IntoLength, JustifyContent, P, Pseudo, View};

#[View]
pub fn NewNoteButton(on_click: Closure<(), ()>) -> El {
    Div(|| P("New note"))
        .box_sizing(BoxSizing::BorderBox)
        .width(100.percent())
        .height(70.px())
        .padding_horizontal(24.px())
        .display(Display::Flex)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::FlexStart)
        .on_click(|_| on_click.call(()))
        .background_color(Color::Hsla(50.0, 1.0, 0.8, 1.0))
        .pseudo(Pseudo::Hover, |e| e
            .background_color(Color::Hsla(50.0, 0.7, 0.7, 1.0))
        )
        .pseudo(Pseudo::Active, |e| e
            .background_color(Color::Hsla(50.0, 1.0, 0.7, 1.0))
        )
}

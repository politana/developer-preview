use politana::{AlignItems, BoxSizing, Closure, Color, Display, Div, El, FlexDirection, IntoLength, JustifyContent, P, Pseudo, View};

use crate::note::Note;

#[View]
pub fn NotePreview(note: Note, on_click: Closure<(), ()>) -> El {
    Div((
        || P(|| note.title())
            .margin(0.px()),
        || P(|| note.date_string())
            .margin(0.px())
    ))
        .box_sizing(BoxSizing::BorderBox)
        .width(100.percent())
        .height(70.px())
        .padding_horizontal(24.px())
        .display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::FlexStart)
        .on_click(|_| on_click.call(()))
        .pseudo(Pseudo::Hover, |e| e
            .background_color(Color::Hsla(50.0, 0.7, 0.7, 1.0))
        )
        .pseudo(Pseudo::Active, |e| e
            .background_color(Color::Hsla(50.0, 1.0, 0.7, 1.0))
        )
}

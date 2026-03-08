use politana::{AlignItems, Button, Closure, Color, Display, Div, El, IntoLength, View};

use crate::note::Paragraph;

#[View]
fn Decoration() -> El {
    Div(())
        .height(1.px())
        .flex_grow(1.0)
        .background_color(Color::Hsla(0.0, 0.0, 0.0, 0.4))
}

#[View]
pub fn AddParagraph(on_add_paragraph: Closure<Paragraph, ()>) -> El {
    Div((
        || Decoration(),
        || Button("H1")
            .on_click(|_| on_add_paragraph.call(Paragraph::H1("Header 1".to_string()))),
        || Button("H2")
            .on_click(|_| on_add_paragraph.call(Paragraph::H2("Header 2".to_string()))),
        || Button("Body")
            .on_click(|_| on_add_paragraph.call(Paragraph::Body("Body".to_string()))),
        || Button("Line")
            .on_click(|_| on_add_paragraph.call(Paragraph::HorizontalLine)),
        || Decoration()
    ))
        .display(Display::Flex)
        .align_items(AlignItems::Center)
        .gap(8.px())
}

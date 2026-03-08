use politana::{Closure, Color, ContentEditable, Div, El, H1, H2, IntoLength, P, State, TypedEventTargets, View};

use crate::note::Paragraph;

#[View]
pub fn ParagraphView(
    paragraph: State<Paragraph>,
    on_change: Closure<(), ()>
) -> El {
    match paragraph.get_once() {
        Paragraph::H1(str) => H1(|| str.clone())
            .content_editable(ContentEditable::PlaintextOnly)
            .on_input(|event| paragraph.set(
                |_| Paragraph::H1(event.opaque_target_element().text_content().unwrap())
            ))
            .on_input(|_| on_change.call(())),
        Paragraph::H2(str) => H2(|| str.clone())
            .content_editable(ContentEditable::PlaintextOnly)
            .on_input(|event| paragraph.set(
                |_| Paragraph::H2(event.opaque_target_element().text_content().unwrap())
            ))
            .on_input(|_| on_change.call(())),
        Paragraph::Body(str) => P(|| str.clone())
            .content_editable(ContentEditable::PlaintextOnly)
            .on_input(|event| paragraph.set(
                |_| Paragraph::Body(event.opaque_target_element().text_content().unwrap())
            ))
            .on_input(|_| on_change.call(())),
        Paragraph::HorizontalLine => Div(())
            .width(100.percent())
            .height(2.px())
            .background_color(Color::Hsla(0.0, 0.0, 0.0, 0.2))
    }
}

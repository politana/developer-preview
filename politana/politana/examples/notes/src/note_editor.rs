use politana::{Button, Closure, Div, El, ForEach, IntoLength, View};

use crate::{add_paragraph::AddParagraph, note::Note, paragraph_view::ParagraphView};

#[View]
pub fn NoteEditor(
    note: Note,
    on_delete: Closure<(), ()>
) -> El {
    Div((
        || Div(ForEach(
            || note.paragraphs(),
            |paragraph| ParagraphView(
                paragraph,
                Closure::new(|_| note.update_date_modified())
            )
        )),
        || AddParagraph(Closure::new(|p| note.add_paragraph(p))),
        || Button("Delete")
            .on_click(|_| on_delete.call(()))
    ))
        .padding(16.px())
}

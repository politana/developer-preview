use politana::{Closure, Color, Div, El, ForEach, State, View};

use crate::{new_note_button::NewNoteButton, note::Note, note_preview::NotePreview};

#[View]
pub fn Sidebar(
    notes: State<Vec<Note>>,
    on_select_note: Closure<Note, ()>
) -> El {
    Div((
        || NewNoteButton(
            Closure::new(|_| {
                notes.update(|n| n.push(Note::default()));
            })
        ),
        || Div(ForEach(
            || {
                let mut notes = notes.get();
                notes.sort_by(|a, b| b.date_modified().cmp(&a.date_modified()));
                notes
            },
            |note| NotePreview(note, Closure::new(|_| on_select_note.call(note)))
        )),
    ))
        .background_color(Color::Hsla(50.0, 1.0, 0.9, 1.0))
}

use politana::{Closure, Display, Div, El, IntoLength, Overflow, State, View};

use crate::{note::Note, note_editor::NoteEditor, sidebar::Sidebar};

#[View]
pub fn App() -> El {
    let notes: State<Vec<Note>> = State::default();
    let editing_note: State<Option<Note>> = State::default();
    Div((
        || Sidebar(
            notes,
            Closure::new(|note| editing_note.put(Some(note)))
        )
            .height(100.vh())
            .width(250.px())
            .overflow_y(Overflow::Scroll),
        || if let Some(note) = editing_note.get() {
            NoteEditor(
                note,
                Closure::new(|_| {
                    editing_note.put(None);
                    notes.update(|n| n.retain(|n| *n != note))
                })
            )
        } else {
            Div(())
        }
            .flex_grow(1.0)
    ))
        .display(Display::Flex)
        .global_css("
            body {
                padding: 0;
                margin: 0;
            }
        ")
}

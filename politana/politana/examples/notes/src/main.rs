use politana::Politana;

use crate::app::App;

mod add_paragraph;
mod app;
mod new_note_button;
mod note;
mod note_editor;
mod note_preview;
mod paragraph_view;
mod sidebar;

fn main() {
    Politana::launch(App);
}

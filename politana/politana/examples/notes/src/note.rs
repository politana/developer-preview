use politana::{State, UniqueId};
use time::{OffsetDateTime, macros::format_description};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct NoteId(pub UniqueId);

#[derive(Clone)]
pub enum Paragraph {
    H1(String),
    H2(String),
    Body(String),
    HorizontalLine
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Note {
    id: NoteId,
    date_modified: State<OffsetDateTime>,
    paragraphs: State<Vec<State<Paragraph>>>
}

impl Default for Note {
    fn default() -> Self {
        Self {
            id: NoteId(UniqueId::new()),
            date_modified: State::new(OffsetDateTime::now_local().unwrap()),
            paragraphs: State::default()
        }
    }
}

impl Note {
    pub fn title(&self) -> String {
        self.paragraphs.get().iter()
            .filter_map(|p| match p.get() {
                Paragraph::H1(str) => Some(str),
                Paragraph::H2(str) => Some(str),
                Paragraph::Body(str) => Some(str),
                _ => None
            })
            .next()
            .unwrap_or("Empty note".to_string())
    }

    pub fn date_string(&self) -> String {
        let format = format_description!("[hour]:[minute]:[second]");
        self.date_modified.map(|d| d.format(format).unwrap())
    }

    pub fn date_modified(&self) -> OffsetDateTime { self.date_modified.get() }

    pub fn add_paragraph(&self, paragraph: Paragraph) {
        self.paragraphs.update(|p| p.push(State::new(paragraph)));
        self.update_date_modified();
    }

    pub fn paragraphs(&self) -> Vec<State<Paragraph>> { self.paragraphs.get() }

    pub fn update_date_modified(&self) {
        self.date_modified.put(OffsetDateTime::now_local().unwrap())
    }
}

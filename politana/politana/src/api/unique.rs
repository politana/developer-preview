use std::fmt::Display;

use crate::{api::attr_style::StringProperty, utils::unique_counter::UniqueCounter};

static COUNTER: UniqueCounter = UniqueCounter::new_starting_at(100000);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct UniqueId {
    id: u64
}

impl UniqueId {
    pub fn new() -> Self {
        UniqueId { id: COUNTER.next_id() }
    }
}

impl StringProperty for UniqueId {
    fn into_function(&self) -> impl Fn() -> String {
        || format!("politana-unique-{}", self.id)
    }
}

impl Display for UniqueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.into_function()())
    }
}

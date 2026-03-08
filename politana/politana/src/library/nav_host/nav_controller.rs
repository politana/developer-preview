use crate::{Closure, State};

#[derive(Clone, Copy)]
pub struct NavController {
    pub(crate) navigate: Closure<String, ()>,
    pub(crate) go_back: Closure<(), ()>,
    pub(crate) wildcards: State<Vec<String>>
}

impl NavController {
    pub fn navigate(&self, destination: impl Into<String>) {
        self.navigate.call(destination.into());
    }

    pub fn go_back(&self) {
        self.go_back.call(());
    }

    pub fn wildcards(&self) -> Vec<String> {
        self.wildcards.get_once()
    }
}

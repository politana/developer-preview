use std::rc::Rc;

use crate::{El, api::computed::Computed};

trait ClassesData {
    fn data(self) -> Vec<String>;
}

impl ClassesData for &str {
    fn data(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl ClassesData for String {
    fn data(self) -> Vec<String> {
        vec![self]
    }
}

impl <const SIZE: usize> ClassesData for [&str; SIZE] {
    fn data(self) -> Vec<String> {
        self.map(|s| s.to_string()).to_vec()
    }
}

impl ClassesData for Vec<String> {
    fn data(self) -> Vec<String> {
        self
    }
}

pub trait Classes {
    fn classes(self) -> Computed<Vec<String>>;
}

impl Classes for &'static str {
    fn classes(self) -> Computed<Vec<String>> {
        Rc::new(|| self.data())
    }
}

impl Classes for String {
    fn classes(self) -> Computed<Vec<String>> {
        Rc::new(move || self.clone().data())
    }
}

impl <const SIZE: usize> Classes for [&'static str; SIZE] {
    fn classes(self) -> Computed<Vec<String>> {
        Rc::new(move || self.clone().data())
    }
}

impl Classes for Vec<String> {
    fn classes(self) -> Computed<Vec<String>> {
        Rc::new(move || self.clone())
    }
}

impl <F, T> Classes for F where F: Fn() -> T + 'static, T: ClassesData {
    fn classes(self) -> Computed<Vec<String>> {
        Rc::new(move || self().data())
    }
}

impl El {
    pub fn class(mut self, classes: impl Classes + 'static) -> Self {
        let classes = classes.classes();
        self.update(move |d| d.classes.push(classes.clone()));
        self
    }
}

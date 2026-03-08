use std::rc::Rc;

use web_sys::HtmlElement;

use crate::El;

impl El {
    pub fn on_appear(mut self, closure: impl FnOnce(HtmlElement) + 'static) -> El {
        self.lifecycle.on_appear.push(Some(Box::new(closure)));
        self
    }

    pub fn on_disappear(mut self, closure: impl FnOnce(HtmlElement) + 'static) -> El {
        self.lifecycle.on_disappear.push(Some(Box::new(closure)));
        self
    }

    pub fn effect(self, closure: impl Fn() + 'static) -> El {
        self.debounced_effect(move || {
            closure();
            true
        })
    }

    /// Your closure should return `true` to update the list of states that the effect depends on, `false` to skip the update
    pub(crate) fn debounced_effect(mut self, closure: impl Fn() -> bool + 'static) -> El {
        let closure = Rc::new(closure);
        self.update(move |d| d.effects.push(closure.clone()));
        self
    }
}

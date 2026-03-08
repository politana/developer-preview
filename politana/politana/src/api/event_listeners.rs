use std::rc::Rc;

use web_sys::Event;

use crate::{El, api::el::EventListener};

impl El {
    pub fn event_listener(
        mut self,
        event: &'static str,
        listener: impl Fn(Event) + 'static
    ) -> Self {
        let listener = Rc::new(listener);
        self.update(move |d| d.event_listeners.push(EventListener {
            event_type: event,
            listener: listener.clone()
        }));
        self
    }
}

macro_rules! define_event_listeners {
    ($($name:ident $event_str:literal $event_type:ty)*) => {
        impl El {
            $(
                pub fn $name(self, listener: impl Fn($event_type) + 'static) -> Self {
                    let mut result = self;
                    let f = move |event: Event| {
                        // Cast the generic web_sys::Event into the specific type
                        // UNEXPECTED: The "event_type" provided in the macro usage is known to be compatible with the "event_str".
                        use crate::utils::unwrap_or_error::UnwrapOrError;
                        listener(event.dyn_into::<$event_type>().ok().unwrap_or_unexpected())
                    };
                    let listener = Rc::new(f);
                    result.update(move |d| d.event_listeners.push(EventListener {
                        event_type: $event_str,
                        listener: listener.clone()
                    }));
                    result
                }
            )*
        }
    };
}
pub(crate) use define_event_listeners;

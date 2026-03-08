use crate::{El, reactivity::head_resources::HeadResources};

impl El {
    pub fn use_resource(self, html: &'static str) -> Self {
        self.on_appear(move |_| {
            HeadResources::use_resource(html);
        })
    }
}

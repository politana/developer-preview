use crate::{El, reactivity::style_manager::StyleManager};

impl El {
    #[cfg(target_arch = "wasm32")]
    pub fn global_css(self, css: &'static str) -> Self {
        self.on_appear(move |_| StyleManager::use_global_style(css))
            .on_disappear(move |_| StyleManager::release_global_style(css))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn global_css(mut self, css: &'static str) -> Self {
        // Suppress warnings
        let _ = (StyleManager::use_global_style, StyleManager::release_global_style);
        self.global_css.push(css);
        self
    }
}

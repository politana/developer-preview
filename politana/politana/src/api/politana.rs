use crate::{El, reactivity::render::render};

pub struct Politana;

impl Politana {
    pub fn launch(app: fn() -> El) {
        Self::launch_with_options(app, RunOptions::default());
    }

    #[cfg(target_arch = "wasm32")]
    pub fn launch_with_options(app: fn() -> El, options: RunOptions) {
        use std::sync::atomic::Ordering;
        use crate::api::suppress_error_alerts::SUPPRESS_ERROR_ALERTS;
        console_error_panic_hook::set_once();
        SUPPRESS_ERROR_ALERTS.store(options.suppress_error_alerts, Ordering::SeqCst);
        render(app);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn launch_with_options(app: fn() -> El, options: RunOptions) {
        use crate::package::package::package;
        // Suppress warnings
        {
            let _ = render;
            let _ = options.suppress_error_alerts;
        }
        package(app);
    }
}

pub struct RunOptions {
    suppress_error_alerts: bool
}

impl Default for RunOptions {
    fn default() -> Self {
        Self { suppress_error_alerts: false }
    }
}

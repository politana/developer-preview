use std::{panic::Location, sync::atomic::Ordering};

use crate::{api::suppress_error_alerts::SUPPRESS_ERROR_ALERTS, reactivity::deconstruct::deconstruct};

#[track_caller]
#[cfg(target_arch = "wasm32")]
pub fn report_warning(warning: &str) {
    let location = Location::caller();
    let message = format!(
        "Politana warning in file: {}\nLine: {}    Column: {}\n{}",
        location.file(),
        location.line(),
        location.column(),
        warning
    );
    if !SUPPRESS_ERROR_ALERTS.load(Ordering::SeqCst) {
        #[cfg(not(feature = "internal-testing"))]
        web_sys::window().unwrap().alert_with_message(&message).unwrap();
    }
    web_sys::console::warn_1(&message.into());
}

#[track_caller]
pub fn report_error(error: &str) -> ! {
    let location = Location::caller();
    let message = format!(
        "Politana error in file: {}\nLine: {}    Column: {}\n{}",
        location.file(),
        location.line(),
        location.column(),
        error
    );
    #[cfg(feature = "internal-testing")] {
        crate::utils::last_error::LAST_ERROR.replace(Some(error.to_string()));
    }
    if !SUPPRESS_ERROR_ALERTS.load(Ordering::SeqCst) {
        #[cfg(not(feature = "internal-testing"))]
        web_sys::window().unwrap().alert_with_message(&message).unwrap();
    }
    web_sys::console::error_1(&message.into());
    deconstruct();
    panic!();
}

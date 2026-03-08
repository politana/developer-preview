use crate::utils::{error_messages, report_error::{report_error}};

pub trait UnwrapOrError {
    type T;
    fn unwrap_or_unexpected(self) -> Self::T;
    fn unwrap_or_report(self, error: &str) -> Self::T;
    #[cfg(target_arch = "wasm32")]
    fn warn_if_none(self, warning: &str) -> Self;
}

impl <T> UnwrapOrError for Option<T> {
    type T = T;

    #[track_caller]
    fn unwrap_or_unexpected(self) -> T {
        if let Some(result) = self {
            result
        } else {
            report_error(error_messages::UNEXPECTED)
        }
    }

    #[track_caller]
    fn unwrap_or_report(self, error: &str) -> Self::T {
        if let Some(result) = self {
            result
        } else {
            report_error(error)
        }
    }

    #[cfg(target_arch = "wasm32")]
    #[track_caller]
    fn warn_if_none(self, warning: &str) -> Self {
        use crate::utils::report_error::report_warning;
        if self.is_none() {
            report_warning(warning);
        }
        self
    }
}

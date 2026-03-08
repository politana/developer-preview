use politana::last_error::LAST_ERROR;

use crate::{js_try::js_try, test_failure::TestFailure};

#[track_caller]
pub fn expect_framework_error(
    message: &str,
    closure: impl FnOnce()
) -> Result<(), TestFailure> {
    let js_error = js_try(closure);
    if let Some(actual_error) = LAST_ERROR.with_borrow(|s| s.clone()) {
        LAST_ERROR.replace(None);
        if actual_error == message {
            Ok(())
        } else {
            Err(TestFailure::new(format!("The framework error thrown was not what was expected.\nExpected:\n{}\nThrown:\n{}", message, actual_error)))
        }
    } else {
        if js_error.is_some() {
            Err(TestFailure::new("Something panicked, but it wasn't a reported framework error."))
        } else {
            Err(TestFailure::new("Was expecting a framework error, but none was triggered"))
        }
    }
}

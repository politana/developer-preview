use std::panic::Location;

pub struct TestFailure {
    pub message: String,
    pub location: Location<'static>
}

impl TestFailure {
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            location: *Location::caller()
        }
    }
}

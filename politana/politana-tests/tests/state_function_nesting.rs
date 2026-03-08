use politana::{Button, El, State, View, error_messages};
use politana_tests::{expect_framework_error::expect_framework_error, politana_test, test::Test};

#[View]
fn TestView() -> El {
    let counter = State::new(0);
    Button("Trigger error")
        .id("trigger")
        .on_click(|_| {
            // This correct-looking code would, with the current API,
            // allow violating Rust's borrow-checking guarantees.
            // (Consider the case of a more complex type than i32.)
            counter.set(|_| counter.get() + 1);
        })
}

politana_test!(Test {
    name: "State function nesting",
    view: TestView,
    test: async |webpage| {
        let trigger = webpage.element_by_id("trigger")?;
        expect_framework_error(error_messages::SAME_STATE_FUNCTION_NESTING, || {
            trigger.html().click();
        })?;
        Ok(())
    }
});

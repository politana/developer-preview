use politana::{Button, Closure, Div, El, State, View, error_messages};
use politana_tests::{expect_framework_error::expect_framework_error, politana_test, test::Test};

#[View]
fn TestView() -> El {
    Div((
        || Button("Bad state")
            .id("bad-state")
            .on_click(|_| {
                let _bad_state = State::new(1);
            }),
        || Button("Bad closure")
            .id("bad-closure")
            .on_click(|_| {
                let _bad_closure = Closure::new(|_: ()| {});
            }),
    ))
}

politana_test!(Test {
    name: "Basic framework errors",
    view: TestView,
    test: async |webpage| {
        let bad_state = webpage.element_by_id("bad-state")?;
        let bad_closure = webpage.element_by_id("bad-closure")?;
        expect_framework_error(error_messages::MISPLACED_STATE_NEW, || {
            bad_state.html().click();
        })?;
        // Shouldn't panic because future event listeners are disabled
        bad_closure.html().click();
        Ok(())
    }
});

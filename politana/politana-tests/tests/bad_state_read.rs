use politana::{Button, Div, El, P, State, View, error_messages};
use politana_tests::{expect_framework_error::expect_framework_error, politana_test, test::Test};

#[View]
fn BadStateRead() -> El {
    let is_showing = State::new(false);
    if is_showing.get() {
        P("Hello world!")
    } else { Div(()) }
}

#[View]
fn TestView() -> El {
    let show_bad_state_read = State::new(false);
    Div((
        || Button("Show bad state read")
            .id("show-bad-state-read")
            .on_click(|_| {
                show_bad_state_read.put(true);
            }),
        || if show_bad_state_read.get() {
            BadStateRead()
        } else { Div(()) }
    ))
}

politana_test!(Test {
    name: "Basic framework errors",
    view: TestView,
    test: async |webpage| {
        let show_bad_state_read = webpage.element_by_id("show-bad-state-read")?;
        expect_framework_error(error_messages::STATE_READ_ABOVE_CREATION, || {
            show_bad_state_read.html().click();
        })?;
        Ok(())
    }
});

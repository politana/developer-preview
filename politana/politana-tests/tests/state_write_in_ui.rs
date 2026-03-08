use politana::{Button, Div, El, State, View, error_messages};
use politana_tests::{expect_framework_error::expect_framework_error, politana_test, test::Test};

#[View]
fn TestView() -> El {
    let is_bad_write_showing = State::new(false);
    Div((
        || Button("Show bad write")
            .id("show")
            .on_click(|_| is_bad_write_showing.put(true)),
        || if is_bad_write_showing.get() {
            let counter = State::new(0);
            counter.put(1);
            Div(())
        } else {
            Div(())
        }
    ))
}

politana_test!(Test {
    name: "State write in UI",
    view: TestView,
    test: async |webpage| {
        let show = webpage.element_by_id("show")?;
        expect_framework_error(error_messages::STATE_WRITE_IN_UI, || {
            show.html().click();
        })?;
        Ok(())
    }
});

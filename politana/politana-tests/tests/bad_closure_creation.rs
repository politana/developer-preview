use politana::{Button, Closure, El, View, error_messages};
use politana_tests::{expect_framework_error::expect_framework_error, politana_test, test::Test};

#[View]
fn TestView() -> El {
    Button("Bad closure")
        .id("bad-closure")
        .on_click(|_| {
            let _bad_closure = Closure::new(|_: ()| {});
        })
}

politana_test!(Test {
    name: "Basic framework errors",
    view: TestView,
    test: async |webpage| {
        let bad_closure = webpage.element_by_id("bad-closure")?;
        expect_framework_error(error_messages::MISPLACED_STATE_NEW, || {
            bad_closure.html().click();
        })?;
        Ok(())
    }
});

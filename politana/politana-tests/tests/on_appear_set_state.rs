use politana::{Div, El, P, State, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn TestView() -> El {
    let is_showing_name = State::new(false);
    Div(|| if is_showing_name.get() {
        P("Roy")
    } else {
        P("Loading name…")
            .on_appear(|_| is_showing_name.put(true))
    })
}

politana_test!(Test {
    name: "On appear set state panic",
    view: TestView,
    // should simply not panic
    test: async |_| Ok(())
});

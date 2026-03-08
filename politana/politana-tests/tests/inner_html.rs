use politana::{Button, Div, El, InnerHtml, P, State, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn TestView() -> El {
    let counter = State::new(0);
    Div((
        || P(|| InnerHtml(format!("<strong>{}</strong>", counter.get())))
            .id("counter"),
        || Button("Increment")
            .on_click(|_| counter.set(|c| c + 1))
            .id("increment")
    ))
}

politana_test!(Test {
    name: "Inner HTML",
    view: TestView,
    test: async |webpage| {
        let counter = webpage.element_by_id("counter")?;
        let increment = webpage.element_by_id("increment")?;
        counter.has_text("0")?;
        increment.html().click();
        counter.has_text("1")?;
        Ok(())
    }
});

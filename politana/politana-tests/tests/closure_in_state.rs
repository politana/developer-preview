use politana::{Button, Closure, Div, El, P, State, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn TestView() -> El {
    let action = State::new(Closure::new(|x: i32| x + 1));
    Div((
        || P(|| action.get().call(3).to_string())
            .id("result"),
        || Button("Update action")
            .id("update")
            .on_click(|_| action.set(|_| Closure::new(|x| x * x)))
    ))
}

politana_test!(Test {
    name: "Closure in State",
    view: TestView,
    test: async |webpage| {
        let result = webpage.element_by_id("result")?;
        let update = webpage.element_by_id("update")?;
        result.has_text("4")?;
        update.html().click();
        result.has_text("9")?;
        Ok(())
    }
});

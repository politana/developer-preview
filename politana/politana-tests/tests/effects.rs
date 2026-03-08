use politana::{Button, Div, El, P, State, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn TestView() -> El {
    let tracker_twice = State::new(0_i32);
    let tracker_square = State::new(0);
    let tracked = State::new(1);
    Div((
        || P("Effect launcher")
            .effect(|| tracker_twice.put(tracked.get() * 2))
            .effect(|| tracker_square.put(tracked.get().pow(2))),
        || P(|| tracker_twice.get().to_string()).id("twice"),
        || P(|| tracker_square.get().to_string()).id("square"),
        || Button("Increment tracked")
            .on_click(|_| tracked.set(|t| t + 1))
            .id("increment")
    ))
}

politana_test!(Test {
    name: "Effects",
    view: TestView,
    test: async |webpage| {
        let twice = webpage.element_by_id("twice")?;
        let square = webpage.element_by_id("square")?;
        let increment = webpage.element_by_id("increment")?;
        twice.has_text("2")?;
        square.has_text("1")?;
        increment.html().click();
        twice.has_text("4")?;
        square.has_text("4")?;
        Ok(())
    }
});

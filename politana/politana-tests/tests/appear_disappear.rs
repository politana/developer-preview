use politana::{Button, Div, El, P, State, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn TestView() -> El {
    let state_one = State::new(0);
    let state_two = State::new(0);
    let state_three = State::new(0);
    let wrong_order = State::new(false);
    let is_mutator_showing = State::new(false);
    Div((
        || P(|| state_one.get().to_string())
            .id("one"),
        || P(|| state_two.get().to_string())
            .id("two"),
        || P(|| state_three.get().to_string())
            .id("three"),
        || P(|| if wrong_order.get() { "Wrong order!" } else { "Right order" })
            .id("order"),
        || if is_mutator_showing.get() {
            Div(
                || P("Mutator")
                    .on_appear(|_| {
                        state_two.put(2);
                        if state_three.get() == 3 {
                            // earlier on_appear closures should run before later ones
                            wrong_order.put(true);
                        }
                    })
                    .on_appear(|_| state_three.put(3))
                    .on_disappear(|_| state_two.put(22))
                    .on_disappear(|_| state_three.put(33))
            )
                .on_appear(|_| state_one.put(1))
                .on_disappear(|_| state_one.put(11))
        } else {
            Div(())
        },
        || Button("Toggle mutator")
            .on_click(|_| is_mutator_showing.set(|m| !m))
            .id("toggle")
    ))
}

politana_test!(Test {
    name: "Appear/disappear",
    view: TestView,
    test: async |webpage| {
        let one = webpage.element_by_id("one")?;
        let two = webpage.element_by_id("two")?;
        let three = webpage.element_by_id("three")?;
        let order = webpage.element_by_id("order")?;
        let toggle = webpage.element_by_id("toggle")?;
        one.has_text("0")?;
        two.has_text("0")?;
        three.has_text("0")?;
        toggle.html().click();
        one.has_text("1")?;
        two.has_text("2")?;
        three.has_text("3")?;
        order.has_text("Right order")?;
        toggle.html().click();
        one.has_text("11")?;
        two.has_text("22")?;
        three.has_text("33")?;
        Ok(())
    }
});

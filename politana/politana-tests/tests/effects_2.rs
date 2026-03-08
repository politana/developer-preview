use politana::{Button, Div, El, H1, P, State, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn TestView() -> El {
    let counter_1 = State::new(0);
    let counter_2 = State::new(0);
    let observe_counter_1 = State::new(true);
    let observation = State::new(0);
    let count_observations = State::new(0);
    Div((
        || H1("Effects"),
        || P(|| counter_1.get().to_string()),
        || Button("Increment counter 1")
            .on_click(|_| counter_1.set(|c| c + 1))
            .id("increment-1"),
        || P(|| counter_2.get().to_string()),
        || Button("Increment counter 2")
            .on_click(|_| counter_2.set(|c| c + 1))
            .id("increment-2"),
        || P(|| format!("Observing counter 1: {}", observe_counter_1.get())),
        || Button("Toggle observing")
            .on_click(|_| observe_counter_1.set(|x| !x))
            .id("toggle-observing"),
        || P(|| observation.get().to_string())
            .id("observation"),
        || P(|| count_observations.get().to_string())
            .id("count-observations")
    ))
        .effect(|| {
            count_observations.set(|c| c + 1);
            if observe_counter_1.get() {
                observation.put(counter_1.get());
            } else {
                observation.put(counter_2.get());
            }
        })
}

politana_test!(Test {
    name: "Effects 2",
    view: TestView,
    test: async |webpage| {
        let increment_1 = webpage.element_by_id("increment-1")?;
        let increment_2 = webpage.element_by_id("increment-2")?;
        let toggle_observing = webpage.element_by_id("toggle-observing")?;
        let observation = webpage.element_by_id("observation")?;
        let count_observations = webpage.element_by_id("count-observations")?;
        count_observations.has_text("1")?;
        for _ in 0..10 {
            increment_2.html().click();
        }
        count_observations.has_text("1")?;
        increment_1.html().click();
        count_observations.has_text("2")?;
        observation.has_text("1")?;
        toggle_observing.html().click();
        count_observations.has_text("3")?;
        observation.has_text("10")?;
        increment_1.html().click();
        count_observations.has_text("3")?;
        observation.has_text("10")?;
        increment_2.html().click();
        count_observations.has_text("4")?;
        observation.has_text("11")?;
        Ok(())
    }
});

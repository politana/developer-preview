use politana::{Div, El, Input, InputType, Label, P, State, TypedEventTargets, UniqueId, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn TestView() -> El {
    let input_id = UniqueId::new();
    let is_checked = State::new(false);
    Div((
        || Input()
            .input_type(InputType::Checkbox)
            .id(input_id)
            .on_change(|event| {
                is_checked.put(event.input_target().checked())
            }),
        || Label("Hello")
            .label_for(input_id)
            .id("label"),
        || P(|| is_checked.get().to_string())
            .id("is-checked"),
        || P(|| format!("{}", input_id))
            .id("input-id")
    ))
}

politana_test!(Test {
    name: "Unique ID",
    view: TestView,
    test: async |webpage| {
        let is_checked = webpage.element_by_id("is-checked")?;
        let label = webpage.element_by_id("label")?;
        let input_id = webpage.element_by_id("input-id")?;
        is_checked.has_text("false")?;
        label.html().click();
        is_checked.has_text("true")?;
        input_id.has_text("politana-unique-100000")?;
        Ok(())
    }
});

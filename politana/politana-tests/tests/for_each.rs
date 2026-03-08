use politana::{Button, Div, El, ForEach, State, View, error_messages};
use politana_tests::{expect_framework_error::expect_framework_error, politana_test, test::Test};

#[View]
fn Counter(id: &'static str) -> El {
    let counter = State::new(0);
    Button(|| counter.get().to_string())
        .on_click(|_| counter.set(|c| c + 1))
        .id(id)
}

#[View]
fn TestView() -> El {
    let items = State::new(vec!["one", "two"]);
    Div((
        || Div(ForEach(
            || items.get(),
            |item| Counter(item)
        )),
        || Button("Insert unique key")
            .on_click(|_| items.update(|i| i.insert(1, "three")))
            .id("unique"),
        || Button("Insert duplicate key")
            .on_click(|_| items.update(|i| i.insert(1, "two")))
            .id("duplicate")
    ))
}

politana_test!(Test {
    name: "ForEach",
    view: TestView,
    test: async |webpage| {
        webpage.element_by_id("one")?.html().click();
        for _ in 0..2 {
            webpage.element_by_id("two")?.html().click();
        }
        webpage.element_by_id("unique")?.html().click();
        for _ in 0..3 {
            webpage.element_by_id("three")?.html().click();
        }
        webpage.element_by_id("one")?.has_text("1")?;
        webpage.element_by_id("two")?.has_text("2")?;
        webpage.element_by_id("three")?.has_text("3")?;
        let duplicate = webpage.element_by_id("duplicate")?;
        expect_framework_error(error_messages::DUPLICATE_FOR_EACH_KEY, || {
            duplicate.html().click();
        })?;
        Ok(())
    }
});

use politana::{Button, Div, El, ForEachKeyed, State, View, error_messages};
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
    let items = State::new(vec![("one", 1), ("two", 2)]);
    Div((
        || Div(ForEachKeyed(
            || items.get(),
            |item| item.1,
            |item| Counter(item.0)
        )),
        || Button("Insert unique key")
            .on_click(|_| items.update(|i| i.insert(1, ("three", 3))))
            .id("unique"),
        || Button("Insert duplicate key")
            .on_click(|_| items.update(|i| i.insert(1, ("four", 3))))
            .id("duplicate")
    ))
}

politana_test!(Test {
    name: "ForEachKeyed",
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

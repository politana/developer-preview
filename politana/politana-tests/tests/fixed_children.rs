use politana::{Div, P};
use politana_tests::{politana_test, test::Test};

politana_test!(Test {
    name: "Fixed children",
    view: || Div((
        || P("One").id("one"),
        || P(|| format!("Two")).id("two"),
        || Div(())
            .id("empty")
    )),
    test: async |webpage| {
        webpage.element_by_id("one")?.has_text("One")?;
        webpage.element_by_id("two")?.has_text("Two")?;
        webpage.element_by_id("empty")?.has_child_count(0)?;
        Ok(())
    }
});

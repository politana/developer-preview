use politana::P;
use politana_tests::{politana_test, test::Test};

politana_test!(Test {
    name: "Custom style",
    view: || P("Hello world")
        .style("writing-mode", "vertical-rl")
        .style("text-orientation", "upright")
        .id("styled"),
    test: async |webpage| {
        webpage.element_by_id("styled")?.has_style("writing-mode", "vertical-rl")?;
        webpage.element_by_id("styled")?.has_style("text-orientation", "upright")?;
        Ok(())
    }
});

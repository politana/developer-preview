use politana::P;
use politana_tests::{politana_test, test::Test};

politana_test!(Test {
    name: "Font variation",
    view: || P("Hello")
        .font_variation_settings([
            ("wght", 400.0),
            ("opsz", 40.0)
        ])
        .id("paragraph"),
    test: async |webpage| webpage.element_by_id("paragraph")?
            .has_style("font-variation-settings", "\"opsz\" 40, \"wght\" 400")
});

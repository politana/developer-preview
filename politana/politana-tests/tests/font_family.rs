use politana::{FontFamily, P};
use politana_tests::{politana_test, test::Test};

politana_test!(Test {
    name: "Font family",
    view: || P("Hello")
        .font_family([FontFamily::Named("EB Garamond"), FontFamily::Cursive])
        .id("styled"),
    test: async |webpage| webpage
        .element_by_id("styled")?
        .has_style("font-family", "\"EB Garamond\", cursive")
});

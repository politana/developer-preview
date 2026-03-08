use politana::P;
use politana_tests::{politana_test, test::Test};

politana_test!(Test {
    name: "Custom attribute",
    view: || P("Blue")
        .id("blue")
        .attr("custom-attr", "blue")
        .global_css("
            [custom-attr='blue'] {
                color: #00F;
            }
        "),
    test: async |webpage| webpage
        .element_by_id("blue")?.has_style("color", "rgb(0, 0, 255)")
});

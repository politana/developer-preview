use politana::{Div, P};
use politana_tests::{politana_test, test::Test};

politana_test!(Test {
    name: "Boolean attribute",
    view: || Div((
        || P("Blue")
            .id("blue")
            .bool_attr("custom-bool", true),
        || P("Green")
            .id("green")
            .bool_attr("custom-bool", || false)
    ))
        .global_css("
            [custom-bool] {
                color: #00F;
            }
            p {
                color: #0F0;
            }
        "),
    test: async |webpage| {
        webpage.element_by_id("blue")?.has_style("color", "rgb(0, 0, 255)")?;
        webpage.element_by_id("green")?.has_style("color", "rgb(0, 255, 0)")?;
        Ok(())
    }
});

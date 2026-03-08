use politana::{Div, IntoLength, P};
use politana_tests::{politana_test, test::Test};

politana_test!(Test {
    name: "Margin/padding",
    view: || Div((
        || P("Horizontal")
            .id("horizontal")
            .margin_horizontal(10.px())
            .margin_left(20.px()),
        || P("Around")
            .id("around")
            .padding(10.px())
            .padding_bottom(20.px()),
    )),
    test: async |webpage| {
        let horizontal = webpage.element_by_id("horizontal")?;
        horizontal.has_style("margin-left", "20px")?;
        horizontal.has_style("margin-right", "10px")?;
        let around = webpage.element_by_id("around")?;
        around.has_style("padding-top", "10px")?;
        around.has_style("padding-left", "10px")?;
        around.has_style("padding-right", "10px")?;
        around.has_style("padding-bottom", "20px")?;
        Ok(())
    }
});

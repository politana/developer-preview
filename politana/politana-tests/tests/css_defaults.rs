use politana::{CssDefaults, Div, P};
use politana_tests::{politana_test, test::Test};

politana_test!(Test {
    name: "CSS defaults",
    view: || Div(
        || P("Hello")
            .color(CssDefaults::Unset)
            .border_color(CssDefaults::Unset)
            .border_style(CssDefaults::Unset)
            .border_width(CssDefaults::Unset)
            .id("paragraph")
    )
        .global_css("
            div {
                color: red;
                border: 5px solid red;
            }
            p {
                color: green;
                border: 2px dashed blue;
            }
        "),
    test: async |webpage| {
        webpage.element_by_id("paragraph")?.has_style("color", "rgb(255, 0, 0)")?;
        webpage.element_by_id("paragraph")?.has_style("border-style", "none")?;
        Ok(())
    }
});

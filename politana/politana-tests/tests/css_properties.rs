use politana::{Color, IntoLength, P};
use politana_tests::{politana_test, test::Test};

politana_test!(Test {
    name: "CSS property types",
    view: || P("Hello")
        .color(Color::Hsla(60.0, 1.0, 0.5, 1.0))
        .height(50.px())
        .id("hi"),
    test: async |webpage| {
        webpage.element_by_id("hi")?.has_style("color", "rgb(255, 255, 0)")?;
        webpage.element_by_id("hi")?.has_style("height", "50px")?;
        Ok(())
    }
});

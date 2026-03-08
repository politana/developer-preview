use politana::{Button, Div, El, IntoLength, P, State, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn TestView() -> El {
    let counter = State::new(0);
    let is_black = State::new(false);
    Div((
        || P(|| format!("Counter is {}", counter.get()))
            .attr("class", || if is_black.get() { "black" } else { "red" })
            .attr("class", "big")
            .class("spacey")
            .padding_right(24.px())
            .id("paragraph"),
        || Button("Increment")
            .on_click(|_| counter.set(|c| c + 1))
            .id("increment"),
        || Button("Toggle black")
            .on_click(|_| is_black.set(|x| !x))
            .id("toggle")
    ))
        .global_css("
            .black {
                color: black;
            }
            .red {
                color: red;
            }
            .big {
                font-size: 30px;
            }
            .spacey {
                padding: 12px;
            }
        ")
}

politana_test!(Test {
    name: "Class",
    view: TestView,
    test: async |webpage| {
        let paragraph = webpage.element_by_id("paragraph")?;
        let toggle = webpage.element_by_id("toggle")?;
        let increment = webpage.element_by_id("increment")?;
        paragraph.has_style("color", "rgb(255, 0, 0)")?;
        paragraph.has_style("font-size", "30px")?;
        paragraph.has_style("padding-left", "12px")?;
        paragraph.has_style("padding-right", "24px")?;
        toggle.html().click();
        paragraph.has_style("color", "rgb(0, 0, 0)")?;
        paragraph.has_style("font-size", "30px")?;
        paragraph.has_style("padding-left", "12px")?;
        paragraph.has_style("padding-right", "24px")?;
        paragraph.has_text("Counter is 0")?;
        increment.html().click();
        paragraph.has_style("color", "rgb(0, 0, 0)")?;
        paragraph.has_style("font-size", "30px")?;
        paragraph.has_style("padding-left", "12px")?;
        paragraph.has_style("padding-right", "24px")?;
        paragraph.has_text("Counter is 1")?;
        Ok(())
    }
});

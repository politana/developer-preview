use politana::{Button, Div, El, P, State, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn GlobalStyleToggle(id: &'static str) -> El {
    let is_enabled = State::new(false);
    Div((
        || Button("Toggle style")
            .id(id)
            .on_click(|_| is_enabled.set(|x| !x)),
        || if is_enabled.get() {
            P("Global style is applied")
                .global_css("
                    p {
                        color: #0F0;
                    }
                ")
                .global_css("
                    p {
                        border-style: solid;
                        border-width: 2px;
                        border-color: #000;
                    }
                ")
        } else {
            Div(())
        }
    ))
}

#[View]
fn GlobalStyle() -> El {
    Div((
        || P("Maybe green?")
            .id("maybe-green"),
        || GlobalStyleToggle("toggle-one"),
        || GlobalStyleToggle("toggle-two")
    ))
}

politana_test!(Test {
    name: "Global CSS",
    view: GlobalStyle,
    test: async |webpage| {
        let toggle_one = webpage.element_by_id("toggle-one")?;
        let toggle_two = webpage.element_by_id("toggle-two")?;
        let maybe_green = webpage.element_by_id("maybe-green")?;
        maybe_green.has_style("color", "rgb(0, 0, 0)")?;
        maybe_green.has_style("border-style", "none")?;
        toggle_one.html().click();
        maybe_green.has_style("color", "rgb(0, 255, 0)")?;
        maybe_green.has_style("border-style", "solid")?;
        toggle_two.html().click();
        maybe_green.has_style("color", "rgb(0, 255, 0)")?;
        maybe_green.has_style("border-style", "solid")?;
        toggle_one.html().click();
        maybe_green.has_style("color", "rgb(0, 255, 0)")?;
        maybe_green.has_style("border-style", "solid")?;
        toggle_two.html().click();
        maybe_green.has_style("color", "rgb(0, 0, 0)")?;
        maybe_green.has_style("border-style", "none")?;
        Ok(())
    }
});

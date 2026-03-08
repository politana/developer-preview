use politana::{Button, Color, Div, El, IntoLength, P, Pseudo, State, View, error_messages};
use politana_tests::{expect_framework_error::expect_framework_error, politana_test, test::Test};

#[View]
fn TestView() -> El {
    let is_showing_nesting_pseudo = State::new(false);
    Div((
        || Button("Show nesting")
            .on_click(|_| is_showing_nesting_pseudo.put(true))
            .id("nesting"),
        || if is_showing_nesting_pseudo.get() {
            P("Hello")
                .pseudo(Pseudo::Hover, |el| el
                    .color(Color::Black)
                    .pseudo(Pseudo::Hover, |el| el
                        .padding(16.px())
                    )
                )
        } else {
            Div(())
        }
    ))
}

politana_test!(Test {
    name: "Pseudo nesting",
    view: TestView,
    test: async |webpage| {
        let nesting = webpage.element_by_id("nesting")?;
        expect_framework_error(error_messages::NESTED_USING_PSEUDO, || {
            nesting.html().click();
        })?;
        Ok(())
    }
});

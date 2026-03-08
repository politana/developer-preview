use politana::{Button, Div, El, P, State, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn TestView() -> El {
    let is_showing_resource_trigger = State::new(true);
    Div((
        || P("Hello world!")
            .id("paragraph"),
        || Button("Hide trigger")
            .id("hide")
            .on_click(|_| is_showing_resource_trigger.put(false)),
        || if is_showing_resource_trigger.get() {
            P("Resource trigger")
                .use_resource("<style> p { color: #0F0; } </style>")
        } else { Div(()) }
    ))
}

politana_test!(Test {
    name: "Use resource",
    view: TestView,
    test: async |webpage| {
        let hide = webpage.element_by_id("hide")?;
        let paragraph = webpage.element_by_id("paragraph")?;
        hide.html().click();
        paragraph.has_style("color", "rgb(0, 255, 0)")?;
        Ok(())
    }
});

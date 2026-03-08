use std::time::Duration;

use gloo_timers::future::sleep;
use politana::{Button, Div, El, P, State, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn TestView() -> El {
    let opacity = State::new(0.);
    let did_transition = State::new(false);
    Div((
        || P("Maybe opaque")
            .opacity(|| opacity.get())
            .style("transition", "opacity 200ms")
            .event_listener("transitionend", |_| {
                did_transition.put(true);
            }),
        || Button("Make opaque")
            .id("make-opaque")
            .on_click(|_| opacity.put(1.)),
        || P(|| if did_transition.get() {
            "Transition complete"
        } else {
            "Transition not complete"
        })
            .id("status")
    ))
}

politana_test!(Test {
    name: "Event listeners",
    view: TestView,
    test: async |webpage| {
        let make_opaque = webpage.element_by_id("make-opaque")?;
        let status = webpage.element_by_id("status")?;
        sleep(Duration::from_millis(100)).await;
        make_opaque.html().click();
        sleep(Duration::from_millis(300)).await;
        status.has_text("Transition complete")?;
        Ok(())
    }
});

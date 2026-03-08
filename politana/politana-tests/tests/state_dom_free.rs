use std::rc::Rc;

use politana::{Button, Div, El, P, State, View};
use politana_tests::{politana_test, test::Test};

#[View]
fn TestView() -> El {
    let shared = State::new(Some(Rc::new("Hello".to_string())));
    let weak = State::new(Rc::downgrade(&shared.get_once().unwrap()));
    let weak_read_result = State::new("");
    let is_taker_visible = State::new(false);
    Div((
        || if is_taker_visible.get() {
            let ours_now: State<Option<Rc<String>>> = State::default();
            P("Taker")
                .on_appear(|_| {
                    shared.update(|s| {
                        ours_now.put(s.take());
                    });
                })
        } else {
            Div(())
        },
        || Button("Toggle taker")
            .id("toggle")
            .on_click(|_| {
                is_taker_visible.set(|x| !x);
            }),
        || Button("Check if weak is alive")
            .id("check")
            .on_click(|_| {
                if weak.map(|w| w.upgrade().is_some()) {
                    weak_read_result.put("Alive!");
                } else {
                    weak_read_result.put("Dead!");
                }
            }),
        || P(|| weak_read_result.get())
            .id("weak-result")
    ))
}

politana_test!(Test {
    name: "State frees memory on DOM node removal",
    view: TestView,
    test: async |webpage| {
        let checker = webpage.element_by_id("check")?;
        let toggle = webpage.element_by_id("toggle")?;
        let weak_result = webpage.element_by_id("weak-result")?;
        // starts out alive
        checker.html().click();
        weak_result.has_text("Alive!")?;
        // move into temporary element
        toggle.html().click();
        checker.html().click();
        weak_result.has_text("Alive!")?;
        // destroy temporary element
        toggle.html().click();
        checker.html().click();
        weak_result.has_text("Dead!")?;
        Ok(())
    }
});

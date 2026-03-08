use std::time::Duration;

use gloo_timers::future::sleep;
use politana::{Button, Closure, Color, Div, El, P, View, library::{NavigationHost, Routes}};
use politana_tests::{politana_test, test::Test};

#[View]
fn RedFish(go_to_blue: Closure<(), ()>) -> El {
    Div((
        || P("Red fish")
            .color(Color::Rgba(255.0, 0.0, 0.0, 1.0))
            .id("fish"),
        || Button("Go to blue")
            .on_click(|_| go_to_blue.call(()))
            .id("go-to-blue")
    ))
}

#[View]
fn BlueFish(go_to_red: Closure<(), ()>) -> El {
    Div((
        || P("Blue fish")
            .color(Color::Rgba(0.0, 0.0, 255.0, 1.0))
            .id("fish"),
        || Button("Go to red")
            .on_click(|_| go_to_red.call(()))
            .id("go-to-red")
    ))
}

#[View]
fn TestView() -> El {
    NavigationHost(
        Routes::new()
            .route("", |_| NavigationHost(
                Routes::new()
                    .route("", |controller| RedFish(
                        Closure::new(|_| controller.navigate("blue"))
                    ))
                    .route("blue", |controller| BlueFish(
                        Closure::new(|_| controller.go_back())
                    )),
                |_| P("404")
            )),
        |_| P("404")
    )
}

politana_test!(Test {
    name: "Nav host back style bug",
    view: TestView,
    test: async |webpage| {
        webpage.element_by_id("fish")?.has_style("color", "rgb(255, 0, 0)")?;
        webpage.element_by_id("go-to-blue")?.html().click();
        webpage.element_by_id("fish")?.has_style("color", "rgb(0, 0, 255)")?;
        webpage.element_by_id("fish")?.has_text("Blue fish")?;
        webpage.element_by_id("go-to-red")?.html().click();
        webpage.element_by_id("fish")?.has_style("color", "rgb(255, 0, 0)")?;
        webpage.element_by_id("fish")?.has_text("Red fish")?;
        webpage.window().history().unwrap().back().unwrap();
        sleep(Duration::from_millis(100)).await;
        webpage.element_by_id("fish")?.has_style("color", "rgb(0, 0, 255)")?;
        webpage.element_by_id("fish")?.has_text("Blue fish")?;
        webpage.window().history().unwrap().back().unwrap();
        sleep(Duration::from_millis(100)).await;
        webpage.element_by_id("fish")?.has_style("color", "rgb(255, 0, 0)")?;
        webpage.element_by_id("fish")?.has_text("Red fish")?;
        Ok(())
    }
});

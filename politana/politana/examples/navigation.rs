use politana::{Button, Div, El, P, Politana, View, library::{NavigationHost, Routes}};

#[View]
fn App() -> El {
    NavigationHost(
        Routes::new()
            .route("", |controller| Div((
                || P("I am Roy Sianez"),
                || Button("My name")
                    .on_click(|_| controller.navigate("name")),
                || Button("About me")
                    .on_click(|_| controller.navigate("about"))
            )))
            .route("name", |controller| NavigationHost(Routes::new(), |_| Div((
                || P("My full name is Roy Fugère Sianez"),
                || Button("Back")
                    .on_click(|_| controller.go_back())
            ))))
            .route("about", |controller| Div((
                || P("Roy is a great guy."),
                || Button("Back")
                    .on_click(|_| controller.go_back())
            )))
            .route("nested", |_| NavigationHost(
                Routes::new()
                    .route("one", |_| P("One"))
                    .route("two", |_| P("Two")),
                |_| P("Nested fallback")
            ))
            .route("message/*", |controller|
                P(|| format!("Your message is: {}", controller.wildcards()[0]))
            ),
        |_| P("404")
    )
}

fn main() {
    Politana::launch(App);
}

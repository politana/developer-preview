use politana::{Button, Div, El, P, Politana, State, View};

#[View]
fn App() -> El {
    let counter = State::new(0);
    Div((
        || P(|| format!("Counter is {}", counter.get())),
        || Button("Increment")
            .on_click(|_| counter.set(|c| c + 1))
    ))
}

fn main() {
    Politana::launch(App);
}

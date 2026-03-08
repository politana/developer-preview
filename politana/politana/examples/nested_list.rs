use politana::{BorderStyle, Button, Closure, Color, Display, Div, El, ForEach, Input, InputType, IntoLength, Politana, State, TypedEventTargets, View};

#[derive(Default)]
struct Item {
    text: String,
    children: Vec<State<Item>>
}

#[View]
fn ItemView(item: State<Item>, delete: Closure<(), ()>) -> El {
    Div((
        || Div((
            || Input()
                .input_type(InputType::Text)
                .value(|| item.map(|i| i.text.clone()))
                .on_input(|event| {
                    let text = event.input_target().value();
                    item.update(|i| i.text = text);
                }),
            || Button("Add child")
                .on_click(|_| item.update(|i| i.children.push(State::default()))),
            || Button("Delete")
                .on_click(|_| delete.call(()))
        ))
            .display(Display::Flex),
        || Div(ForEach(
            || item.map(|i| i.children.clone()).into_iter().enumerate().collect(),
            |(index, child)| ItemView(
                child,
                Closure::new(|_| item.update(|i| i.children.remove(index)))
            )
        ))
            .border_left_style(BorderStyle::Solid)
            .border_left_color(Color::Black)
            .border_left_width(1.pt())
            .padding_left(8.pt())
    ))
}

#[View]
fn App() -> El {
    let items: State<Vec<State<Item>>> = State::default();
    Div((
        || Div(ForEach(
            || items.get().into_iter().enumerate().collect(),
            |(index, item)| ItemView(
                item,
                Closure::new(|_| items.update(|i| i.remove(index)))
            )
        )),
        || Button("Add item")
            .on_click(|_| items.update(|i| i.push(State::default())))
    ))
}

fn main() {
    Politana::launch(App);
}

use politana::{AlignItems, Button, Color, Display, Div, El, Environment, ForEach, H1, Input, IntoLength, Label, Option, Overflow, P, Politana, Select, State, TypedEventTargets, UniqueId, View, library::{NavigationHost, Routes}};
use rand::{rng, seq::SliceRandom};

#[View]
fn Counter() -> El {
    let counter = State::new(0);
    Div((
        || Button("Increment")
            .on_click(|_| counter.set(|c| c + 1)),
        || P(|| counter.get().to_string())
    ))
        .display(Display::Flex)
        .align_items(AlignItems::Center)
}

#[View]
fn ForEachRerendering() -> El {
    let keys: State<Vec<UniqueId>> = State::default();
    Div((
        || H1("ForEach rerendering"),
        || P("The ForEach component re-renders its children even when they don't change, and does not associate elements with keys."),
        || Div(ForEach(
            || keys.get(),
            |_| Counter()
        )),
        || Button("Add counter")
            .on_click(|_| keys.update(|k| k.push(UniqueId::new()))),
        || Button("Shuffle counters")
            .on_click(|_| keys.update(|k| k.shuffle(&mut rng())))
    ))
}

#[View]
fn StateRegions() -> El {
    let states: State<Vec<State<String>>> = State::default();
    let new_name: State<String> = State::default();
    Div((
        || H1("State regions"),
        || P("When you add a name, the view's code incorrectly depends on its state at too high a level, so the name's counter resets when you chop letters off the name. This is expected, but because of the state region bug, ALL of the name cells' counters wind up resetting! You can also see the ForEach pathology where adding a new element resets all the counters."),
        || Div(ForEach(
            || states.get(),
            |state| {
                Div((
                    || P(|| state.get()),
                    || Button("Chop")
                        .on_click(|_| state.update(|s| s.pop())),
                    || Counter()
                ))
                    .color(if state.get().len() % 2 == 0 {
                        Color::Rgba(255.0, 0.0, 0.0, 1.0)
                    } else {
                        Color::Black
                    })
            }
        )),
        || Input()
            .on_input(|event| new_name.put(event.input_target().value())),
        || Button("Add name")
            .on_click(|_| {
                let name = new_name.get();
                states.update(|s| s.push(State::new(name)))
            })
    ))
}

#[View]
fn DependencyPerformance() -> El {
    let table_size = State::new(1);
    let is_red = State::new(false);
    let is_showing_single_item = State::new(false);
    let select_id = UniqueId::new();
    Div((
        || H1("Dependency performance"),
        || P("Changing a state dependency requires an exhaustive search. In this example, it is slower to toggle the one element out than to toggle it in because of the dependency search."),
        || Button("Add items")
            .on_click(|_| table_size.set(|s| s * 2)),
        || Button("Toggle just one item")
            .on_click(|_| is_showing_single_item.set(|s| !s)),
        || if is_showing_single_item.get() {
            P("Single item")
                .color(|| if is_red.get() {
                    Color::Rgba(255.0, 0.0, 0.0, 1.0)
                } else {
                    Color::Black
                })
        } else {
            Div(())
        },
        || Div((
            || Label("Color")
                .label_for(select_id),
            || Select((
                || Option("Black"),
                || Option("Red")
            ))
                .id(select_id)
                .on_change(|event| {
                    is_red.put(event.select_target().value() == "Red");
                })
        )),
        || Div(ForEach(
            || (0 .. table_size.get()).collect(),
            |row| Div(ForEach(
                || (0 .. table_size.get()).collect(),
                |column| P(|| (row * column).to_string())
                    .color(|| if is_red.get() {
                        Color::Rgba(255.0, 0.0, 0.0, 1.0)
                    } else {
                        Color::Black
                    })
                    .margin(0.px())
                    .width(50.px())
                    .flex_shrink(0.0)
            ))
                .display(Display::Flex)
        ))
            .overflow_x(Overflow::Scroll)
    ))
}

#[View]
fn MemoryProfiling() -> El {
    let keys: State<Vec<UniqueId>> = State::default();
    Div((
        || H1("Memory profiling"),
        || P("Creating and deleting items leaks memory"),
        || Div(ForEach(
            || keys.get(),
            |_| {
                let _lots_of_memory = State::new({
                    let mut result = Vec::<u8>::with_capacity(100_000_000);
                    result.push(3);
                    result
                });
                P("Allocating lots of memory")
            }
        )),
        || Button("Add item")
            .on_click(|_| keys.update(|k| k.push(UniqueId::new()))),
        || Button("Delete")
            .on_click(|_| keys.update(|k| k.pop()))
    ))
}

#[View]
fn Effects() -> El {
    let counter_1 = State::new(0);
    let counter_2 = State::new(0);
    let observe_counter_1 = State::new(true);
    let observation = State::new(0);
    let count_observations = State::new(0);
    Div((
        || H1("Effects"),
        || P(|| format!("Counter 1: {}", counter_1.get())),
        || Button("Increment counter 1")
            .on_click(|_| counter_1.set(|c| c + 1)),
        || P(|| format!("Counter 2: {}", counter_2.get())),
        || Button("Increment counter 2")
            .on_click(|_| counter_2.set(|c| c + 1)),
        || P(|| format!("Observing counter 1: {}", observe_counter_1.get())),
        || Button("Toggle observing")
            .on_click(|_| observe_counter_1.set(|x| !x)),
        || P(|| format!("Observation: {}", observation.get())),
        || P(|| format!("Count observations: {}", count_observations.get()))
    ))
        .effect(|| {
            count_observations.set(|c| c + 1);
            if observe_counter_1.get() {
                observation.put(counter_1.get());
            } else {
                observation.put(counter_2.get());
            }
        })
}

#[View]
fn WindowSize() -> El {
    let window_size = Environment::map_window_size(|w, h| (w, h));
    Div((
        || H1("Window size"),
        || P(|| {
            let (w, h) = window_size.call(());
            format!("Window width: {}, height: {}", w, h)
        })
    ))
}

#[View]
fn NestedFlattenView() -> El {
    let counter = State::new(0);
    Div((
        || H1("Nested FlattenView"),
        || {
            counter.get();
            NavigationHost(Routes::new(), |_|
                Button(|| format!("Counter is {}", counter.get_once()))
                    .on_click(|_| counter.set(|c| c + 1))
            )
        }
    ))
}

#[View]
fn GlobalStyleToggle() -> El {
    let is_enabled = State::new(false);
    Div((
        || Button("Toggle style")
            .on_click(|_| is_enabled.set(|x| !x)),
        || if is_enabled.get() {
            P("Global style is applied")
                .global_css("
                    body {
                        color: green;
                        font-family: sans-serif;
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
        || H1("Global Style"),
        || GlobalStyleToggle(),
        || GlobalStyleToggle()
    ))
}

#[View]
fn BadStateRead() -> El {
    let is_showing = State::new(false);
    if is_showing.get() {
        P("Hello world!")
    } else { Div(()) }
}

#[View]
fn ErrorExampleView() -> El {
    let show_bad_state_read = State::new(false);
    Div((
        || H1("Framework error example"),
        || Button("Show bad state read")
            .id("show-bad-state-read")
            .on_click(|_| {
                show_bad_state_read.put(true);
            }),
        || if show_bad_state_read.get() {
            BadStateRead()
        } else { Div(()) }
    ))
}

#[View]
fn App() -> El {
    Div((
        || ForEachRerendering(),
        || StateRegions(),
        || DependencyPerformance(),
        || MemoryProfiling(),
        || Effects(),
        || WindowSize(),
        || NestedFlattenView(),
        || GlobalStyle(),
        || ErrorExampleView()
    ))
}

fn main() {
    Politana::launch(App);
}

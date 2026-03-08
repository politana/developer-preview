use std::{rc::Rc, sync::atomic::Ordering};

use web_sys::window;

use crate::{El, debug_bar::ui::add_debug_bar::AddDebugBar, utils::is_test_suite::IS_TEST_SUITE, reactivity::{create_html_element::CreateHtmlElement, default_head_content::DEFAULT_HEAD_CONTENT, environment::EnvironmentStorage, update_ui::update_ui, vdom::{Vdom, VirtualElement}, vdom_ref::VdomRef, window_resize::start_observing_resize}};

pub fn render(app: fn() -> El) {
    let is_test_suite = IS_TEST_SUITE.with(|s| s.load(Ordering::Relaxed));
    let app: Box<dyn Fn() -> El> = if cfg!(debug_assertions) && !is_test_suite {
        Box::new(move || AddDebugBar(app))
    } else { Box::new(move || app()) };
    let window = window().unwrap();
    let document = window.document().unwrap();
    start_observing_resize(window.clone());
    EnvironmentStorage::set_environment(EnvironmentStorage {
        window: window.clone(),
        document: document.clone()
    });
    let head = document.head().unwrap();
    if !is_test_suite {
        head.set_inner_html(DEFAULT_HEAD_CONTENT);
    }
    let body = document.body().unwrap();
    if !is_test_suite {
        body.replace_children_with_node_0();
    }
    let phantom_el = document.create_html_element("div");
    body.append_child(&phantom_el).unwrap();
    let root = VdomRef::new(Vdom::new(
        phantom_el,
        VirtualElement::ReplaceElement {
            current_children: Vec::new(),
            content: Rc::new(app)
        },
        Vec::new()
    ));
    Vdom::init_tree(document, root);
    Vdom::using(root, update_ui);
}

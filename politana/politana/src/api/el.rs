use std::{collections::HashMap, rc::Rc};

use web_sys::{Event, HtmlElement};

#[cfg(not(target_arch = "wasm32"))]
use crate::library::{nav_host::routes::PathComponent, Routes};
use crate::{api::{computed::Computed, element_children::ElementChildren, property::{Property, PropertyKey, Pseudo}}, reactivity::hash_eq_clone::{AnyClone, HashEq}};

pub struct El {
    pub(crate) inner: ElInner,
    pub(crate) current_pseudo: Pseudo,
    /// Adds this path to the current navigation prefix
    pub(crate) navigation_path: Vec<String>,
    pub(crate) lifecycle: LifecycleEvents,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) global_css: Vec<&'static str>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) possible_routes: Vec<Vec<PathComponent>>
}

pub enum ElInner {
    Static(ElData),
    Flattened(Rc<dyn Fn() -> El>)
}

pub struct ElData {
    pub(crate) tag: &'static str,
    pub(crate) children: Children,
    pub(crate) properties: HashMap<PropertyKey, Property>,
    pub(crate) event_listeners: Vec<EventListener>,
    /// The return value is only used internally for effects that need to debounce.
    /// A return value of `true` means the framework will track the states that were read.
    pub(crate) effects: Vec<Rc<dyn Fn() -> bool>>,
    pub(crate) classes: Vec<Computed<Vec<String>>>
}

pub enum Children {
    Fixed(Vec<Computed<El>>),
    StaticString(Computed<&'static str>),
    String(Computed<String>),
    ForEach(ForEachContent),
    InnerHtml(Computed<InnerHtmlContent>)
}

pub struct ForEachContent {
    pub items: Computed<Vec<AnyClone>>,
    pub item_id: Rc<dyn Fn(&AnyClone) -> HashEq>,
    pub element: Rc<dyn Fn(AnyClone) -> El>
}

#[derive(Clone)]
pub struct InnerHtmlContent(pub String);

pub fn InnerHtml(html: impl Into<String>) -> InnerHtmlContent {
    InnerHtmlContent(html.into())
}

pub struct EventListener {
    pub event_type: &'static str,
    pub listener: Rc<dyn Fn(Event)>
}

#[derive(Default)]
pub struct LifecycleEvents {
    pub on_appear: Vec<Option<Box<dyn FnOnce(HtmlElement)>>>,
    pub on_disappear: Vec<Option<Box<dyn FnOnce(HtmlElement)>>>
}

pub fn El(tag: &'static str, children: impl ElementChildren) -> El {
    El {
        inner: ElInner::Static(ElData {
            tag,
            children: children.element_children(),
            properties: HashMap::new(),
            event_listeners: Vec::new(),
            effects: Vec::new(),
            classes: Vec::new()
        }),
        current_pseudo: Pseudo::None,
        navigation_path: Vec::new(),
        lifecycle: LifecycleEvents::default(),
        #[cfg(not(target_arch = "wasm32"))]
        global_css: Vec::new(),
        #[cfg(not(target_arch = "wasm32"))]
        possible_routes: Vec::new()
    }
}

pub(crate) fn FlattenView(view: impl Fn() -> El + 'static) -> El {
    El {
        inner: ElInner::Flattened(Rc::new(view)),
        current_pseudo: Pseudo::None,
        navigation_path: Vec::new(),
        lifecycle: LifecycleEvents::default(),
        #[cfg(not(target_arch = "wasm32"))]
        global_css: Vec::new(),
        #[cfg(not(target_arch = "wasm32"))]
        possible_routes: Vec::new()
    }
}

impl El {
    pub(crate) fn map<F: Fn(ElData) -> ElData + 'static>(self, closure: F) -> Self {
        self.map_impl(Box::new(closure))
    }

    fn map_impl(self, closure: Box<dyn Fn(ElData) -> ElData + 'static>) -> Self {
        let closure = Rc::new(closure);
        match self.inner {
            ElInner::Static(el_data) => El {
                inner: ElInner::Static(closure(el_data)),
                ..self
            },
            ElInner::Flattened(view) => El {
                inner: ElInner::Flattened(Rc::new(move || {
                    let data = view();
                    let closure = closure.clone();
                    data.map(move |d| closure.clone()(d))
                })),
                ..self
            }
        }
    }

    pub(crate) fn update<F: Fn(&mut ElData) -> T + 'static, T>(&mut self, closure: F) {
        let closure = Rc::new(closure);
        match &mut self.inner {
            ElInner::Static(el_data) => {
                closure(el_data);
            }
            ElInner::Flattened(view) => {
                let view_clone = view.clone();
                *view = Rc::new(move || {
                    let data = view_clone();
                    let closure = closure.clone();
                    data.map(move |mut d| {
                        closure(&mut d);
                        d
                    })
                });
            }
        }
    }

    pub(crate) fn navigation_path(mut self, path: Vec<String>) -> Self {
        self.navigation_path = path;
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn possible_routes(mut self, routes: &Routes) -> Self {
        self.possible_routes = routes.0.iter().map(|r| r.path.clone()).collect();
        self
    }
}

use std::rc::Rc;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::Event;

use crate::{Closure, El, Environment, State, api::el::FlattenView, library::nav_host::{nav_controller::NavController, path_tools::PathTools, routes::{PathComponent, Routes}}, reactivity::current_nav_prefix::current_nav_prefix, utils::{error_messages, report_error::report_error, unwrap_or_error::UnwrapOrError}};

pub fn NavigationHostImpl(
    routes: Routes,
    fallback: impl Fn(NavController) -> El + 'static
) -> El {
    let popstate_event_handler: State<Option<wasm_bindgen::prelude::Closure<dyn Fn(Event)>>> =
        State::default();
    // Cached values, never mutated
    let routes = State::new(routes);
    let fallback = Closure::new(move |controller| fallback(controller));
    let nav_prefix = State::new(
        current_nav_prefix()
            .unwrap_or_else(|| report_error(error_messages::MISPLACED_NAV_HOST))
    );
    // Manually mutated to update the view
    let local_nav_path = State::new(
        // Won't be null because the parent has just run with nav_prefix
        get_local_nav_path(nav_prefix.get_once())
            .unwrap_or_else(|| report_error(error_messages::UNEXPECTED))
    );
    // Stored to check on updates
    let last_consumed_path = State::default();
    FlattenView(move || {
        local_nav_path.map(|_| ()); // establish a dependency
        let path_match = routes.map(
            // Use "get_once" internally to avoid a double clone
            |routes| find_matching_path(local_nav_path.get_once(), routes)
                .unwrap_or_else(|| PathMatch {
                    view: fallback,
                    consumed_path: Vec::new(),
                    wildcards: local_nav_path.get_once()
                })
        );
        let no_history_error: fn() -> String = || error_messages::browser_error(
            "NavigationHost: Browser window doesn't have a history property");
        let bad_path_error: fn(&str) -> String = |path| error_messages::browser_error(&format!(
            "NavigationHost: Browser cannot navigate to invalid destination URL\n\nThe URL was \"{}\"\n\nThis might be caused by passing an incorrect argument to NavigationController::navigate.",
            path));
        let controller = NavController {
            navigate: Closure::new(move |destination: String| {
                let mut destination = destination.split_path();
                let full_path = nav_prefix.get_once().iter()
                    .map(|s| s.as_str())
                    .chain(local_nav_path.get_once().iter().map(|s| s.as_str()))
                    .chain(destination.iter().map(|s| s.as_str()))
                    .collect::<Vec<_>>()
                    .join("/")
                    .add_leading_slash();
                Environment::window().history()
                    .ok().unwrap_or_report(&no_history_error())
                    .push_state_with_url(&JsValue::undefined(), "", Some(&full_path))
                    .ok().unwrap_or_report(&bad_path_error(&full_path));
                local_nav_path.update(|p| p.append(&mut destination));
            }),
            go_back: Closure::new(move |_| {
                let mut full_path = nav_prefix.get_once().into_iter()
                    .map(|s| s.to_string())
                    .chain(local_nav_path.get_once().into_iter().map(|s| s.to_string()))
                    .collect::<Vec<_>>();
                full_path.pop();
                let full_path = full_path.join("/").add_leading_slash();
                Environment::window().history()
                    .ok().unwrap_or_report(&no_history_error())
                    .push_state_with_url(&JsValue::undefined(), "", Some(&full_path))
                    .ok().unwrap_or_report(&bad_path_error(&full_path));
                local_nav_path.update(|p| p.pop());
            }),
            wildcards: State::new(path_match.wildcards)
        };
        FlattenView(move || path_match.view.call(controller))
            .navigation_path(path_match.consumed_path.clone())
            .on_appear(move |_| last_consumed_path.put(path_match.consumed_path))
    })
        .on_appear(move |_| {
            let closure = wasm_bindgen::prelude::Closure::<dyn Fn(Event)>::wrap(
                Box::new(move |_| {
                    if let Some(new_local_path) = get_local_nav_path(nav_prefix.get_once()) {
                        let old_consumed_path = last_consumed_path.get_once();
                        if should_reload_on_popstate(&old_consumed_path, &new_local_path) {
                            local_nav_path.put(new_local_path)
                        }
                    }
                })
            );
            Environment::window()
                .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
                .ok().unwrap_or_report(&error_messages::browser_error("NavigationHost: Cannot attach event listener"));
            popstate_event_handler.put(Some(closure));
        })
        .on_disappear(move |_| {
            popstate_event_handler.update(|h| {
                if let Some(closure) = h.take() {
                    Environment::window()
                        .remove_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
                        .ok().unwrap_or_report(&error_messages::browser_error("NavigationHost: Cannot tear down event listener"));
                }
            });
        })
}

struct PathMatch {
    view: Closure<NavController, El>,
    consumed_path: Vec<String>,
    wildcards: Vec<String>
}

fn find_matching_path(
    local_nav_path: Vec<String>, routes: &Routes
) -> Option<PathMatch> {
    routes.0.iter()
        .rev() // first match gets priority
        .filter(|route| route.matches_path(&local_nav_path))
        .max_by_key(|route| route.path.len())
        .map(|route| {
            let wildcards = local_nav_path.iter()
                .zip(&route.path)
                .filter_map(|(actual_str, spec)| match spec {
                    PathComponent::Wildcard => Some(actual_str.clone()),
                    PathComponent::Literal(_) => None,
                })
                .collect();
            let consumed = local_nav_path.into_iter()
                .take(route.path.len()).collect();
            PathMatch {
                view: route.view,
                consumed_path: consumed,
                wildcards
            }
        })
}

/// Null if the prefix does not match the beginning of the full path (i.e. the parent has navigated away)
fn get_local_nav_path(prefix: Vec<Rc<String>>) -> Option<Vec<String>> {
    let components = full_nav_path();
    if components.len() >= prefix.len()
        && components.iter().zip(prefix.iter()).all(|(a, b)| *a == **b)
    {
        Some(components.iter().skip(prefix.len()).map(|s| s.to_string()).collect())
    } else {
        None
    }
}

#[cfg(target_arch = "wasm32")]
fn full_nav_path() -> Vec<String> {
    Environment::window()
        .location()
        .pathname()
        .unwrap_or_default()
        .split_path()
}

// In this case, it is the local nav path and `current_nav_path` is the empty path
#[cfg(not(target_arch = "wasm32"))]
fn full_nav_path() -> Vec<String> {
    use crate::package::mock_nav_path::MOCK_NAV_PATH;
    MOCK_NAV_PATH.with_borrow(|p| p.clone())
}

fn should_reload_on_popstate(
    old_consumed_path: &Vec<String>,
    new_local_path: &Vec<String>
) -> bool {
    new_local_path.len() < old_consumed_path.len()
    || old_consumed_path.into_iter().zip(new_local_path).any(|(a, b)| a != b)
}

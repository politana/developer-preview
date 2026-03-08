use crate::{El, api::el::{Children, ElInner}, package::{concrete_path::concrete_path, mock_nav_path::MOCK_NAV_PATH}};

pub fn possible_nav_paths(el: impl Fn() -> El) -> Vec<Vec<&'static str>> {
    let possible_routes = el().possible_routes;
    if possible_routes.is_empty() {
        let mut result = None;
        for child in children(el()) {
            MOCK_NAV_PATH.set(Vec::new());
            let paths = possible_nav_paths(child);
            if !paths.is_empty() {
                if result.is_some() {
                    panic!("Multiple nav paths!");
                }
                result = Some(paths);
            }
        }
        result.unwrap_or_default()
    } else {
        let mut result: Vec<Vec<_>> = Vec::new();
        let mut possible_routes: Vec<_> = possible_routes.into_iter().filter_map(concrete_path).collect();
        if !possible_routes.contains(&Vec::new()) {
            possible_routes.push(Vec::new());
        }
        for route in possible_routes {
            let mut subpaths = None;
            let search_path = route.iter().map(|s| s.to_string()).collect();
            MOCK_NAV_PATH.set(search_path);
            for child in children(el()) {
                let paths = possible_nav_paths(child);
                if !paths.is_empty() {
                    if subpaths.is_some() {
                        panic!("Multiple nav paths!");
                    }
                    subpaths = Some(paths);
                }
            }
            if let Some(subpaths) = subpaths {
                for subpath in subpaths {
                    result.push(route.iter().chain(&subpath).copied().collect());
                }
            } else {
                result.push(route);
            }
        }
        result
    }
}

fn children(el: El) -> Vec<Box<dyn Fn() -> El>> {
    match el.inner {
        ElInner::Static(data) => match data.children {
            Children::Fixed(items) => items.into_iter()
                .map(|i| Box::new(move || i()) as Box<dyn Fn() -> El>)
                .collect(),
            Children::StaticString(_) => Vec::new(),
            Children::String(_) => Vec::new(),
            Children::ForEach(content) => {
                let element = content.element.clone();
                (content.items)().into_iter()
                    .map(move |i| {
                        let element = element.clone();
                        Box::new(move || element.clone()(i.clone())) as Box<dyn Fn() -> El>
                    })
                    .collect()
            }
            Children::InnerHtml(_) => Vec::new()
        },
        ElInner::Flattened(f) => vec![Box::new(move || f())]
    }
}

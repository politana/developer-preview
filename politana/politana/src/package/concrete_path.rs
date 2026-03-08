use crate::library::nav_host::routes::PathComponent;

pub fn concrete_path(path: Vec<PathComponent>) -> Option<Vec<&'static str>> {
    path.into_iter()
        .map(|c| match c {
            PathComponent::Literal(comp) => Some(comp),
            PathComponent::Wildcard => None
        })
        .collect()
}

use crate::{Closure, El, library::nav_host::nav_controller::NavController};

#[derive(Clone)]
pub enum PathComponent {
    Literal(&'static str),
    Wildcard
}

#[derive(Clone)]
pub struct Route {
    pub path: Vec<PathComponent>,
    pub view: Closure<NavController, El>
}

#[derive(Clone)]
pub struct Routes(pub(crate) Vec<Route>);

impl Routes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn route(
        mut self,
        path: &'static str,
        view: impl Fn(NavController) -> El + 'static
    ) -> Self {
        let path = path.split("/")
            .filter(|s| !s.is_empty())
            .map(|comp| match comp {
                "*" => PathComponent::Wildcard,
                _ => PathComponent::Literal(comp)
            })
            .collect();
        self.0.push(Route {
            path: path,
            view: Closure::new(move |controller| view(controller))
        });
        self
    }
}

impl Route {
    pub fn matches_path(&self, path: &Vec<String>) -> bool {
        if self.path.is_empty() {
            path.is_empty()
        } else {
            path.len() >= self.path.len()
            && path.iter().zip(&self.path)
                .all(|(string_comp, path_comp)| match path_comp {
                    PathComponent::Literal(comp) => comp == string_comp,
                    PathComponent::Wildcard => true
                })
        }
    }
}

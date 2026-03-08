use crate::{El, library::{NavController, Routes, nav_host::nav_host_impl::NavigationHostImpl}};

#[cfg(target_arch = "wasm32")]
pub fn NavigationHost(
    routes: Routes,
    fallback: impl Fn(NavController) -> El + 'static
) -> El {
    NavigationHostImpl(routes, fallback)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn NavigationHost(
    routes: Routes,
    fallback: impl Fn(NavController) -> El + 'static
) -> El {
    NavigationHostImpl(routes.clone(), fallback)
        .possible_routes(&routes)
}

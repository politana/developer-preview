pub mod closure;
mod create_html_element;
pub mod current_nav_prefix;
pub mod deconstruct;
pub mod default_head_content;
pub mod environment;
mod generate_inspector_tree;
pub mod get_inspector_tree;
pub mod hash_eq_clone;
pub mod head_resources;
pub mod inert;
pub mod render;
#[cfg(target_arch = "wasm32")]
pub mod state;
pub mod style_manager;
mod update_ui;
mod vdom;
mod vdom_ref;
mod window_resize;

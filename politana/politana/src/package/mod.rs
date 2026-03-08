#![cfg(not(target_arch = "wasm32"))]

pub mod api;
mod concrete_path;
mod generate_static;
mod make_html;
mod make_wasm;
mod mini_style_manager;
pub mod mock_nav_path;
pub mod package;
mod possible_nav_paths;
mod serve_debug_site;

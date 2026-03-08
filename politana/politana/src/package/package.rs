use std::path::Path;

use crate::{El, debug_bar::ui::add_debug_bar::AddDebugBar, package::{generate_static::{StaticContent, generate_static}, make_html::{make_html, make_html_filename}, make_wasm::make_wasm, possible_nav_paths::possible_nav_paths, serve_debug_site::serve_debug_site}};

pub fn package(app: fn() -> El) {
    let is_debug = cfg!(debug_assertions);
    let app: Box<dyn Fn() -> El> = if is_debug {
        Box::new(|| AddDebugBar(app))
    } else { Box::new(|| app()) };
    let output_path = Path::new("dist");
    let wasm_output = make_wasm(is_debug, output_path);
    let mut possible_nav_paths = possible_nav_paths(&app);
    if possible_nav_paths.is_empty() {
        possible_nav_paths.push(Vec::new());
    }
    for path in possible_nav_paths {
        let static_content = generate_static(&app, path.clone());
        make_html(&wasm_output, static_content, output_path, path);
    }
    make_html_filename(
        &wasm_output,
        StaticContent {
            head: r#"<base href="/">"#.to_string(),
            body: "".to_string()
        },
        output_path,
        Vec::new(),
        "_fallback.html"
    );
    if is_debug {
        serve_debug_site(output_path);
    } else {
        println!("Generated project files at:\n    {}", output_path.display());
    }
}

use std::{fs, path::{Path, PathBuf}};

use crate::{package::{generate_static::StaticContent, make_wasm::WasmOutput}, reactivity::default_head_content::DEFAULT_HEAD_CONTENT};

pub fn make_html(
    wasm_output: &WasmOutput,
    static_content: StaticContent,
    output_path: &Path,
    output_subdirectory: Vec<&'static str>
) {
    make_html_filename(wasm_output, static_content, output_path, output_subdirectory, "index.html");
}

pub fn make_html_filename(
    wasm_output: &WasmOutput,
    static_content: StaticContent,
    output_path: &Path,
    output_subdirectory: Vec<&'static str>,
    filename: &'static str
) {
    let js_name = &wasm_output.js_filename;
    let wasm_name = &wasm_output.wasm_filename;
    let head_content = static_content.head;
    let body_content = static_content.body;
    let path_to_parent = "../".repeat(output_subdirectory.len());
    let html_content = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            {DEFAULT_HEAD_CONTENT}
            {head_content}
            <script type="module">
                import init from "./{path_to_parent}{js_name}";
                init({{ module_or_path: "./{path_to_parent}{wasm_name}" }}).catch(console.error);
            </script>
        </head>
        <body>
            {body_content}
        </body>
        </html>
    "#);

    let output_dir = output_path
        .join(output_subdirectory.iter().collect::<PathBuf>());
    fs::create_dir_all(&output_dir).expect("Failed to create output directory for HTML files");
    fs::write(output_dir.join(filename), html_content)
        .expect("Failed to write index.html");
}

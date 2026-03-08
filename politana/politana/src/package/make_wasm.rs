use std::{fs, path::Path, process::Command};

use cargo_metadata::{MetadataCommand, TargetKind};
use rand::{rng, seq::IndexedRandom};
use wasm_bindgen_cli_support::Bindgen;
use wasm_opt::OptimizationOptions;

pub struct WasmOutput {
    pub wasm_filename: String,
    pub js_filename: String
}

pub fn make_wasm(
    is_debug: bool,
    output_path: &Path
) -> WasmOutput {
    let target_name = std::env::current_exe()
        .expect("Failed to get current executable path")
        .file_stem()
        .expect("Failed to get executable name")
        .to_str().unwrap().to_string();
    let metadata = MetadataCommand::new().exec()
        .expect("Can't get current project metadata with Cargo");
    let (_, target) = metadata.packages.iter()
        .find_map(|p| {
            p.targets.iter().find(|t| {
                t.name == target_name &&
                (t.kind.contains(&TargetKind::Bin) || t.kind.contains(&TargetKind::Example))
            }).map(|t| (p, t))
        })
        .expect(&format!("Could not find a bin or example target named '{}' in the workspace", target_name));
    let is_example = target.kind.contains(&TargetKind::Example);
    let target_flag = if is_example { "--example" } else { "--bin" };
    let suffix = random_suffix();

    println!("----- Compiling project for WASM -----");
    let status = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg(target_flag)
        .arg(&target_name)
        .args(if is_debug { vec![] } else { vec!["--release"] })
        .status()
        .expect("Cannot run Cargo command. Is Cargo installed and available on the path?");
    if !status.success() {
        panic!("Cargo failed to build WASM artifacts for the binary");
    }

    if output_path.exists() {
        fs::remove_dir_all(output_path).expect("Cannot remove dist directory");
    }
    fs::create_dir_all(output_path).expect("Cannot create dist directory");

    println!("----- Generating output files -----");
    let profile = if is_debug { "debug" } else { "release" };
    let target_dir = metadata.target_directory.as_std_path();
    let mut wasm_source = target_dir
        .join("wasm32-unknown-unknown")
        .join(profile);
    if is_example {
        wasm_source = wasm_source.join("examples");
    }
    let wasm_source = wasm_source.join(format!("{}.wasm", target_name));
    Bindgen::new()
        .input_path(&wasm_source)
        .web(true)
        .expect("Issue building for web using wasm-bindgen")
        .generate(output_path)
        .expect("Issue building for web using wasm-bindgen");

    if !is_debug {
        let wasm_bg_path = output_path.join(format!("{}_bg.wasm", target_name));
        OptimizationOptions::new_opt_level_4()
            .run(&wasm_bg_path, &wasm_bg_path)
            .expect("Failed to run wasm-opt");
    }

    let js_name = format!("{}-{}.js", target_name, suffix);
    let wasm_name = format!("{}-{}.wasm", target_name, suffix);

    fs::rename(
        output_path.join(format!("{}.js", target_name)),
        output_path.join(&js_name)
    ).expect("Failed to rename .js file");
    fs::rename(
        output_path.join(format!("{}_bg.wasm", target_name)),
        output_path.join(&wasm_name)
    ).expect("Failed to rename .wasm file");

    WasmOutput { wasm_filename: wasm_name, js_filename: js_name }
}

fn random_suffix() -> String {
    let characters: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789POLITANA";
    (0..8)
        .map(|_| *characters.choose(&mut rng()).unwrap() as char)
        .collect()
}

use std::path::Path;

fn main() {
    let Ok(crate_dir) = std::env::var("CARGO_MANIFEST_DIR") else {
        return;
    };
    let workspace_root = Path::new(&crate_dir).join("../..");
    let config_path = workspace_root.join("cbindgen.toml");
    let output_path = workspace_root.join("target/app_core.h");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=../../cbindgen.toml");
    println!("cargo:rerun-if-changed=../ffi/src/");
    println!("cargo:rerun-if-changed=../errors/src/");
    println!("cargo:rerun-if-changed=../domain/src/");

    let config = match cbindgen::Config::from_file(&config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("cargo:warning=cbindgen config error: {e}, using defaults");
            cbindgen::Config::default()
        }
    };

    match cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
    {
        Ok(bindings) => {
            bindings.write_to_file(&output_path);
        }
        Err(e) => {
            eprintln!("cargo:warning=cbindgen generation failed: {e}");
        }
    }
}

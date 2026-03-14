use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../protos");
    println!("cargo:rerun-if-changed={}", proto_root.display());
    if !proto_root.exists() {
        return Ok(());
    }
    let protos = find_protos(&proto_root)?;
    for proto in &protos {
        println!("cargo:rerun-if-changed={}", proto.display());
    }
    if protos.is_empty() {
        return Ok(());
    }
    prost_build::Config::new().compile_protos(&protos, &[&proto_root])?;
    Ok(())
}

fn find_protos(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                result.extend(find_protos(&path)?);
            } else if path.extension().is_some_and(|e| e == "proto") {
                result.push(path);
            }
        }
    }
    Ok(result)
}

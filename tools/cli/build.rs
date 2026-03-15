use std::env;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let template_root = manifest_dir.join("../..").canonicalize().unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let output_file = out_dir.join("template_files.rs");

    let mut entries: Vec<(String, PathBuf, bool)> = Vec::new();
    walk_dir(&template_root, &template_root, &mut entries);
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut code = String::with_capacity(entries.len() * 200);
    writeln!(code, "struct TemplateFile {{").unwrap();
    writeln!(code, "    path: &'static str,").unwrap();
    writeln!(code, "    content: &'static [u8],").unwrap();
    writeln!(code, "    executable: bool,").unwrap();
    writeln!(code, "}}").unwrap();
    writeln!(code).unwrap();
    writeln!(code, "static TEMPLATE_FILES: &[TemplateFile] = &[").unwrap();

    for (rel_path, abs_path, executable) in &entries {
        let abs_str = abs_path.display().to_string().replace('\\', "/");
        writeln!(code, "    TemplateFile {{").unwrap();
        writeln!(code, "        path: {rel_path:?},").unwrap();
        writeln!(code, "        content: include_bytes!({abs_str:?}),").unwrap();
        writeln!(code, "        executable: {executable},").unwrap();
        writeln!(code, "    }},").unwrap();
    }

    writeln!(code, "];").unwrap();
    fs::write(&output_file, &code).unwrap();

    eprintln!("cargo:warning=Embedded {} template files", entries.len());

    println!("cargo:rerun-if-changed=../../CLAUDE.md");
    println!("cargo:rerun-if-changed=../../justfile");
    println!("cargo:rerun-if-changed=../../.gitignore");
    println!("cargo:rerun-if-changed=../../.editorconfig");
    println!("cargo:rerun-if-changed=../../protos");
    println!("cargo:rerun-if-changed=../../rust/crates");
    println!("cargo:rerun-if-changed=../../flutter");
    println!("cargo:rerun-if-changed=../../scripts");
    println!("cargo:rerun-if-changed=../../docs/architecture");
}

fn walk_dir(root: &Path, dir: &Path, entries: &mut Vec<(String, PathBuf, bool)>) {
    let Ok(read_dir) = fs::read_dir(dir) else {
        return;
    };
    let mut dir_entries: Vec<_> = read_dir.flatten().collect();
    dir_entries.sort_by_key(|e| e.file_name());

    for entry in dir_entries {
        let path = entry.path();
        let rel = path
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .to_string();

        if should_skip(&rel) {
            continue;
        }

        if path.is_dir() {
            walk_dir(root, &path, entries);
        } else if path.is_file() {
            let executable = rel.ends_with(".sh");
            let abs = path.canonicalize().unwrap();
            entries.push((rel, abs, executable));
        }
    }
}

fn should_skip(rel: &str) -> bool {
    let parts: Vec<&str> = rel.split('/').collect();

    for p in &parts {
        match *p {
            ".git" | "target" | ".dart_tool" | "build" | "generated" | "tools" | ".github" => {
                return true
            }
            _ => {}
        }
    }

    if rel.starts_with("docs/analysis") {
        return true;
    }

    if rel.ends_with(".lock") {
        return true;
    }

    if rel.ends_with(".DS_Store") || rel.ends_with("Thumbs.db") {
        return true;
    }
    if rel.ends_with(".swp") || rel.ends_with(".swo") || rel.ends_with('~') {
        return true;
    }
    if parts.iter().any(|p| *p == ".idea" || *p == ".vscode") {
        return true;
    }

    if rel == ".env" || rel.contains(".local.") {
        return true;
    }

    false
}

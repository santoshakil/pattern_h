use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

include!(concat!(env!("OUT_DIR"), "/template_files.rs"));

pub struct Config {
    pub name: String,
    pub org: String,
    pub seed_color: String,
}

impl Config {
    fn pascal_case(&self) -> String {
        self.name
            .split('_')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(ch) => ch.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect()
    }

    fn title_case(&self) -> String {
        self.name
            .split('_')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(ch) => ch.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn kebab_case(&self) -> String {
        self.name.replace('_', "-")
    }

    fn upper_snake(&self) -> String {
        self.name.to_uppercase()
    }

    fn channel_prefix(&self) -> String {
        format!("{}/{}", self.org, self.name)
    }

    fn include_guard(&self) -> String {
        format!("{}_CORE_H", self.upper_snake())
    }

    fn cdylib_name(&self) -> String {
        format!("{}_core", self.name)
    }

    fn cdylib_kebab(&self) -> String {
        self.cdylib_name().replace('_', "-")
    }
}

pub fn generate(config: &Config, output_dir: &str) -> io::Result<()> {
    let project_dir = Path::new(output_dir).join(&config.name);

    if project_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("directory already exists: {}", project_dir.display()),
        ));
    }

    let abs_path = fs::canonicalize(output_dir)
        .unwrap_or_else(|_| Path::new(output_dir).to_path_buf())
        .join(&config.name);

    println!("Creating {} from Pattern H skeleton...", config.name);
    println!("  Output: {}", abs_path.display());
    println!("  Org:    {}", config.org);
    println!("  Color:  0xFF{}", config.seed_color);
    println!();

    let mut count = 0;
    for file in TEMPLATE_FILES {
        let output_path = transform_path(file.path, config);
        let full_path = project_dir.join(&output_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = if is_text_file(file.path) {
            match std::str::from_utf8(file.content) {
                Ok(text) => apply_replacements(text, config).into_bytes(),
                Err(_) => file.content.to_vec(),
            }
        } else {
            file.content.to_vec()
        };

        fs::write(&full_path, &content)?;

        #[cfg(unix)]
        if file.executable {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&full_path, fs::Permissions::from_mode(0o755))?;
        }

        count += 1;
    }

    println!("  Extracted {count} files");

    println!("  Initializing git...");
    let git_ok = Command::new("git")
        .args(["init", "-q"])
        .current_dir(&project_dir)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if git_ok {
        let _ = Command::new("git")
            .args(["add", "."])
            .current_dir(&project_dir)
            .status();
        let _ = Command::new("git")
            .args(["commit", "-q", "-m", "Initial commit from Pattern H skeleton"])
            .current_dir(&project_dir)
            .status();
        println!("  Git initialized with initial commit");
    } else {
        println!("  Git not available, skipping init");
    }

    println!();
    println!("Project {} created!", config.name);
    println!();
    println!("Next steps:");
    println!("  cd {}", config.name);
    println!("  ./scripts/setup.sh");
    println!("  just check");

    Ok(())
}

fn transform_path(path: &str, config: &Config) -> String {
    path.replace("app_core", &config.cdylib_name())
}

fn is_text_file(path: &str) -> bool {
    let text_ext = [
        ".rs", ".toml", ".yaml", ".yml", ".dart", ".proto", ".md", ".sh",
    ];
    let text_names = [".gitignore", ".editorconfig", "justfile"];

    text_ext.iter().any(|ext| path.ends_with(ext))
        || text_names.iter().any(|name| {
            path == *name || path.ends_with(&format!("/{name}"))
        })
}

fn apply_replacements(content: &str, config: &Config) -> String {
    let replacements = [
        ("com.pattern_h", config.channel_prefix()),
        ("APP_CORE_H", config.include_guard()),
        ("app_core", config.cdylib_name()),
        ("app-core", config.cdylib_kebab()),
        ("Pattern H", config.title_case()),
        ("PatternH", config.pascal_case()),
        ("PATTERN_H", config.upper_snake()),
        ("pattern-h", config.kebab_case()),
        ("pattern_h", config.name.clone()),
    ];

    let mut result = content.to_string();
    for (from, to) in &replacements {
        result = result.replace(from, to);
    }

    if config.seed_color != "1A73E8" {
        result = result.replace("0xFF1A73E8", &format!("0xFF{}", config.seed_color));
    }

    result
}

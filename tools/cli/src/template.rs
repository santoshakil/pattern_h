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

    fn android_package(&self) -> String {
        format!("{}.{}", self.org, self.name)
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

    init_platforms(config, &project_dir);
    init_git(&project_dir);

    println!();
    println!("Project {} created!", config.name);
    println!();
    println!("Next steps:");
    println!("  cd {}", config.name);
    println!("  ./scripts/setup.sh");
    println!("  just check");

    Ok(())
}

fn init_platforms(config: &Config, project_dir: &Path) {
    let flutter_available = Command::new("flutter")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !flutter_available {
        println!("  Flutter not found, skipping platform init");
        println!("  Run ./scripts/init_platforms.sh later to add platform support");
        return;
    }

    let app_dir = project_dir.join(format!("flutter/apps/{}", config.name));

    println!("  Creating platform directories...");
    let create_ok = Command::new("flutter")
        .args([
            "create",
            "--platforms",
            "android,ios,macos,windows,linux",
            "--org",
            &config.org,
            "--project-name",
            &config.name,
            ".",
        ])
        .current_dir(&app_dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !create_ok {
        println!("  flutter create failed, skipping platform injection");
        return;
    }

    add_permissions(&app_dir);
    inject_platform_channels(config, &app_dir);
    println!("  Platform support ready");
}

fn init_git(project_dir: &Path) {
    println!("  Initializing git...");
    let git_ok = Command::new("git")
        .args(["init", "-q"])
        .current_dir(project_dir)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if git_ok {
        let _ = Command::new("git")
            .args(["add", "."])
            .current_dir(project_dir)
            .status();
        let _ = Command::new("git")
            .args(["commit", "-q", "-m", "Initial commit from Pattern H skeleton"])
            .current_dir(project_dir)
            .status();
        println!("  Git initialized with initial commit");
    } else {
        println!("  Git not available, skipping init");
    }
}

fn add_permissions(app_dir: &Path) {
    // Android: INTERNET + ACCESS_NETWORK_STATE
    let manifest = app_dir.join("android/app/src/main/AndroidManifest.xml");
    if manifest.exists() {
        if let Ok(content) = fs::read_to_string(&manifest) {
            if !content.contains("android.permission.INTERNET") {
                let patched = content.replacen(
                    "<manifest",
                    "<manifest xmlns:tools=\"http://schemas.android.com/tools\"",
                    1,
                );
                let patched = patched.replacen(
                    "<application",
                    "    <uses-permission android:name=\"android.permission.INTERNET\"/>\n    <uses-permission android:name=\"android.permission.ACCESS_NETWORK_STATE\"/>\n\n    <application",
                    1,
                );
                let _ = fs::write(&manifest, patched);
            }
        }
    }

    // macOS: network.client entitlement for both debug and release
    for name in &[
        "macos/Runner/Release.entitlements",
        "macos/Runner/DebugProfile.entitlements",
    ] {
        let path = app_dir.join(name);
        if let Ok(content) = fs::read_to_string(&path) {
            if !content.contains("network.client") {
                let patched = content.replacen(
                    "</dict>",
                    "\t<key>com.apple.security.network.client</key>\n\t<true/>\n</dict>",
                    1,
                );
                let _ = fs::write(&path, patched);
            }
        }
    }
}

fn inject_platform_channels(_config: &Config, app_dir: &Path) {
    let tpl = app_dir
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|root| root.join("templates/platform"))
        .unwrap_or_default();

    if !tpl.exists() {
        return;
    }

    // Android: copy channel files into the kotlin package
    if let Some(main_kt) = find_file(app_dir, "MainActivity.kt") {
        if let Some(pkg_dir) = main_kt.parent() {
            let channels_dir = pkg_dir.join("channels");
            let _ = fs::create_dir_all(&channels_dir);
            copy_tpl(&tpl.join("android/MainActivity.kt"), &main_kt);
            copy_tpl(
                &tpl.join("android/ChannelRegistry.kt"),
                &channels_dir.join("ChannelRegistry.kt"),
            );
            copy_tpl(
                &tpl.join("android/DeviceInfoChannel.kt"),
                &channels_dir.join("DeviceInfoChannel.kt"),
            );
        }
    }

    // iOS
    let ios_runner = app_dir.join("ios/Runner");
    if ios_runner.exists() {
        copy_tpl(
            &tpl.join("ios/AppDelegate.swift"),
            &ios_runner.join("AppDelegate.swift"),
        );
        copy_tpl(
            &tpl.join("ios/ChannelRegistry.swift"),
            &ios_runner.join("ChannelRegistry.swift"),
        );
        copy_tpl(
            &tpl.join("ios/DeviceInfoChannel.swift"),
            &ios_runner.join("DeviceInfoChannel.swift"),
        );
        add_swift_to_xcode(&app_dir.join("ios"), &["ChannelRegistry.swift", "DeviceInfoChannel.swift"]);
    }

    // macOS
    let macos_runner = app_dir.join("macos/Runner");
    if macos_runner.exists() {
        copy_tpl(
            &tpl.join("macos/AppDelegate.swift"),
            &macos_runner.join("AppDelegate.swift"),
        );
        copy_tpl(
            &tpl.join("macos/ChannelRegistry.swift"),
            &macos_runner.join("ChannelRegistry.swift"),
        );
        copy_tpl(
            &tpl.join("macos/DeviceInfoChannel.swift"),
            &macos_runner.join("DeviceInfoChannel.swift"),
        );
        add_swift_to_xcode(&app_dir.join("macos"), &["ChannelRegistry.swift", "DeviceInfoChannel.swift"]);
    }

    // Windows
    let win_runner = app_dir.join("windows/runner");
    if win_runner.exists() {
        for name in &[
            "channel_registry.h",
            "channel_registry.cpp",
            "device_info_channel.h",
            "device_info_channel.cpp",
        ] {
            copy_tpl(&tpl.join(format!("windows/{name}")), &win_runner.join(name));
        }
        patch_file(
            &win_runner.join("flutter_window.cpp"),
            "#include \"flutter/generated_plugin_registrant.h\"",
            "#include \"channel_registry.h\"\n#include \"flutter/generated_plugin_registrant.h\"",
        );
        patch_file(
            &win_runner.join("flutter_window.cpp"),
            "RegisterPlugins(flutter_controller_->engine());",
            "RegisterPlugins(flutter_controller_->engine());\n  RegisterChannels(flutter_controller_->engine());",
        );
        patch_file(
            &win_runner.join("CMakeLists.txt"),
            "\"flutter_window.cpp\"",
            "\"flutter_window.cpp\"\n  \"channel_registry.cpp\"\n  \"device_info_channel.cpp\"",
        );
    }

    // Linux
    let linux_runner = app_dir.join("linux/runner");
    if linux_runner.exists() {
        for name in &[
            "channel_registry.h",
            "channel_registry.cc",
            "device_info_channel.h",
            "device_info_channel.cc",
        ] {
            copy_tpl(&tpl.join(format!("linux/{name}")), &linux_runner.join(name));
        }
        patch_file(
            &linux_runner.join("my_application.cc"),
            "#include \"flutter/generated_plugin_registrant.h\"",
            "#include \"channel_registry.h\"\n#include \"flutter/generated_plugin_registrant.h\"",
        );
        patch_file(
            &linux_runner.join("my_application.cc"),
            "fl_register_plugins(FL_PLUGIN_REGISTRY(view));",
            "fl_register_plugins(FL_PLUGIN_REGISTRY(view));\n\n  register_channels(view);",
        );
        patch_file(
            &linux_runner.join("CMakeLists.txt"),
            "\"my_application.cc\"",
            "\"my_application.cc\"\n  \"channel_registry.cc\"\n  \"device_info_channel.cc\"",
        );
    }

    // Dart platform wrapper
    let platform_dir = app_dir.join("lib/platform");
    let _ = fs::create_dir_all(&platform_dir);
    copy_tpl(
        &tpl.join("dart/device_info.dart"),
        &platform_dir.join("device_info.dart"),
    );
}

fn copy_tpl(src: &Path, dst: &Path) {
    if let Ok(content) = fs::read_to_string(src) {
        let _ = fs::write(dst, content);
    }
}

fn patch_file(path: &Path, find: &str, replace: &str) {
    if let Ok(content) = fs::read_to_string(path) {
        if content.contains(find) && !content.contains(replace) {
            let _ = fs::write(path, content.replacen(find, replace, 1));
        }
    }
}

fn find_file(dir: &Path, name: &str) -> Option<std::path::PathBuf> {
    for entry in walkdir(dir) {
        if entry.file_name().map(|n| n == name).unwrap_or(false) {
            return Some(entry);
        }
    }
    None
}

fn walkdir(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut result = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                result.extend(walkdir(&path));
            } else {
                result.push(path);
            }
        }
    }
    result
}

fn add_swift_to_xcode(platform_dir: &Path, files: &[&str]) {
    let ruby_available = Command::new("ruby")
        .args(["-e", "require 'xcodeproj'"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !ruby_available {
        return;
    }

    let file_list = files
        .iter()
        .map(|f| format!("'{f}'"))
        .collect::<Vec<_>>()
        .join(", ");

    let script = format!(
        r#"
require 'xcodeproj'
proj = Xcodeproj::Project.open('{}/Runner.xcodeproj')
target = proj.targets.find {{ |t| t.name == 'Runner' }}
group = proj.main_group.find_subpath('Runner', true)
[{}].each do |name|
  next if group.files.any? {{ |f| f.path == name }}
  ref = group.new_file(name)
  target.source_build_phase.add_file_reference(ref)
end
proj.save
"#,
        platform_dir.display(),
        file_list
    );

    let _ = Command::new("ruby")
        .args(["-e", &script])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
}

fn transform_path(path: &str, config: &Config) -> String {
    path.replace("app_core", &config.cdylib_name())
        .replace("main_app", &config.name)
}

fn is_text_file(path: &str) -> bool {
    let text_ext = [
        ".rs", ".toml", ".yaml", ".yml", ".dart", ".proto", ".md", ".sh", ".kt", ".swift",
        ".cpp", ".cc", ".h", ".patch",
    ];
    let text_names = [".gitignore", ".editorconfig", "justfile"];

    text_ext.iter().any(|ext| path.ends_with(ext))
        || text_names.iter().any(|name| path == *name || path.ends_with(&format!("/{name}")))
}

fn apply_replacements(content: &str, config: &Config) -> String {
    let replacements = [
        ("{{channel_prefix}}", config.channel_prefix()),
        ("{{package}}", config.android_package()),
        ("com.pattern_h", config.channel_prefix()),
        ("APP_CORE_H", config.include_guard()),
        ("app_core", config.cdylib_name()),
        ("app-core", config.cdylib_kebab()),
        ("Pattern H", config.title_case()),
        ("PatternH", config.pascal_case()),
        ("PATTERN_H", config.upper_snake()),
        ("pattern-h", config.kebab_case()),
        ("main_app", config.name.clone()),
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

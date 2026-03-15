mod template;

use clap::Parser;
use std::process;

#[derive(Parser)]
#[command(
    name = "pattern-h",
    about = "Create Flutter+Rust hexagonal architecture projects from the Pattern H skeleton",
    version
)]
struct Cli {
    /// Project name in snake_case (e.g. my_restaurant_app)
    name: String,

    /// Organization domain
    #[arg(long, default_value = "com.example")]
    org: String,

    /// Material 3 seed color hex without 0x prefix (e.g. FF6B35)
    #[arg(long, default_value = "1A73E8")]
    seed_color: String,

    /// Output directory (project will be created as a subdirectory)
    #[arg(short, long, default_value = ".")]
    output: String,
}

fn main() {
    let cli = Cli::parse();

    if !is_valid_snake_case(&cli.name) {
        eprintln!("Error: name must be lowercase snake_case (a-z, 0-9, _), starting with a letter");
        process::exit(1);
    }

    if cli.seed_color.len() != 6 || !cli.seed_color.chars().all(|c| c.is_ascii_hexdigit()) {
        eprintln!("Error: seed-color must be a 6-character hex string (e.g. FF6B35)");
        process::exit(1);
    }

    let config = template::Config {
        name: cli.name,
        org: cli.org,
        seed_color: cli.seed_color.to_uppercase(),
    };

    if let Err(e) = template::generate(&config, &cli.output) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn is_valid_snake_case(s: &str) -> bool {
    !s.is_empty()
        && s.starts_with(|c: char| c.is_ascii_lowercase())
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && !s.ends_with('_')
        && !s.contains("__")
}

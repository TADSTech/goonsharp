/// GoonHub — the package manager, build system, and brain of GoonSharp.
///
/// Like cargo, but for goons. Commands:
///   goonhub new <name>     Create a new GoonSharp project
///   goonhub init           Initialize GoonSharp in current directory
///   goonhub build          Build the project
///   goonhub run            Build and run
///   goonhub add <dep>      Add a dependency
///   goonhub test           Run tests
///   goonhub publish        Publish to GoonHub registry
///   goonhub goon           Start an interactive goon session

use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "goonhub",
    version = "69.0.0",
    about = "GoonHub — the package manager for GoonSharp 🟣"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new GoonSharp project
    New {
        /// Project name
        name: String,
        /// Use library template instead of binary
        #[arg(long)]
        lib: bool,
    },
    /// Initialize GoonSharp in current directory
    Init {
        /// Use library template
        #[arg(long)]
        lib: bool,
    },
    /// Build the project
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Build and run the project
    Run {
        /// Build in release mode
        #[arg(long)]
        release: bool,
        /// Arguments to pass to the program
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Add a dependency to Goon.toml
    Add {
        /// Package name
        package: String,
        /// Version requirement
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Run tests
    Test {
        /// Test name filter
        filter: Option<String>,
    },
    /// Publish package to GoonHub registry
    Publish,
    /// Start an interactive goon REPL session
    Goon,
    /// Clean build artifacts
    Clean,
}

/// Goon.toml project manifest
#[derive(Serialize, Deserialize, Debug)]
struct GoonManifest {
    package: GoonPackage,
    #[serde(default)]
    dependencies: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GoonPackage {
    name: String,
    version: String,
    #[serde(default = "default_edition")]
    edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authors: Option<Vec<String>>,
}

fn default_edition() -> String {
    "2024".to_string()
}

fn print_banner() {
    eprintln!(
        "{}",
        r#"
   ██████   ██████   ██████  ███    ██ ██   ██ ██    ██ ██████
  ██       ██    ██ ██    ██ ████   ██ ██   ██ ██    ██ ██   ██
  ██   ███ ██    ██ ██    ██ ██ ██  ██ ███████ ██    ██ ██████
  ██    ██ ██    ██ ██    ██ ██  ██ ██ ██   ██ ██    ██ ██   ██
   ██████   ██████   ██████  ██   ████ ██   ██  ██████  ██████
"#
        .purple()
    );
}

fn find_manifest() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let manifest = dir.join("Goon.toml");
        if manifest.exists() {
            return Some(manifest);
        }
        if !dir.pop() {
            return None;
        }
    }
}

fn create_project(name: &str, path: &Path, is_lib: bool) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(path.join("src"))?;

    // Goon.toml
    let manifest = GoonManifest {
        package: GoonPackage {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            edition: "2024".to_string(),
            description: Some(format!("A goon-tier {} written in GoonSharp", if is_lib { "library" } else { "application" })),
            authors: None,
        },
        dependencies: std::collections::HashMap::new(),
    };
    let toml_str = toml::to_string_pretty(&manifest)?;
    fs::write(path.join("Goon.toml"), toml_str)?;

    // .gitignore
    fs::write(
        path.join(".gitignore"),
        "/target\n/goon_out\n*.goon_temp*\n",
    )?;

    // Source file
    if is_lib {
        fs::write(
            path.join("src/lib.goons"),
            r#"/// A goon-tier library.

goonpub goonsesh add(a: i32, b: i32) -> i32 {
    goonreturn a + b;
}

#[cfg(test)]
goonmod tests {
    goonsesh test_add() {
        goonconst result = add(69, 420);
        assert_eq!(result, 489);
    }
}
"#,
        )?;
    } else {
        fs::write(
            path.join("src/main.goons"),
            r#"/// Welcome to your first GoonSharp project!
/// Run with: goonhub run

goonsesh main() {
    goonprint!("welcome to the goon zone 🟣");

    goon counter = 0;
    goonloop (counter < 5) {
        goonprint!("goon level: {}", counter);
        counter += 1;
    }

    goonif (counter == 5) {
        goonprint!("max goon achieved. you're goated.");
    } goonelse {
        goonprint!("skill issue.");
    }
}
"#,
        )?;
    }

    Ok(())
}

fn build_project(manifest_path: &Path, release: bool) -> Result<PathBuf, ()> {
    let project_dir = manifest_path.parent().unwrap();

    // Read manifest
    let manifest_str = fs::read_to_string(manifest_path).map_err(|e| {
        eprintln!("{} couldn't read Goon.toml: {}", "error:".red().bold(), e);
    })?;
    let manifest: GoonManifest = toml::from_str(&manifest_str).map_err(|e| {
        eprintln!("{} invalid Goon.toml: {}", "error:".red().bold(), e);
    })?;

    // Find the main source file
    let main_file = project_dir.join("src/main.goons");
    let lib_file = project_dir.join("src/lib.goons");

    let source_file = if main_file.exists() {
        main_file
    } else if lib_file.exists() {
        lib_file
    } else {
        eprintln!(
            "{} no src/main.goons or src/lib.goons found",
            "error:".red().bold()
        );
        return Err(());
    };

    let src = fs::read_to_string(&source_file).map_err(|e| {
        eprintln!("{} couldn't read source: {}", "error:".red().bold(), e);
    })?;

    let filename = source_file.to_string_lossy().to_string();

    // Compile
    eprintln!(
        "  {} {} v{} ({})",
        if release { "Compiling" } else { "Building" }.green().bold(),
        manifest.package.name,
        manifest.package.version,
        project_dir.display()
    );

    let ast = goonsharp_parser::compile_to_ast(&src, &filename).map_err(|_| ())?;
    let rust_code = goonsharp_codegen::transpile(&ast);

    // Output directory
    let out_dir = project_dir.join(if release { "target/release" } else { "target/debug" });
    fs::create_dir_all(&out_dir).map_err(|e| {
        eprintln!("{} couldn't create output dir: {}", "error:".red().bold(), e);
    })?;

    let rs_path = out_dir.join(format!("{}.rs", manifest.package.name));
    let bin_name = if cfg!(target_os = "windows") {
        format!("{}.exe", manifest.package.name)
    } else {
        manifest.package.name.clone()
    };
    let bin_path = out_dir.join(&bin_name);

    fs::write(&rs_path, &rust_code).map_err(|e| {
        eprintln!("{} couldn't write temp file: {}", "error:".red().bold(), e);
    })?;

    let mut rustc_cmd = Command::new("rustc");
    rustc_cmd
        .arg(&rs_path)
        .arg("-o")
        .arg(&bin_path)
        .arg("--edition")
        .arg("2021");

    if release {
        rustc_cmd.arg("-O");
    }

    let compile = rustc_cmd.output().map_err(|_| {
        eprintln!("{} rustc not found", "error:".red().bold());
    })?;

    if !compile.status.success() {
        eprintln!("{} compilation failed", "error:".red().bold());
        let stderr = String::from_utf8_lossy(&compile.stderr);
        for line in stderr.lines() {
            eprintln!("    {}", line);
        }
        return Err(());
    }

    // Cleanup temp .rs
    let _ = fs::remove_file(&rs_path);

    eprintln!(
        "  {} {}: {}",
        "Finished".green().bold(),
        if release { "release" } else { "debug" },
        bin_path.display()
    );

    Ok(bin_path)
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, lib } => {
            print_banner();
            let path = PathBuf::from(&name);
            if path.exists() {
                eprintln!(
                    "{} directory '{}' already exists",
                    "error:".red().bold(),
                    name
                );
                std::process::exit(1);
            }
            match create_project(&name, &path, lib) {
                Ok(_) => {
                    eprintln!(
                        "  {} `{}` package",
                        "Created".green().bold(),
                        name
                    );
                    eprintln!("\n  To get started:\n");
                    eprintln!("    cd {}", name);
                    eprintln!("    goonhub run\n");
                    eprintln!("  Happy gooning! 🟣");
                }
                Err(e) => {
                    eprintln!("{} failed to create project: {}", "error:".red().bold(), e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Init { lib } => {
            print_banner();
            let cwd = std::env::current_dir().expect("can't get cwd");
            let name = cwd
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("goon_project")
                .to_string();

            if cwd.join("Goon.toml").exists() {
                eprintln!(
                    "{} Goon.toml already exists",
                    "error:".red().bold()
                );
                std::process::exit(1);
            }

            match create_project(&name, &cwd, lib) {
                Ok(_) => {
                    eprintln!(
                        "  {} GoonSharp project in {}",
                        "Initialized".green().bold(),
                        cwd.display()
                    );
                }
                Err(e) => {
                    eprintln!("{} {}", "error:".red().bold(), e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Build { release } => {
            print_banner();
            let manifest = find_manifest().unwrap_or_else(|| {
                eprintln!(
                    "{} could not find Goon.toml in this or any parent directory",
                    "error:".red().bold()
                );
                std::process::exit(1);
            });

            if build_project(&manifest, release).is_err() {
                std::process::exit(1);
            }
        }

        Commands::Run { release, args } => {
            print_banner();
            let manifest = find_manifest().unwrap_or_else(|| {
                eprintln!(
                    "{} could not find Goon.toml in this or any parent directory",
                    "error:".red().bold()
                );
                std::process::exit(1);
            });

            match build_project(&manifest, release) {
                Ok(bin_path) => {
                    eprintln!("  {} `{}`\n", "Running".green().bold(), bin_path.display());

                    let status = Command::new(&bin_path)
                        .args(&args)
                        .status()
                        .expect("failed to run binary");

                    if !status.success() {
                        std::process::exit(status.code().unwrap_or(1));
                    }
                }
                Err(_) => std::process::exit(1),
            }
        }

        Commands::Add { package, version } => {
            let manifest_path = find_manifest().unwrap_or_else(|| {
                eprintln!("{} could not find Goon.toml", "error:".red().bold());
                std::process::exit(1);
            });

            let mut manifest_str = fs::read_to_string(&manifest_path).expect("can't read Goon.toml");
            let ver = version.unwrap_or_else(|| "*".to_string());

            // Simple append to dependencies
            if !manifest_str.contains("[dependencies]") {
                manifest_str.push_str("\n[dependencies]\n");
            }
            manifest_str.push_str(&format!("{} = \"{}\"\n", package, ver));
            fs::write(&manifest_path, manifest_str).expect("can't write Goon.toml");

            eprintln!(
                "  {} {} = \"{}\" to dependencies",
                "Added".green().bold(),
                package,
                ver
            );
        }

        Commands::Test { filter: _ } => {
            eprintln!(
                "  {} goon testing framework coming soon™",
                "test:".yellow().bold()
            );
            eprintln!("  for now, write your tests in goonsesh and assert manually.");
        }

        Commands::Publish => {
            eprintln!(
                "  {} the GoonHub registry is under construction",
                "publish:".yellow().bold()
            );
            eprintln!("  until then, share your .goons files on GitHub like a normal person.");
        }

        Commands::Goon => {
            print_banner();
            eprintln!(
                "  {} interactive goon REPL",
                "starting".bright_purple().bold()
            );
            eprintln!("  type GoonSharp expressions and see them transpile in real-time.");
            eprintln!("  type '{}' to exit.\n", "quit".dimmed());

            let stdin = std::io::stdin();
            let mut line = String::new();
            loop {
                eprint!("{} ", "goon>".purple().bold());
                line.clear();
                if stdin.read_line(&mut line).is_err() || line.trim() == "quit" {
                    eprintln!("\n  {} the sesh has ended. goon again soon. 🟣", "bye:".purple());
                    break;
                }
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                // Wrap in a main function and try to compile
                let wrapped = format!("goonsesh main() {{ {} }}", input);
                match goonsharp_parser::compile_to_ast(&wrapped, "<repl>") {
                    Ok(ast) => {
                        let rust = goonsharp_codegen::transpile(&ast);
                        eprintln!("{}", "  → Rust:".dimmed());
                        for line in rust.lines() {
                            eprintln!("    {}", line.bright_white());
                        }
                    }
                    Err(_) => {
                        eprintln!("  {} couldn't parse that. try again, king.", "error:".red());
                    }
                }
            }
        }

        Commands::Clean => {
            let manifest = find_manifest().unwrap_or_else(|| {
                eprintln!("{} could not find Goon.toml", "error:".red().bold());
                std::process::exit(1);
            });
            let target = manifest.parent().unwrap().join("target");
            if target.exists() {
                fs::remove_dir_all(&target).expect("failed to clean target/");
            }
            eprintln!("  {} removed target/", "Cleaned".green().bold());
        }
    }
}

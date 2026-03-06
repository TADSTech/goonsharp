/// GoonSharp CLI — the official compiler for the ultimate meme language.
///
/// Usage:
///   goonsharp <file.goons>                 Compile and run
///   goonsharp build <file.goons>           Compile only
///   goonsharp check <file.goons>           Parse check only
///   goonsharp emit-rust <file.goons>       Show transpiled Rust
///   goonsharp fmt <file.goons>             Format (lol)

use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "goonsharp",
    version = "69.0.0",
    about = "GoonSharp — the ultimate shitpost programming language 🟣",
    long_about = "GoonSharp compiler. Transpiles .goons files to Rust, compiles, and runs them.\n\nBecause every language starts as a meme, but only the real ones ship."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// The .goons file to compile and run (shorthand for `goonsharp run`)
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile and run a .goons file
    Run {
        /// The .goons file to run
        file: PathBuf,
        /// Arguments to pass to the compiled program
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Compile a .goons file without running
    Build {
        /// The .goons file to compile
        file: PathBuf,
        /// Output binary path
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Check a .goons file for errors without compiling
    Check {
        /// The .goons file to check
        file: PathBuf,
    },
    /// Emit the transpiled Rust source code
    EmitRust {
        /// The .goons file to transpile
        file: PathBuf,
    },
    /// Format a .goons file (just kidding, it prints it back)
    Fmt {
        /// The .goons file to "format"
        file: PathBuf,
    },
}

fn print_banner() {
    eprintln!(
        "{}",
        r#"
   ██████   ██████   ██████  ███    ██ ███████ ██   ██  █████  ██████  ██████
  ██       ██    ██ ██    ██ ████   ██ ██      ██   ██ ██   ██ ██   ██ ██   ██
  ██   ███ ██    ██ ██    ██ ██ ██  ██ ███████ ███████ ███████ ██████  ██████
  ██    ██ ██    ██ ██    ██ ██  ██ ██      ██ ██   ██ ██   ██ ██   ██ ██
   ██████   ██████   ██████  ██   ████ ███████ ██   ██ ██   ██ ██   ██ ██
"#
        .purple()
    );
    eprintln!(
        "  {} {}\n",
        "v69.0.0".bright_purple(),
        "— the ultimate shitpost programming language".dimmed()
    );
}

fn read_goons_file(path: &PathBuf) -> Result<String, ()> {
    if !path.exists() {
        eprintln!(
            "{} file not found: {}",
            "goon error:".red().bold(),
            path.display()
        );
        return Err(());
    }

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext != "goons" {
        eprintln!(
            "{} only .goons files allowed here. this ain't Rust, bro.",
            "goon error:".red().bold()
        );
        return Err(());
    }

    fs::read_to_string(path).map_err(|e| {
        eprintln!(
            "{} couldn't read '{}': {}",
            "goon error:".red().bold(),
            path.display(),
            e
        );
    })
}

fn compile_and_run(file: &PathBuf, run: bool, output: Option<PathBuf>, release: bool, program_args: &[String]) -> Result<(), ()> {
    let filename = file.to_string_lossy().to_string();
    let src = read_goons_file(file)?;

    // Phase 1: Lex
    eprint!("{}", "  lexing...".dimmed());
    let (tokens, lex_errors) = goonsharp_parser::lex(&src);

    if !lex_errors.is_empty() {
        eprintln!(" {}", "FAILED".red().bold());
        goonsharp_parser::error::report_lex_errors(&src, &filename, lex_errors);
        return Err(());
    }
    eprintln!(" {}", "ok".green());

    let tokens = tokens.ok_or(())?;

    // Phase 2: Parse
    eprint!("{}", "  parsing...".dimmed());
    let (ast, parse_errors) = goonsharp_parser::parse(tokens, src.len());

    if !parse_errors.is_empty() {
        eprintln!(" {}", "FAILED".red().bold());
        goonsharp_parser::error::report_parse_errors(&src, &filename, parse_errors);
        return Err(());
    }
    eprintln!(" {}", "ok".green());

    let ast = ast.ok_or(())?;

    // Phase 3: Codegen
    eprint!("{}", "  transpiling...".dimmed());
    let rust_code = goonsharp_codegen::transpile(&ast);
    eprintln!(" {}", "ok".green());

    // Phase 4: Compile with rustc
    let temp_dir = std::env::temp_dir();
    let rs_path = temp_dir.join("goon_temp.rs");
    let has_explicit_output = output.is_some();
    let bin_path = output.unwrap_or_else(|| {
        if cfg!(target_os = "windows") {
            temp_dir.join("goon_temp.exe")
        } else {
            temp_dir.join("goon_temp")
        }
    });

    fs::write(&rs_path, &rust_code).map_err(|e| {
        eprintln!(
            "{} failed to write temp Rust file: {}",
            "goon error:".red().bold(),
            e
        );
    })?;

    eprint!("{}", "  compiling...".dimmed());
    let mut rustc_cmd = Command::new("rustc");
    rustc_cmd.arg(&rs_path).arg("-o").arg(&bin_path);

    if release {
        rustc_cmd.arg("-O");
    }

    // Add edition
    rustc_cmd.arg("--edition").arg("2021");

    let compile = rustc_cmd.output().map_err(|_| {
        eprintln!(
            "\n{} rustc not found. you DO have Rust installed... right?",
            "goon error:".red().bold()
        );
    })?;

    if !compile.status.success() {
        eprintln!(" {}", "FAILED".red().bold());
        eprintln!(
            "\n{}",
            "  the goon transpiler produced invalid Rust. here's the tea from rustc:"
                .yellow()
        );
        let stderr = String::from_utf8_lossy(&compile.stderr);
        for line in stderr.lines() {
            eprintln!("    {}", line);
        }
        eprintln!(
            "\n{}\n",
            "  (this is a GoonSharp compiler bug. the transpiler needs to goon harder)"
                .dimmed()
        );
        // Dump the generated Rust for debugging
        eprintln!("{}", "  generated Rust source:".dimmed());
        for (i, line) in rust_code.lines().enumerate() {
            eprintln!("    {:>4} | {}", i + 1, line);
        }
        let _ = fs::remove_file(&rs_path);
        return Err(());
    }
    eprintln!(" {}", "ok".green());

    // Cleanup temp .rs
    let _ = fs::remove_file(&rs_path);

    if !run {
        eprintln!(
            "\n  {} {}",
            "built:".green().bold(),
            bin_path.display()
        );
        return Ok(());
    }

    // Phase 5: Run
    eprintln!("{}", "  running...\n".dimmed());

    let status = Command::new(&bin_path)
        .args(program_args)
        .status()
        .map_err(|e| {
            eprintln!(
                "{} failed to run the goon binary: {}",
                "goon error:".red().bold(),
                e
            );
        })?;

    // Cleanup temp binary (only if no explicit output)
    if !has_explicit_output {
        let _ = fs::remove_file(&bin_path);
    }

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

fn main() {
    // chumsky's deeply-boxed recursive parsers need a larger stack
    let builder = std::thread::Builder::new()
        .name("goonsharp-main".into())
        .stack_size(64 * 1024 * 1024); // 64 MB

    let handler = builder
        .spawn(real_main)
        .expect("failed to spawn main thread");

    if let Err(e) = handler.join() {
        eprintln!("goon error: thread panicked: {:?}", e);
        std::process::exit(1);
    }
}

fn real_main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { file, args }) => {
            print_banner();
            if compile_and_run(&file, true, None, false, &args).is_err() {
                std::process::exit(1);
            }
        }
        Some(Commands::Build {
            file,
            output,
            release,
        }) => {
            print_banner();
            if compile_and_run(&file, false, output, release, &[]).is_err() {
                std::process::exit(1);
            }
        }
        Some(Commands::Check { file }) => {
            print_banner();
            let filename = file.to_string_lossy().to_string();
            let src = match read_goons_file(&file) {
                Ok(s) => s,
                Err(_) => std::process::exit(1),
            };

            let (tokens, lex_errors) = goonsharp_parser::lex(&src);
            if !lex_errors.is_empty() {
                goonsharp_parser::error::report_lex_errors(&src, &filename, lex_errors);
                std::process::exit(1);
            }

            if let Some(tokens) = tokens {
                let (_, parse_errors) = goonsharp_parser::parse(tokens, src.len());
                if !parse_errors.is_empty() {
                    goonsharp_parser::error::report_parse_errors(&src, &filename, parse_errors);
                    std::process::exit(1);
                }
            }

            eprintln!(
                "  {} no goon errors found. you're goated.",
                "check passed:".green().bold()
            );
        }
        Some(Commands::EmitRust { file }) => {
            let filename = file.to_string_lossy().to_string();
            let src = match read_goons_file(&file) {
                Ok(s) => s,
                Err(_) => std::process::exit(1),
            };

            match goonsharp_parser::compile_to_ast(&src, &filename) {
                Ok(ast) => {
                    let rust = goonsharp_codegen::transpile(&ast);
                    println!("{}", rust);
                }
                Err(_) => std::process::exit(1),
            }
        }
        Some(Commands::Fmt { file: _ }) => {
            eprintln!(
                "  {} goon code is already perfect. no formatting needed.",
                "fmt:".bright_purple().bold()
            );
            eprintln!("  (but seriously, this feature is coming soon™)");
        }
        None => {
            // If no subcommand but a file is provided, run it
            if let Some(file) = cli.file {
                print_banner();
                if compile_and_run(&file, true, None, false, &[]).is_err() {
                    std::process::exit(1);
                }
            } else {
                print_banner();
                eprintln!("  Usage: goonsharp <file.goons>");
                eprintln!("         goonsharp run <file.goons>");
                eprintln!("         goonsharp build <file.goons>");
                eprintln!("         goonsharp check <file.goons>");
                eprintln!("         goonsharp emit-rust <file.goons>");
                eprintln!();
                eprintln!("  Run `goonsharp --help` for more options.");
            }
        }
    }
}

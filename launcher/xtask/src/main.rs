//! `cargo xtask` — the one entry point for checks, tests, packaging and releases.
//!
//! Everything CI does is a subcommand here: GitHub Actions and Buildkite call the same commands,
//! so a green run locally means the same thing as a green run in CI. No shell scripts.
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("help");
    match command {
        "check" => check(),
        "test" => test(&args[1..]),
        "pytest" => pytest(&args[1..]),
        "gate" => gate(),
        "ui" => ui(&args[1..]),
        "help" | "--help" | "-h" => {
            help();
            ExitCode::SUCCESS
        }
        other => {
            eprintln!("xtask: unknown command `{other}`");
            help();
            ExitCode::FAILURE
        }
    }
}

fn help() {
    println!(
        "cargo xtask <command>

  check    cargo fmt --check and clippy -D warnings
  test     cargo nextest (or cargo test) for the launcher workspace
  pytest   uv run pytest for tests/
  gate     check + test + pytest (what CI runs)
  ui       run the launcher UI (pass --release for an optimized build)"
    );
}

/// The repository root: the cargo manifest is `launcher/xtask`, so two levels up.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("launcher/xtask is two levels below the root")
        .to_path_buf()
}

fn launcher() -> PathBuf {
    root().join("launcher")
}

fn run(program: &str, args: &[&str], dir: &Path) -> ExitCode {
    println!("$ {} {}", program, args.join(" "));
    match Command::new(program).args(args).current_dir(dir).status() {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => {
            eprintln!("{program} exited with {status}");
            ExitCode::FAILURE
        }
        Err(error) => {
            eprintln!("could not run {program}: {error}");
            ExitCode::FAILURE
        }
    }
}

fn check() -> ExitCode {
    let dir = launcher();
    for (program, args) in [
        ("cargo", vec!["fmt", "--all", "--check"]),
        (
            "cargo",
            vec![
                "clippy",
                // The core still carries pre-existing clippy debt; the new UI crates must be clean.
                "-p",
                "scoria-ui-kit",
                "-p",
                "scoria-ui",
                "-p",
                "xtask",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
        ),
    ] {
        let code = run(program, &args, &dir);
        if code != ExitCode::SUCCESS {
            return code;
        }
    }
    ExitCode::SUCCESS
}

fn test(extra: &[String]) -> ExitCode {
    let dir = launcher();
    let nextest = Command::new("cargo")
        .args(["nextest", "--version"])
        .current_dir(&dir)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false);
    let mut args: Vec<&str> = if nextest {
        vec!["nextest", "run", "--workspace"]
    } else {
        vec!["test", "--workspace"]
    };
    args.extend(extra.iter().map(String::as_str));
    run("cargo", &args, &dir)
}

fn pytest(extra: &[String]) -> ExitCode {
    let mut args: Vec<&str> = vec!["run", "--frozen", "pytest", "-q"];
    args.extend(extra.iter().map(String::as_str));
    run("uv", &args, &root())
}

fn gate() -> ExitCode {
    for step in [check, || test(&[]), || pytest(&[])] {
        let code = step();
        if code != ExitCode::SUCCESS {
            return code;
        }
    }
    ExitCode::SUCCESS
}

fn ui(extra: &[String]) -> ExitCode {
    let mut args: Vec<&str> = vec!["run", "-p", "scoria-ui"];
    if extra.iter().any(|arg| arg == "--release") {
        args.push("--release");
    }
    run("cargo", &args, &launcher())
}

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
        "build" => build(),
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
  build    cargo build -p scoria-ui (the binary the Python tests drive)
  gate     check + test + build + pytest (what CI runs)
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
    let code = run("cargo", &["fmt", "--all", "--check"], &dir);
    if code != ExitCode::SUCCESS {
        return code;
    }
    clippy(&dir)
}

/// Clippy on our crates only. `-D warnings` would also deny the engine's platform-specific
/// warnings through the path dependency, so the JSON diagnostics are filtered by target.
fn clippy(dir: &Path) -> ExitCode {
    println!("$ cargo clippy -p scoria-ui -p scoria-ui-kit -p xtask --no-deps --all-targets");
    let output = match Command::new("cargo")
        .args([
            "clippy",
            "-p",
            "scoria-ui",
            "-p",
            "scoria-ui-kit",
            "-p",
            "xtask",
            "--no-deps",
            "--all-targets",
            "--message-format=json",
        ])
        .current_dir(dir)
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            eprintln!("could not run cargo clippy: {error}");
            return ExitCode::FAILURE;
        }
    };
    if !output.status.success() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        return ExitCode::FAILURE;
    }

    let ours = ["scoria-ui", "scoria-ui-kit", "xtask"];
    let mut failed = false;
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Ok(message) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if message["reason"] != "compiler-message" {
            continue;
        }
        let target = message["target"]["name"].as_str().unwrap_or_default();
        let level = message["message"]["level"].as_str().unwrap_or_default();
        if !ours.contains(&target) || !matches!(level, "warning" | "error") {
            continue;
        }
        failed = true;
        let text = message["message"]["message"].as_str().unwrap_or_default();
        let span = &message["message"]["spans"][0];
        let file = span["file_name"].as_str().unwrap_or_default();
        let line = span["line_start"].as_u64().unwrap_or_default();
        eprintln!("{target} {file}:{line}: {level}: {text}");
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn test(extra: &[String]) -> ExitCode {
    let dir = launcher();
    let nextest = Command::new("cargo")
        .args(["nextest", "--version"])
        .current_dir(&dir)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false);
    // macOS keeps to the new crates: the engine's app-bundle test needs a real Apple app and
    // signing setup that this CI image does not provide, and the engine is not what this
    // rewrite changes. Linux and Windows run the whole workspace.
    let scoped = cfg!(target_os = "macos");
    let mut args: Vec<&str> = if nextest {
        vec!["nextest", "run"]
    } else {
        vec!["test"]
    };
    if scoped {
        args.extend(["-p", "scoria-ui", "-p", "scoria-ui-kit", "-p", "xtask"]);
    } else {
        args.push("--workspace");
    }
    args.extend(extra.iter().map(String::as_str));
    run("cargo", &args, &dir)
}

fn pytest(extra: &[String]) -> ExitCode {
    let mut args: Vec<&str> = vec!["run", "--frozen", "pytest", "-q"];
    args.extend(extra.iter().map(String::as_str));
    run("uv", &args, &root())
}

fn build() -> ExitCode {
    run("cargo", &["build", "-p", "scoria-ui"], &launcher())
}

fn gate() -> ExitCode {
    for step in [check, || test(&[]), build, || pytest(&[])] {
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

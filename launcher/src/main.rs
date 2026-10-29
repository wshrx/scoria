//! Scoria launcher engine: installs builds, prepares them from the player's own files, and
//! serves the JSON API the desktop UI talks to.
//!
//! The legacy GPUI launcher was removed: Scoria's UI is `scoria-launcher-ui`, and this binary is
//! the engine behind it (`horizon-launcher api`). The CLI stays for scripts and diagnostics.
mod cli;
mod cli_tools;
mod console;
mod game_tools;
mod mcp;
mod perf;

// `perf` and the CLI refer to the shared engine through `crate::backend`, as the old root did.
use horizon_launcher::backend;

fn main() {
    if let Err(error) = console::attach_for_cli() {
        eprintln!("Could not restore the launcher's terminal streams: {error}");
        std::process::exit(1);
    }
    perf::process_started();
    horizon_launcher::backend::exe_path();
    // What the launcher downloads goes through the player's proxy, if they set one.
    horizon_launcher::proxy::announce(horizon_launcher::backend::Settings::load().proxy);
    if let Some(code) = cli::run() {
        std::process::exit(code);
    }
    eprintln!(
        "Scoria's window is scoria-launcher-ui. This binary is the engine: `horizon-launcher api`."
    );
    std::process::exit(2);
}

//! Developer-only Mimas harness. Off unless the `dev-scripts` feature is built and
//! `SCORIA_DEV_SCRIPTS` points at a folder; a player build never loads scripts.
//!
//! A script is any `*.mim` file in that folder. The window compiles them at each hook and calls
//! the one named after the event. Host functions queue actions for the window; a script cannot
//! touch the file system or the engine directly.
use std::{path::Path, sync::Mutex};

use mimas::mimas;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Toast(String),
    Log(String),
    Play,
    Download,
    Screen(String),
    /// Apply one of the synthetic `SCORIA_UI_STATE` fixtures.
    State(String),
    /// Show a synthetic download at `percent` (0..100) with a log line.
    Progress(f64, String),
    Cancel,
    Stop,
}

static ACTIONS: Mutex<Vec<Action>> = Mutex::new(Vec::new());

fn push(action: Action) {
    if let Ok(mut queue) = ACTIONS.lock() {
        queue.push(action);
    }
}

#[mimas]
fn toast(message: &str) {
    push(Action::Toast(message.to_string()));
}

#[mimas]
fn log(message: &str) {
    push(Action::Log(message.to_string()));
}

#[mimas]
fn play() {
    push(Action::Play);
}

#[mimas]
fn download() {
    push(Action::Download);
}

#[mimas]
fn screen(name: &str) {
    push(Action::Screen(name.to_string()));
}

/// Apply a synthetic QA state, e.g. `state("home-playing")`.
#[mimas]
fn state(name: &str) {
    push(Action::State(name.to_string()));
}

/// Draw a synthetic download. `percent` is 0..100; this does not touch the network.
#[mimas]
fn progress(percent: f64, line: &str) {
    push(Action::Progress(percent, line.to_string()));
}

/// Cancel a real or synthetic prepare.
#[mimas]
fn cancel() {
    push(Action::Cancel);
}

/// Stop a running game.
#[mimas]
fn stop() {
    push(Action::Stop);
}

/// Compile every `*.mim` in `dir` and call `hook` if a script defines it.
pub fn run(hook: &str, dir: &Path) -> Result<Vec<Action>, String> {
    let mut sources = Vec::new();
    for entry in std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().is_some_and(|extension| extension == "mim") {
            let source =
                std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            sources.push((path.display().to_string(), source));
        }
    }
    if sources.is_empty() {
        return Ok(Vec::new());
    }
    run_source(
        &sources
            .iter()
            .map(|(_, source)| source.as_str())
            .collect::<Vec<_>>()
            .join("\n"),
        hook,
    )
}

fn run_source(source: &str, hook: &str) -> Result<Vec<Action>, String> {
    let mut vm = mimas::compile_source(source).map_err(|error| format!("{error:?}"))?;
    vm.run().map_err(|error| format!("{error:?}"))?;
    if let Ok(mut queue) = ACTIONS.lock() {
        queue.clear();
    }
    match vm.call::<()>(hook, ()) {
        Ok(()) => {}
        Err(error) => {
            let message = format!("{error:?}");
            // A script without this hook is normal; any other fault is reported to the window.
            if !message.contains(&format!("no fn `{hook}` is reachable")) {
                return Err(format!("{hook}: {message}"));
            }
        }
    }
    Ok(std::mem::take(
        &mut *ACTIONS.lock().map_err(|e| e.to_string())?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_script_can_queue_actions_for_the_window() {
        let actions = run_source(
            r#"fn on_start() {
                toast("hello from mimas");
                log("script ran");
                screen("mods");
            }"#,
            "on_start",
        )
        .unwrap();
        assert_eq!(
            actions,
            vec![
                Action::Toast("hello from mimas".to_string()),
                Action::Log("script ran".to_string()),
                Action::Screen("mods".to_string()),
            ]
        );
    }

    #[test]
    fn the_download_examples_compile() {
        for example in [
            include_str!("../../dev/scripts/download-demo.mim"),
            include_str!("../../dev/scripts/download-real.mim"),
            include_str!("../../dev/scripts/download-flow.mim"),
        ] {
            run_source(example, "on_start").unwrap();
        }
    }

    #[test]
    fn a_script_can_drive_a_synthetic_download() {
        let actions = run_source(
            r#"fn on_start() {
                progress(42.0, "Downloading Minecraft 26.3 from Mojang");
            }"#,
            "on_start",
        )
        .unwrap();
        assert_eq!(
            actions,
            vec![Action::Progress(
                42.0,
                "Downloading Minecraft 26.3 from Mojang".to_string()
            )]
        );
    }

    #[test]
    fn an_unknown_hook_is_not_an_error() {
        assert!(
            run_source(r#"fn on_start() { toast("x"); }"#, "on_game_exit")
                .unwrap()
                .is_empty()
        );
    }
}

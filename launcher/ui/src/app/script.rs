//! Developer-only Mimas harness. Off unless the `dev-scripts` feature is built and
//! `SCORIA_DEV_SCRIPTS` points at a folder; a player build never loads scripts.
//!
//! A script is any `*.mim` file in that folder. The window compiles them at each hook and calls
//! the one named after the event. Host functions queue actions for the window; a script cannot
//! touch the file system or the engine directly.
use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
    path::Path,
    sync::Mutex,
};

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
    Install(String),
    AddServer(String, String),
    RemoveServer(usize),
    SelectOffline(String),
    SetAppearance(String),
    SetLanguage(String),
    Refresh,
}

static ACTIONS: Mutex<Vec<Action>> = Mutex::new(Vec::new());

#[derive(Clone, Default)]
struct State {
    prepared: bool,
    playing: bool,
    progress_done: f64,
    progress_total: f64,
    progress_line: String,
    last_ok: bool,
    last_error: String,
    last_project: String,
    screen: String,
    account_kind: String,
    account_name: String,
    server_name: String,
    server_ip: String,
    server_index: f64,
    setting_key: String,
    setting_value: String,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn with_state<T>(fallback: T, f: impl FnOnce(&State) -> T) -> T {
    STATE
        .lock()
        .ok()
        .and_then(|guard| guard.as_ref().map(f))
        .unwrap_or(fallback)
}

fn update_state(f: impl FnOnce(&mut State)) {
    if let Ok(mut guard) = STATE.lock() {
        f(guard.get_or_insert_with(State::default));
    }
}

/// The snapshot facts a developer script may branch on.
pub fn set_state(prepared: bool, playing: bool) {
    update_state(|state| {
        state.prepared = prepared;
        state.playing = playing;
    });
}

pub fn set_prepare(done: f64, total: f64, line: &str) {
    update_state(|state| {
        state.progress_done = done;
        state.progress_total = total;
        state.progress_line = line.to_string();
    });
}

pub fn set_result(project: &str, ok: bool, error: &str) {
    update_state(|state| {
        state.last_project = project.to_string();
        state.last_ok = ok;
        state.last_error = error.to_string();
    });
}

pub fn set_prepare_result(ok: bool, error: &str) {
    update_state(|state| {
        state.last_ok = ok;
        state.last_error = error.to_string();
    });
}

pub fn set_screen(screen: &str) {
    update_state(|state| state.screen = screen.to_string());
}

pub fn set_account(kind: &str, name: &str) {
    update_state(|state| {
        state.account_kind = kind.to_string();
        state.account_name = name.to_string();
    });
}

pub fn set_server(name: &str, ip: &str, index: usize) {
    update_state(|state| {
        state.server_name = name.to_string();
        state.server_ip = ip.to_string();
        state.server_index = index as f64;
    });
}

pub fn set_setting(key: &str, value: &str) {
    update_state(|state| {
        state.setting_key = key.to_string();
        state.setting_value = value.to_string();
    });
}

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
fn prepared() -> bool {
    with_state(false, |state| state.prepared)
}

#[mimas]
fn playing() -> bool {
    with_state(false, |state| state.playing)
}

#[mimas]
fn prepare_done() -> f64 {
    with_state(0.0, |state| state.progress_done)
}

#[mimas]
fn prepare_total() -> f64 {
    with_state(0.0, |state| state.progress_total)
}

#[mimas]
fn prepare_line() -> String {
    with_state(String::new(), |state| state.progress_line.clone())
}

#[mimas]
fn last_ok() -> bool {
    with_state(false, |state| state.last_ok)
}

#[mimas]
fn last_error() -> String {
    with_state(String::new(), |state| state.last_error.clone())
}

#[mimas]
fn last_project() -> String {
    with_state(String::new(), |state| state.last_project.clone())
}

#[mimas]
fn screen_name() -> String {
    with_state(String::new(), |state| state.screen.clone())
}

#[mimas]
fn account_kind() -> String {
    with_state(String::new(), |state| state.account_kind.clone())
}

#[mimas]
fn account_name() -> String {
    with_state(String::new(), |state| state.account_name.clone())
}

#[mimas]
fn server_name() -> String {
    with_state(String::new(), |state| state.server_name.clone())
}

#[mimas]
fn server_ip() -> String {
    with_state(String::new(), |state| state.server_ip.clone())
}

#[mimas]
fn server_index() -> f64 {
    with_state(0.0, |state| state.server_index)
}

#[mimas]
fn setting_key() -> String {
    with_state(String::new(), |state| state.setting_key.clone())
}

#[mimas]
fn setting_value() -> String {
    with_state(String::new(), |state| state.setting_value.clone())
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

#[mimas]
fn install(project: &str) {
    push(Action::Install(project.to_string()));
}

#[mimas]
fn add_server(name: &str, ip: &str) {
    push(Action::AddServer(name.to_string(), ip.to_string()));
}

#[mimas]
fn remove_server(index: f64) {
    push(Action::RemoveServer(index.max(0.0) as usize));
}

#[mimas]
fn select_offline(name: &str) {
    push(Action::SelectOffline(name.to_string()));
}

#[mimas]
fn set_appearance(value: &str) {
    push(Action::SetAppearance(value.to_string()));
}

#[mimas]
fn set_language(value: &str) {
    push(Action::SetLanguage(value.to_string()));
}

#[mimas]
fn refresh() {
    push(Action::Refresh);
}

/// A cheap fingerprint of the script folder: newest modification and file count.
pub fn stamp(dir: &Path) -> Option<(std::time::SystemTime, usize)> {
    let mut newest = None;
    let mut count = 0;
    for entry in std::fs::read_dir(dir).ok()? {
        let path = entry.ok()?.path();
        if path.extension().is_some_and(|extension| extension == "mim") {
            count += 1;
            if let Ok(modified) = std::fs::metadata(&path).and_then(|meta| meta.modified()) {
                newest = Some(newest.map_or(modified, |current: std::time::SystemTime| {
                    current.max(modified)
                }));
            }
        }
    }
    newest.map(|time| (time, count))
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
    let actions = run_cached(
        &sources
            .iter()
            .map(|(_, source)| source.as_str())
            .collect::<Vec<_>>()
            .join("\n"),
        hook,
    )?;
    let allowed = capabilities(dir);
    Ok(actions
        .into_iter()
        .filter(|action| {
            allowed
                .as_ref()
                .is_none_or(|allowed| allowed.contains(action.name()))
        })
        .collect())
}

fn capabilities(dir: &Path) -> Option<HashSet<String>> {
    let text = std::fs::read_to_string(dir.join("capabilities")).ok()?;
    let mut allowed = HashSet::new();
    for line in text.lines() {
        let line = line.trim();
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if key.trim() != "allow" {
            continue;
        }
        for action in value.split(',') {
            let action = action.trim();
            if !action.is_empty() {
                allowed.insert(action.to_string());
            }
        }
    }
    (!allowed.is_empty()).then_some(allowed)
}

impl Action {
    fn name(&self) -> &'static str {
        match self {
            Action::Toast(_) => "toast",
            Action::Log(_) => "log",
            Action::Play => "play",
            Action::Download => "download",
            Action::Screen(_) => "screen",
            Action::State(_) => "state",
            Action::Progress(_, _) => "progress",
            Action::Cancel => "cancel",
            Action::Stop => "stop",
            Action::Install(_) => "install",
            Action::AddServer(_, _) => "add_server",
            Action::RemoveServer(_) => "remove_server",
            Action::SelectOffline(_) => "select_offline",
            Action::SetAppearance(_) => "set_appearance",
            Action::SetLanguage(_) => "set_language",
            Action::Refresh => "refresh",
        }
    }
}

thread_local! {
    /// The last compiled folder, reused while its source is unchanged. A Mimas VM is not
    /// `Send`; hooks run on the window's thread, so a thread-local cache is enough.
    static CACHE: RefCell<Option<(u64, mimas::Vm)>> = const { RefCell::new(None) };
}

fn hash_source(source: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    hasher.finish()
}

/// Compile once per source version, then call hooks on the cached VM.
fn run_cached(source: &str, hook: &str) -> Result<Vec<Action>, String> {
    let hash = hash_source(source);
    CACHE.with(|cell| {
        let mut cache = cell.borrow_mut();
        if cache.as_ref().is_some_and(|(cached, _)| *cached == hash) {
            let vm = &mut cache.as_mut().expect("checked above").1;
            return call_vm(vm, hook);
        }
        let mut vm = mimas::compile_source(source).map_err(|error| format!("{error:?}"))?;
        vm.run().map_err(|error| format!("{error:?}"))?;
        let result = call_vm(&mut vm, hook);
        if result.is_ok() {
            *cache = Some((hash, vm));
        }
        result
    })
}

fn call_vm(vm: &mut mimas::Vm, hook: &str) -> Result<Vec<Action>, String> {
    if let Ok(mut queue) = ACTIONS.lock() {
        queue.clear();
    }
    match vm.call::<()>(hook, ()) {
        Ok(()) => {}
        Err(error) => {
            let message = format!("{error:?}");
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
fn run_source(source: &str, hook: &str) -> Result<Vec<Action>, String> {
    // Scripts run one at a time: hooks share the action queue, and tests run in parallel.
    static RUN_LOCK: Mutex<()> = Mutex::new(());
    let _guard = RUN_LOCK
        .lock()
        .map_err(|_| "script lock poisoned".to_string())?;
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
            include_str!("../../dev/scripts/logic.mim"),
            include_str!("../../dev/scripts/hooks-all.mim"),
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
    fn a_script_can_branch_on_the_snapshot() {
        set_state(true, false);
        let actions = run_source(
            r#"fn on_start() {
                if prepared() {
                    toast("ready");
                } else {
                    toast("missing");
                }
            }"#,
            "on_start",
        )
        .unwrap();
        assert_eq!(actions, vec![Action::Toast("ready".to_string())]);
        set_state(false, false);
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

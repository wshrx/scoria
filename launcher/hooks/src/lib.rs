//! Scoria's developer hook engine.
//!
//! A typed event/command library, independent of GPUI:
//!
//! * the launcher sends an [`Event`] (`prepare_done`, `screen_change`, …);
//! * every `.mim` file in the scripts folder gets the matching hook;
//! * scripts call typed host functions and return [`Command`]s;
//! * the launcher applies the commands.
//!
//! Each script is compiled separately, so one broken file does not stop the others. The library
//! owns the OS watcher, the per-file VM cache and the capability filter. Tokens and secrets never
//! enter this crate: scripts only see public state and queue actions.
//!
//! A player build does not depend on this crate; the window pulls it in with `dev-scripts`.

use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver},
};

use mimas::mimas;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

/// One event the launcher sends to the scripts.
#[derive(Clone, Debug)]
pub enum Event {
    Start,
    Reload,
    PrepareStart,
    PrepareProgress {
        done: f64,
        total: f64,
        line: String,
    },
    PrepareDone {
        ok: bool,
        error: String,
    },
    InstallStart {
        project: String,
    },
    InstallDone {
        project: String,
        ok: bool,
        error: String,
    },
    ServerAdd {
        name: String,
        ip: String,
        index: usize,
    },
    ServerRemove {
        index: usize,
    },
    AccountSwitch {
        kind: String,
        name: String,
    },
    ScreenChange {
        screen: String,
    },
    SettingsChange {
        key: String,
        value: String,
    },
    GameStart,
    GameExit,
}

impl Event {
    fn hooks(&self) -> &'static [&'static str] {
        match self {
            Self::Start => &["on_start"],
            Self::Reload => &["on_reload"],
            Self::PrepareStart => &["on_prepare_start"],
            Self::PrepareProgress { .. } => &["on_prepare_progress"],
            // `on_download_done` is the name earlier scripts used.
            Self::PrepareDone { .. } => &["on_prepare_done", "on_download_done"],
            Self::InstallStart { .. } => &["on_install_start"],
            Self::InstallDone { .. } => &["on_install_done"],
            Self::ServerAdd { .. } => &["on_server_add"],
            Self::ServerRemove { .. } => &["on_server_remove"],
            Self::AccountSwitch { .. } => &["on_account_switch"],
            Self::ScreenChange { .. } => &["on_screen_change"],
            Self::SettingsChange { .. } => &["on_settings_change"],
            Self::GameStart => &["on_game_start"],
            Self::GameExit => &["on_game_exit"],
        }
    }

    fn apply(&self, state: &mut State) {
        match self {
            Self::PrepareStart => {
                state.progress_done = 0.0;
                state.progress_total = 1.0;
                state.progress_line = "Starting…".into();
            }
            Self::PrepareProgress { done, total, line } => {
                state.progress_done = *done;
                state.progress_total = *total;
                state.progress_line = line.clone();
            }
            Self::PrepareDone { ok, error } => {
                state.last_ok = *ok;
                state.last_error = error.clone();
            }
            Self::InstallDone { project, ok, error } => {
                state.last_project = project.clone();
                state.last_ok = *ok;
                state.last_error = error.clone();
            }
            Self::InstallStart { project } => {
                state.last_project = project.clone();
                state.last_ok = false;
                state.last_error.clear();
            }
            Self::ServerAdd { name, ip, index } => {
                state.server_name = name.clone();
                state.server_ip = ip.clone();
                state.server_index = *index as f64;
            }
            Self::ServerRemove { index } => {
                state.server_name.clear();
                state.server_ip.clear();
                state.server_index = *index as f64;
            }
            Self::AccountSwitch { kind, name } => {
                state.account_kind = kind.clone();
                state.account_name = name.clone();
            }
            Self::ScreenChange { screen } => state.screen = screen.clone(),
            Self::SettingsChange { key, value } => {
                state.setting_key = key.clone();
                state.setting_value = value.clone();
            }
            Self::Start | Self::Reload | Self::GameStart | Self::GameExit => {}
        }
    }
}

/// An action a script asks the launcher to perform.
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Toast(String),
    Log(String),
    Play,
    Download,
    Screen(String),
    State(String),
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

impl Command {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Toast(_) => "toast",
            Self::Log(_) => "log",
            Self::Play => "play",
            Self::Download => "download",
            Self::Screen(_) => "screen",
            Self::State(_) => "state",
            Self::Progress(_, _) => "progress",
            Self::Cancel => "cancel",
            Self::Stop => "stop",
            Self::Install(_) => "install",
            Self::AddServer(_, _) => "add_server",
            Self::RemoveServer(_) => "remove_server",
            Self::SelectOffline(_) => "select_offline",
            Self::SetAppearance(_) => "set_appearance",
            Self::SetLanguage(_) => "set_language",
            Self::Refresh => "refresh",
        }
    }
}

/// What a dispatch produced.
#[derive(Clone, Debug, Default)]
pub struct Dispatch {
    pub commands: Vec<Command>,
    /// One message per broken script; the other scripts still ran.
    pub errors: Vec<String>,
}

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

thread_local! {
    static STATE: RefCell<State> = const { RefCell::new(State::new()) };
    static ACTIONS: RefCell<Vec<Command>> = const { RefCell::new(Vec::new()) };
}

impl State {
    const fn new() -> Self {
        Self {
            prepared: false,
            playing: false,
            progress_done: 0.0,
            progress_total: 0.0,
            progress_line: String::new(),
            last_ok: false,
            last_error: String::new(),
            last_project: String::new(),
            screen: String::new(),
            account_kind: String::new(),
            account_name: String::new(),
            server_name: String::new(),
            server_ip: String::new(),
            server_index: 0.0,
            setting_key: String::new(),
            setting_value: String::new(),
        }
    }
}

struct Script {
    path: PathBuf,
    hash: u64,
    vm: Option<mimas::Vm>,
    error: Option<String>,
}

impl Script {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            hash: 0,
            vm: None,
            error: None,
        }
    }

    fn ensure(&mut self) {
        let source = match std::fs::read_to_string(&self.path) {
            Ok(source) => source,
            Err(error) => {
                self.error = Some(format!("{}: {error}", self.path.display()));
                return;
            }
        };
        let hash = hash_source(&source);
        if self.vm.is_some() && self.hash == hash {
            return;
        }
        match mimas::compile_source(&source) {
            Ok(mut vm) => match vm.run() {
                Ok(()) => {
                    self.hash = hash;
                    self.vm = Some(vm);
                    self.error = None;
                }
                Err(error) => {
                    self.error = Some(format!("{}: {error:?}", self.path.display()));
                    self.vm = None;
                }
            },
            Err(error) => {
                self.error = Some(format!("{}: {error:?}", self.path.display()));
                self.vm = None;
            }
        }
    }
}

/// The owned script host: folder, script VMs, capabilities and watcher.
pub struct Hooks {
    dir: PathBuf,
    scripts: Vec<Script>,
    capabilities: Option<HashSet<String>>,
    watcher: Option<RecommendedWatcher>,
    prepared: bool,
    playing: bool,
}

impl Hooks {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        let dir = dir.into();
        let mut hooks = Self {
            capabilities: capabilities(&dir),
            dir,
            scripts: Vec::new(),
            watcher: None,
            prepared: false,
            playing: false,
        };
        hooks.refresh();
        hooks
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Keep the public snapshot the getters expose in sync with the window.
    pub fn set_snapshot(&mut self, prepared: bool, playing: bool) {
        self.prepared = prepared;
        self.playing = playing;
    }

    /// Re-scan the folder. Existing VMs survive if their source hash is unchanged.
    pub fn refresh(&mut self) {
        let mut paths = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|extension| extension == "mim") {
                    paths.push(path);
                }
            }
        }
        paths.sort();
        let mut scripts = Vec::with_capacity(paths.len());
        for path in paths {
            let existing = self.scripts.iter().position(|script| script.path == path);
            scripts.push(match existing {
                Some(index) => self.scripts.remove(index),
                None => Script::new(path),
            });
        }
        self.scripts = scripts;
        self.capabilities = capabilities(&self.dir);
    }

    /// Dispatch one event to every script and collect the surviving commands.
    pub fn dispatch(&mut self, event: Event) -> Dispatch {
        self.refresh();
        let mut state = STATE.with(|state| state.borrow().clone());
        state.prepared = self.prepared;
        state.playing = self.playing;
        event.apply(&mut state);
        STATE.with(|slot| *slot.borrow_mut() = state);

        let mut commands = Vec::new();
        let mut errors = Vec::new();
        for script in &mut self.scripts {
            script.ensure();
            if let Some(error) = &script.error {
                errors.push(error.clone());
                continue;
            }
            let Some(vm) = script.vm.as_mut() else {
                continue;
            };
            for hook in event.hooks() {
                ACTIONS.with(|actions| actions.borrow_mut().clear());
                match vm.call::<()>(hook, ()) {
                    Ok(()) => {}
                    Err(error) => {
                        let message = format!("{error:?}");
                        if !message.contains(&format!("no fn `{hook}` is reachable")) {
                            errors.push(format!("{}: {hook}: {message}", script.path.display()));
                        }
                    }
                }
                let mut actions =
                    ACTIONS.with(|actions| std::mem::take(&mut *actions.borrow_mut()));
                commands.append(&mut actions);
            }
        }
        if let Some(allowed) = &self.capabilities {
            commands.retain(|command| allowed.contains(command.name()));
        }
        Dispatch { commands, errors }
    }

    /// Start watching the folder. The returned receiver fires after `.mim` changes; the caller
    /// owns the debounce timer.
    pub fn watch(&mut self) -> Option<Receiver<()>> {
        let (sender, receiver) = mpsc::channel();
        let mut watcher =
            notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
                let Ok(event) = event else { return };
                if event
                    .paths
                    .iter()
                    .any(|path| path.extension().is_some_and(|extension| extension == "mim"))
                {
                    let _ = sender.send(());
                }
            })
            .ok()?;
        watcher.watch(&self.dir, RecursiveMode::NonRecursive).ok()?;
        self.watcher = Some(watcher);
        Some(receiver)
    }
}

fn capabilities(dir: &Path) -> Option<HashSet<String>> {
    let text = std::fs::read_to_string(dir.join("capabilities")).ok()?;
    let mut allowed = HashSet::new();
    for line in text.lines() {
        let Some((key, value)) = line.trim().split_once('=') else {
            continue;
        };
        if key.trim() != "allow" {
            continue;
        }
        for command in value.split(',') {
            let command = command.trim();
            if !command.is_empty() {
                allowed.insert(command.to_string());
            }
        }
    }
    (!allowed.is_empty()).then_some(allowed)
}

fn hash_source(source: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    hasher.finish()
}

/* Host functions ---------------------------------------------------------------- */

fn push(command: Command) {
    ACTIONS.with(|actions| actions.borrow_mut().push(command));
}

#[mimas]
fn toast(message: &str) {
    push(Command::Toast(message.to_string()));
}

#[mimas]
fn log(message: &str) {
    push(Command::Log(message.to_string()));
}

#[mimas]
fn play() {
    push(Command::Play);
}

#[mimas]
fn download() {
    push(Command::Download);
}

#[mimas]
fn screen(name: &str) {
    push(Command::Screen(name.to_string()));
}

#[mimas]
fn state(name: &str) {
    push(Command::State(name.to_string()));
}

#[mimas]
fn progress(percent: f64, line: &str) {
    push(Command::Progress(percent, line.to_string()));
}

#[mimas]
fn cancel() {
    push(Command::Cancel);
}

#[mimas]
fn stop() {
    push(Command::Stop);
}

#[mimas]
fn install(project: &str) {
    push(Command::Install(project.to_string()));
}

#[mimas]
fn add_server(name: &str, ip: &str) {
    push(Command::AddServer(name.to_string(), ip.to_string()));
}

#[mimas]
fn remove_server(index: f64) {
    push(Command::RemoveServer(index.max(0.0) as usize));
}

#[mimas]
fn select_offline(name: &str) {
    push(Command::SelectOffline(name.to_string()));
}

#[mimas]
fn set_appearance(value: &str) {
    push(Command::SetAppearance(value.to_string()));
}

#[mimas]
fn set_language(value: &str) {
    push(Command::SetLanguage(value.to_string()));
}

#[mimas]
fn refresh() {
    push(Command::Refresh);
}

#[mimas]
fn prepared() -> bool {
    STATE.with(|state| state.borrow().prepared)
}

#[mimas]
fn playing() -> bool {
    STATE.with(|state| state.borrow().playing)
}

#[mimas]
fn prepare_done() -> f64 {
    STATE.with(|state| state.borrow().progress_done)
}

#[mimas]
fn prepare_total() -> f64 {
    STATE.with(|state| state.borrow().progress_total)
}

#[mimas]
fn prepare_line() -> String {
    STATE.with(|state| state.borrow().progress_line.clone())
}

#[mimas]
fn last_ok() -> bool {
    STATE.with(|state| state.borrow().last_ok)
}

#[mimas]
fn last_error() -> String {
    STATE.with(|state| state.borrow().last_error.clone())
}

#[mimas]
fn last_project() -> String {
    STATE.with(|state| state.borrow().last_project.clone())
}

#[mimas]
fn screen_name() -> String {
    STATE.with(|state| state.borrow().screen.clone())
}

#[mimas]
fn account_kind() -> String {
    STATE.with(|state| state.borrow().account_kind.clone())
}

#[mimas]
fn account_name() -> String {
    STATE.with(|state| state.borrow().account_name.clone())
}

#[mimas]
fn server_name() -> String {
    STATE.with(|state| state.borrow().server_name.clone())
}

#[mimas]
fn server_ip() -> String {
    STATE.with(|state| state.borrow().server_ip.clone())
}

#[mimas]
fn server_index() -> f64 {
    STATE.with(|state| state.borrow().server_index)
}

#[mimas]
fn setting_key() -> String {
    STATE.with(|state| state.borrow().setting_key.clone())
}

#[mimas]
fn setting_value() -> String {
    STATE.with(|state| state.borrow().setting_value.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_broken_script_does_not_hide_the_others() {
        let dir = std::env::temp_dir().join(format!("scoria-hooks-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("good.mim"), "fn on_start() { toast(\"ok\"); }").unwrap();
        std::fs::write(dir.join("bad.mim"), "fn on_start( {").unwrap();
        let mut hooks = Hooks::new(&dir);
        let result = hooks.dispatch(Event::Start);
        assert_eq!(result.commands, vec![Command::Toast("ok".into())]);
        assert!(!result.errors.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }
}

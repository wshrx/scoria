//! The launcher's state and the actions every screen shares: prepare, play, install, settings.
//! Screens and their drawing live in `screens`, the synthetic QA states in this file.
mod screens;
#[cfg(feature = "dev-scripts")]
mod script;
mod watch;

use std::{
    fs::OpenOptions,
    io::Write,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use gpui::{
    actions, div, ease_out_quint, img, linear_color_stop, linear_gradient, prelude::*, px, rgb,
    rgba, Animation, AnimationExt, Context, Image, ImageFormat, ImageSource, IntoElement,
    MouseButton, ObjectFit, Render, ResizeEdge, Window, WindowControlArea,
};

actions!(
    scoria,
    [
        ShowHome,
        ShowMods,
        ShowServers,
        ShowSettings,
        RunGame,
        CloseOverlay
    ]
);
use gpui_ce_elements::editable_text::{EditableTextState, StringStorage};
use scoria_core::account::{Accounts, Player};
use scoria_core::backend::{Appearance, Settings};
use scoria_core::i18n::Language;
use scoria_ui_kit::{
    badge, card, clamp_fraction, empty_state, error_panel, page_header, segment, soft_card, toast,
    tone_for_support, Button, ButtonSize, ButtonVariant, Theme, Tone,
};
use serde_json::json;

use crate::assets::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModsSource {
    Horizon,
    Modrinth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Screen {
    Home,
    Mods,
    Servers,
    Settings,
}

impl Screen {
    fn icon(self) -> &'static [u8] {
        match self {
            Screen::Home => ICON_PLAY,
            Screen::Mods => ICON_MODS,
            Screen::Servers => ICON_SERVERS,
            Screen::Settings => ICON_SETTINGS,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Screen::Home => "Play",
            Screen::Mods => "Mods",
            Screen::Servers => "Servers",
            Screen::Settings => "Settings",
        }
    }
}

/// Horizon and Modrinth as two tabs above the search field.
fn catalogue_tabs(shell: &Shell, cx: &mut Context<Shell>) -> impl IntoElement {
    let theme = shell.theme;
    let mut tabs = div()
        .flex()
        .self_start()
        .gap(px(2.0))
        .p(px(3.0))
        .rounded(px(theme.radii.lg))
        .bg(if theme.dark {
            rgba(0xFFFFFF08)
        } else {
            rgba(0x1D15100A)
        });
    for (id, source, label) in [
        ("tab-horizon", ModsSource::Horizon, "Horizon"),
        ("tab-modrinth", ModsSource::Modrinth, "Modrinth"),
    ] {
        let selected = shell.mods_source == source;
        let on_click = cx.listener(move |this, _: &gpui::ClickEvent, _window, cx| {
            this.mods_source = source;
            cx.notify();
        });
        tabs = tabs.child(segment(theme, id, label, selected, on_click));
    }
    tabs
}

/// One row of a dropdown: the selected option is tinted and marked.
fn menu_item(
    theme: Theme,
    id: &'static str,
    label: String,
    selected: bool,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_between()
        .h(px(34.0))
        .px(px(theme.spacing.md))
        .rounded(px(theme.radii.md))
        .border_1()
        .border_color(theme.colors.border)
        .bg(if selected {
            theme.colors.accent_soft
        } else {
            theme.colors.surface
        })
        .text_color(if selected {
            theme.colors.accent
        } else {
            theme.colors.foreground
        })
        .cursor_pointer()
        .on_click(on_click)
        .child(label)
        .children(selected.then(|| div().text_size(px(theme.type_scale.caption)).child("✓")))
}

/// A segmented-control option: the selected one carries the accent.
/// The engine's mode strings as the static keys the translation table uses.
fn mode_key(mode: &str) -> &'static str {
    match mode {
        "Creative" => "Creative",
        "Hardcore" => "Hardcore",
        "Adventure" => "Adventure",
        _ => "Survival",
    }
}

fn label(theme: &Theme, text: &str) -> impl IntoElement {
    div()
        .text_size(px(theme.type_scale.subtitle))
        .child(text.to_string())
}

fn qa_snapshot(prepared: bool, playing: bool, worlds: usize) -> serde_json::Value {
    let worlds: Vec<serde_json::Value> = (0..worlds)
        .map(|index| {
            let at = index.min(2);
            let name = ["New World", "Build", "Hardcore Run"][at];
            let mode = ["Survival", "Creative", "Hardcore"][at];
            let last_played = ["2 h ago", "3 days ago", "last week"][at];
            let age_ms = [2 * 3600, 3 * 86400, 7 * 86400][at] * 1000;
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_millis() as u64)
                .unwrap_or(0);
            json!({
                "folder": format!("world-{index}"),
                "name": name,
                "mode": mode,
                "lastPlayed": last_played,
                "lastPlayedAt": now.saturating_sub(age_ms),
            })
        })
        .collect();
    json!({
        "version": "v0.1.2-beta-game",
        "prepared": prepared,
        "playing": playing,
        "worlds": worlds,
        "offline": "Player",
    })
}

struct Toast {
    id: u64,
    tone: Tone,
    message: String,
}

pub(crate) struct Shell {
    theme: Theme,
    screen: Screen,
    settings: Settings,
    accounts: Accounts,
    name: gpui::Entity<EditableTextState>,
    name_message: Option<(bool, String)>,
    snapshot: Result<serde_json::Value, String>,
    play_message: Option<(bool, String)>,
    mods_query: gpui::Entity<EditableTextState>,
    mods_results: Vec<serde_json::Value>,
    mods_error: Option<String>,
    mods_loading: bool,
    installing: Option<String>,
    preparing: bool,
    prepare_status: Option<String>,
    prepare_percent: Option<f32>,
    mods_source: ModsSource,
    appearance_open: bool,
    language_open: bool,
    fade: f32,
    polling: bool,
    servers: Result<serde_json::Value, String>,
    /// The world whose poster Home shows; None means the newest world.
    selected_world: Option<String>,
    /// Keeps the filesystem watcher alive while the window is open.
    watcher: Option<watch::Keep>,
    server_name: gpui::Entity<EditableTextState>,
    server_ip: gpui::Entity<EditableTextState>,
    toasts: Vec<Toast>,
    toast_id: u64,
    /// A script action must not start the same hook again.
    #[cfg(feature = "dev-scripts")]
    script_running: bool,
}

impl Shell {
    pub(crate) fn new(cx: &mut Context<Self>, screen: Screen, query: Option<String>) -> Self {
        let settings = Settings::load();
        let theme = match settings.appearance {
            Appearance::Light => Theme::light(),
            _ => Theme::dark(),
        };
        let accounts = Accounts::load();
        let current = match accounts.player() {
            Player::Offline(name) => name,
            Player::GameName => String::new(),
        };
        let name = cx.new(|cx| EditableTextState::new(StringStorage::from(current), cx));
        let mut shell = Self {
            theme,
            screen,
            settings,
            accounts,
            name,
            name_message: None,
            snapshot: scoria_core::api::snapshot(),
            play_message: None,
            mods_query: cx.new(|cx| EditableTextState::new(StringStorage::default(), cx)),
            mods_results: Vec::new(),
            mods_error: None,
            mods_loading: false,
            installing: None,
            preparing: false,
            prepare_status: None,
            prepare_percent: None,
            mods_source: ModsSource::Horizon,
            appearance_open: false,
            language_open: false,
            fade: 0.0,
            polling: false,
            servers: scoria_core::api::saved_servers(),
            selected_world: None,
            watcher: None,
            server_name: cx.new(|cx| EditableTextState::new(StringStorage::default(), cx)),
            server_ip: cx.new(|cx| EditableTextState::new(StringStorage::default(), cx)),
            toasts: Vec::new(),
            toast_id: 0,
            #[cfg(feature = "dev-scripts")]
            script_running: false,
        };
        let qa_state = std::env::var("SCORIA_UI_STATE").ok();
        if let Some(state) = &qa_state {
            shell.apply_qa_state(state);
        }
        if qa_state.is_none() && screen == Screen::Mods && query.is_none() {
            shell.search_mods(cx);
        }
        if qa_state.is_none() && std::env::var("SCORIA_AUTO_PREPARE").is_ok() {
            shell.prepare(cx);
        }
        if qa_state.is_none() && shell.playing_now() {
            shell.poll_playing(cx);
        }
        if let Some(query) = query {
            if !query.is_empty() {
                shell
                    .mods_query
                    .update(cx, |state, cx| state.emplace(&query, cx));
            }
            shell.search_mods(cx);
        }
        if qa_state.is_none() {
            shell.watch_instance(cx);
            shell.run_scripts("on_start", cx);
        }
        shell
    }

    /// Synthetic data for the screenshot/QA matrix. The launcher never persists it.
    fn apply_qa_state(&mut self, state: &str) {
        self.screen = Screen::Home;
        self.play_message = None;
        self.preparing = false;
        self.prepare_percent = None;
        self.prepare_status = None;
        self.mods_loading = false;
        self.mods_error = None;
        self.mods_results.clear();
        self.snapshot = Ok(qa_snapshot(false, false, 0));

        match state {
            "home-empty" | "no-version" => {
                self.snapshot = Err("No version is installed yet.".to_string());
            }
            "home-not-prepared" => {
                self.snapshot = Ok(qa_snapshot(false, false, 2));
            }
            "home-preparing" => {
                self.snapshot = Ok(qa_snapshot(false, false, 2));
                self.preparing = true;
                self.prepare_percent = Some(0.42);
                self.prepare_status = Some("Downloading Minecraft 26.3 from Mojang".to_string());
            }
            "home-prepared" => {
                self.snapshot = Ok(qa_snapshot(true, false, 3));
            }
            "home-playing" => {
                self.snapshot = Ok(qa_snapshot(true, true, 3));
            }
            "home-error" => {
                self.snapshot = Ok(qa_snapshot(true, false, 3));
                self.play_message = Some((
                    false,
                    "Could not start the game: no display is available.".to_string(),
                ));
            }
            "mods-empty" => {
                self.screen = Screen::Mods;
            }
            "mods-loading" => {
                self.screen = Screen::Mods;
                self.mods_loading = true;
            }
            "mods-error" => {
                self.screen = Screen::Mods;
                self.mods_error =
                    Some("No connection: https://api.modrinth.com timed out.".to_string());
            }
            "mods-results" => {
                self.screen = Screen::Mods;
                self.mods_results = vec![
                    json!({"id":"zoom","title":"Zoom","summary":"Hold a key to zoom in; the mouse wheel zooms further while you hold it.","author":"Horizon","support":"Works","source":"Horizon","installable":true}),
                    json!({"id":"worldedit","title":"WorldEdit","summary":"Edit thousands of blocks at once with brushes, patterns and schematics.","author":"Horizon","support":"Works","source":"Horizon","installable":true}),
                    json!({"id":"sodium","title":"Sodium","summary":"A modern rendering engine that improves frame rates and reduces micro-stutter.","author":"jellysquid3","support":"Partly","source":"Modrinth","installable":true}),
                    json!({"id":"fabric-api","title":"Fabric API","summary":"Common hooks and intercompatibility measures for Fabric mods.","author":"modmuss50","support":"Partly","source":"Modrinth","installable":true}),
                ];
            }
            "servers-empty" => {
                self.screen = Screen::Servers;
                self.servers = Ok(json!({"servers": []}));
            }
            "servers-list" => {
                self.screen = Screen::Servers;
                self.servers = Ok(json!({"servers": [
                    {"index": 0, "name": "SMP", "ip": "play.example.net"},
                    {"index": 1, "name": "Creative", "ip": "127.0.0.1:25565"}
                ]}));
            }
            "servers-error" => {
                self.screen = Screen::Servers;
                self.servers = Err("No connection: could not read servers.json".to_string());
            }
            "settings" => {
                self.screen = Screen::Settings;
                self.settings.appearance = Appearance::Dark;
                self.theme = Theme::dark();
                self.accounts.offline = vec!["Player".to_string(), "Builder".to_string()];
                self.accounts.offline_selected = Some("Player".to_string());
            }
            "settings-light" => {
                self.screen = Screen::Settings;
                self.settings.appearance = Appearance::Light;
                self.theme = Theme::light();
                self.accounts.offline = vec!["Player".to_string(), "Builder".to_string()];
                self.accounts.offline_selected = Some("Player".to_string());
            }
            "home-light" => {
                self.settings.appearance = Appearance::Light;
                self.theme = Theme::light();
                self.snapshot = Ok(qa_snapshot(true, false, 3));
            }
            "compact" => {
                self.settings.rail_collapsed = true;
                self.snapshot = Ok(qa_snapshot(true, false, 2));
            }
            "small" | "wide" => {
                self.snapshot = Ok(qa_snapshot(true, false, 3));
            }
            _ => {}
        }
    }

    fn support_label(&self, support: &str) -> String {
        match support {
            "Works" => self.t("Works"),
            "Partly" => self.t("Partly"),
            "Doesn't run" => self.t("Doesn't run"),
            other => other.to_string(),
        }
    }

    fn language(&self) -> Language {
        self.settings.language.unwrap_or_else(Language::system)
    }

    fn t(&self, english: &'static str) -> String {
        scoria_core::i18n::tr_in(self.language(), english).to_string()
    }

    /// A world's last-played time in the window's language, from the timestamp the snapshot
    /// carries. Older snapshots only have the English sentence, which is used as a fallback.
    fn ago(&self, world: &serde_json::Value) -> String {
        let at = world["lastPlayedAt"].as_u64().unwrap_or(0);
        if at == 0 {
            return world["lastPlayed"].as_str().unwrap_or("").to_string();
        }
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as u64)
            .unwrap_or(at);
        let seconds = now.saturating_sub(at) / 1000;
        let (value, unit) = if seconds < 60 {
            (0, "now")
        } else if seconds < 3600 {
            (seconds / 60, "min")
        } else if seconds < 86_400 {
            (seconds / 3600, "hour")
        } else if seconds < 604_800 {
            (seconds / 86_400, "day")
        } else if seconds < 2_592_000 {
            (seconds / 604_800, "week")
        } else if seconds < 31_536_000 {
            (seconds / 2_592_000, "month")
        } else {
            (seconds / 31_536_000, "year")
        };
        let unit = match (self.language(), unit) {
            (Language::Russian, "min") => "мин",
            (Language::Russian, "hour") => "ч",
            (Language::Russian, "day") => "дн",
            (Language::Russian, "week") => "нед",
            (Language::Russian, "month") => "мес",
            (Language::Russian, "year") => "г.",
            (Language::French, "min") => "min",
            (Language::French, "hour") => "h",
            (Language::French, "day") => "j",
            (Language::French, "week") => "sem",
            (Language::French, "month") => "mois",
            (Language::French, "year") => "an",
            (_, "hour") => "h",
            (_, "day") => "d",
            (_, "week") => "w",
            (_, "month") => "mo",
            (_, "year") => "y",
            (_, other) => other,
        };
        match (self.language(), value) {
            (Language::Russian, 0) => "только что".to_string(),
            (Language::Russian, n) => format!("{n} {unit} назад"),
            (Language::French, 0) => "à l'instant".to_string(),
            (Language::French, n) => format!("il y a {n} {unit}"),
            (_, 0) => "just now".to_string(),
            (_, n) => format!("{n} {unit} ago"),
        }
    }

    /// A runtime string (an engine error or log line) through the same table: a known line is
    /// translated, an unknown one stays as it is.
    fn td(&self, text: &str) -> String {
        scoria_core::i18n::tr_in(self.language(), text).to_string()
    }

    /// `english` in the current language with its `{name}` holes filled.
    fn tf(&self, english: &'static str, args: &[(&str, &str)]) -> String {
        let mut out = self.t(english);
        for (name, value) in args {
            out = out.replace(&format!("{{{name}}}"), value);
        }
        out
    }

    /// Refresh Home and Servers when the game (or a dev build) writes a save, mod, pack or
    /// `servers.json`. The watcher stays alive in `self.watcher`.
    fn watch_instance(&mut self, cx: &mut Context<Self>) {
        if self.watcher.is_some() {
            return;
        }
        let instance = scoria_core::instances::selected();
        let data = scoria_core::backend::data_dir();
        let Some((keep, changes)) = watch::start(&instance.home, &data) else {
            return;
        };
        self.watcher = Some(keep);
        cx.spawn(async move |this, cx| loop {
            cx.background_executor().timer(watch::DEBOUNCE).await;
            let mut changed = false;
            while changes.try_recv().is_ok() {
                changed = true;
            }
            if !changed {
                continue;
            }
            let alive = this.update(cx, |this, cx| {
                this.snapshot = scoria_core::api::snapshot();
                this.refresh_servers();
                cx.notify();
            });
            if alive.is_err() {
                break;
            }
        })
        .detach();
    }

    /// Run one Mimas hook, then apply what the script asked for. The guard stops a script's
    /// `play()` from running `on_game_start` again.
    #[cfg(feature = "dev-scripts")]
    fn run_scripts(&mut self, hook: &str, cx: &mut Context<Self>) {
        if self.script_running {
            return;
        }
        self.script_running = true;
        let Some(dir) = std::env::var_os("SCORIA_DEV_SCRIPTS").map(std::path::PathBuf::from) else {
            self.script_running = false;
            return;
        };
        let prepared = self
            .snapshot
            .as_ref()
            .ok()
            .and_then(|snapshot| snapshot["prepared"].as_bool())
            .unwrap_or(false);
        script::set_state(prepared, self.playing_now());
        let result = script::run(hook, &dir);
        match result {
            Ok(actions) => {
                for action in actions {
                    match action {
                        script::Action::Toast(message) => {
                            self.show_toast(Tone::Neutral, message, cx)
                        }
                        script::Action::Log(message) => self.log_line(&message),
                        script::Action::Play => self.game_action(cx),
                        script::Action::Download => self.prepare(cx),
                        script::Action::Screen(name) => {
                            let screen = match name.as_str() {
                                "mods" => Screen::Mods,
                                "servers" => Screen::Servers,
                                "settings" => Screen::Settings,
                                _ => Screen::Home,
                            };
                            self.go(screen, cx);
                        }
                        script::Action::State(name) => {
                            self.apply_qa_state(&name);
                            cx.notify();
                        }
                        script::Action::Progress(percent, line) => {
                            self.apply_qa_state("home-prepared");
                            self.preparing = true;
                            self.prepare_percent = Some((percent / 100.0).clamp(0.0, 1.0) as f32);
                            self.prepare_status = Some(line);
                            cx.notify();
                        }
                        script::Action::Cancel => {
                            scoria_core::api::cancel_prepare();
                            self.preparing = false;
                            self.prepare_percent = None;
                            self.prepare_status = None;
                            cx.notify();
                        }
                        script::Action::Stop => self.stop_game(cx),
                    }
                }
            }
            Err(error) => self.show_toast(Tone::Danger, format!("script: {error}"), cx),
        }
        self.script_running = false;
    }

    #[cfg(not(feature = "dev-scripts"))]
    fn run_scripts(&mut self, _hook: &str, _cx: &mut Context<Self>) {}

    fn go(&mut self, screen: Screen, cx: &mut Context<Self>) {
        self.screen = screen;
        match screen {
            Screen::Home => self.snapshot = scoria_core::api::snapshot(),
            Screen::Servers => self.refresh_servers(),
            Screen::Mods if self.mods_results.is_empty() && !self.mods_loading => {
                self.search_mods(cx)
            }
            _ => {}
        }
        cx.notify();
    }

    /// The one game action: prepare when the files are missing, play when they are there.
    fn game_action(&mut self, cx: &mut Context<Self>) {
        if self.playing_now() {
            self.stop_game(cx);
            return;
        }
        if self.preparing {
            scoria_core::api::cancel_prepare();
            self.show_toast(Tone::Neutral, self.t("Cancelling…"), cx);
            return;
        }
        let prepared = self
            .snapshot
            .as_ref()
            .ok()
            .and_then(|snapshot| snapshot["prepared"].as_bool())
            .unwrap_or(false);
        if prepared {
            self.play(cx);
        } else {
            self.prepare(cx);
        }
    }

    fn stop_game(&mut self, cx: &mut Context<Self>) {
        self.play_message = Some(match scoria_core::api::stop_game() {
            Ok(_) => (true, self.t("Stopping…")),
            Err(error) => (false, self.td(&error)),
        });
        self.snapshot = scoria_core::api::snapshot();
        if self.playing_now() {
            self.poll_playing(cx);
        }
        cx.notify();
    }

    fn close_overlay(&mut self, cx: &mut Context<Self>) {
        self.appearance_open = false;
        self.language_open = false;
        cx.notify();
    }

    fn show_toast(&mut self, tone: Tone, message: impl Into<String>, cx: &mut Context<Self>) {
        self.toast_id += 1;
        let id = self.toast_id;
        self.toasts.push(Toast {
            id,
            tone,
            message: message.into(),
        });
        // Never cover the window with a toast pile; the oldest leaves first.
        if self.toasts.len() > 3 {
            self.toasts.remove(0);
        }
        cx.notify();
        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_secs(4))
                .await;
            this.update(cx, |this, cx| {
                this.toasts.retain(|toast| toast.id != id);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn refresh_servers(&mut self) {
        self.servers = scoria_core::api::saved_servers();
    }

    fn add_server(&mut self, cx: &mut Context<Self>) {
        let name = self.server_name.read(cx).as_str().trim().to_string();
        let ip = self.server_ip.read(cx).as_str().trim().to_string();
        match scoria_core::api::add_saved_server(&name, &ip) {
            Ok(list) => {
                self.servers = Ok(list);
                self.server_name
                    .update(cx, |state, cx| state.emplace("", cx));
                self.server_ip.update(cx, |state, cx| state.emplace("", cx));
                self.show_toast(
                    Tone::Success,
                    self.tf("Added {name}.", &[("name", &name)]),
                    cx,
                );
            }
            Err(error) => self.show_toast(Tone::Danger, self.td(&error), cx),
        }
    }

    fn remove_server(&mut self, index: usize, cx: &mut Context<Self>) {
        match scoria_core::api::remove_saved_server(index) {
            Ok(list) => {
                self.servers = Ok(list);
                self.show_toast(Tone::Success, self.t("Removed."), cx);
            }
            Err(error) => self.show_toast(Tone::Danger, self.td(&error), cx),
        }
    }

    fn install(&mut self, project_id: String, cx: &mut Context<Self>) {
        if self.installing.is_some() {
            return;
        }
        self.installing = Some(project_id.clone());
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_spawn(async move { scoria_core::api::install_project(&project_id) })
                .await;
            this.update(cx, |this, cx| {
                this.installing = None;
                match result {
                    Ok(_) => this.show_toast(
                        Tone::Success,
                        this.t("Installed into the selected instance."),
                        cx,
                    ),
                    Err(error) => this.show_toast(Tone::Danger, this.td(&error), cx),
                }
            })
            .ok();
        })
        .detach();
    }

    fn search_mods(&mut self, cx: &mut Context<Self>) {
        let query = self.mods_query.read(cx).as_str().trim().to_string();
        self.mods_loading = true;
        self.mods_error = None;
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_spawn(async move { scoria_core::api::search_catalog(&query) })
                .await;
            this.update(cx, |this, cx| {
                this.mods_loading = false;
                match result {
                    Ok(found) => {
                        this.mods_results =
                            found["results"].as_array().cloned().unwrap_or_default();
                    }
                    Err(error) => {
                        this.mods_results.clear();
                        this.mods_error = Some(error);
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn log_line(&self, line: &str) {
        let dir = scoria_core::backend::data_dir();
        if std::fs::create_dir_all(&dir).is_ok() {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(dir.join("ui.log"))
            {
                let _ = writeln!(file, "{line}");
            }
        }
    }

    fn prepare(&mut self, cx: &mut Context<Self>) {
        if self.preparing {
            return;
        }
        self.preparing = true;
        self.prepare_status = Some("Starting…".to_string());
        self.prepare_percent = None;
        self.log_line("prepare: start");
        cx.notify();

        let needs_bundle = scoria_core::api::installed_version().is_none();
        let (sender, receiver) = std::sync::mpsc::channel::<serde_json::Value>();
        std::thread::spawn(move || {
            let result = (|| -> Result<(), String> {
                if needs_bundle {
                    scoria_core::api::install_latest_game()?;
                }
                let events = sender.clone();
                scoria_core::api::prepare_game_with_progress(move |event| {
                    let _ = events.send(event);
                })?;
                Ok(())
            })();
            let _ = sender.send(serde_json::json!({
                "ui_done": true,
                "error": result.err(),
            }));
        });

        cx.spawn(async move |this, cx| loop {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(150))
                .await;
            let (mut finished, mut failure) = (false, None);
            let (mut percent, mut line) = (None, None);
            while let Ok(event) = receiver.try_recv() {
                if event.get("ui_done").is_some() {
                    finished = true;
                    failure = event["error"].as_str().map(str::to_string);
                } else if event["event"] == "progress" {
                    if let (Some(done), Some(total)) =
                        (event["done"].as_f64(), event["total"].as_f64())
                    {
                        if total > 0.0 {
                            percent = Some((done / total) as f32);
                        }
                    }
                } else if event["event"] == "log" {
                    line = event["line"].as_str().map(str::to_string);
                }
            }
            this.update(cx, |this, cx| {
                if let Some(percent) = percent {
                    this.prepare_percent = Some(percent);
                }
                if let Some(line) = line {
                    this.prepare_status = Some(line);
                }
                if finished {
                    this.preparing = false;
                    this.prepare_percent = None;
                    this.snapshot = scoria_core::api::snapshot();
                    match failure {
                        Some(error) => {
                            this.log_line(&format!("prepare: failed: {error}"));
                            this.show_toast(Tone::Danger, this.td(&error), cx);
                        }
                        None => {
                            this.log_line("prepare: done");
                            this.show_toast(Tone::Success, this.t("26.3 is prepared."), cx);
                            this.run_scripts("on_download_done", cx);
                        }
                    }
                }
                cx.notify();
            })
            .ok();
            if finished {
                break;
            }
        })
        .detach();
    }

    fn playing_now(&self) -> bool {
        self.snapshot
            .as_ref()
            .ok()
            .and_then(|snapshot| snapshot["playing"].as_bool())
            .unwrap_or(false)
    }

    /// The game is a separate process; refresh Home while it runs so its exit clears "Playing".
    fn poll_playing(&mut self, cx: &mut Context<Self>) {
        if self.polling {
            return;
        }
        self.polling = true;
        cx.spawn(async move |this, cx| loop {
            cx.background_executor()
                .timer(std::time::Duration::from_secs(1))
                .await;
            let still_playing = this
                .update(cx, |this, cx| {
                    this.snapshot = scoria_core::api::snapshot();
                    let playing = this.playing_now();
                    if !playing {
                        this.polling = false;
                    }
                    cx.notify();
                    playing
                })
                .unwrap_or(false);
            if !still_playing {
                break;
            }
        })
        .detach();
    }

    fn play(&mut self, cx: &mut Context<Self>) {
        self.play_world(None, cx);
    }

    fn play_world(&mut self, world: Option<String>, cx: &mut Context<Self>) {
        self.play_message = Some(match scoria_core::api::start_game(world.as_deref()) {
            Ok(started) => (
                true,
                self.tf(
                    "Started {version} (pid {pid}).",
                    &[
                        ("version", started["version"].as_str().unwrap_or("the game")),
                        ("pid", &started["pid"].as_u64().unwrap_or(0).to_string()),
                    ],
                ),
            ),
            Err(error) => (false, self.td(&error)),
        });
        self.snapshot = scoria_core::api::snapshot();
        if self.playing_now() {
            self.poll_playing(cx);
            self.run_scripts("on_game_start", cx);
        }
        cx.notify();
    }

    fn join_server(&mut self, server: String, cx: &mut Context<Self>) {
        match scoria_core::api::start_server(&server) {
            Ok(started) => {
                self.snapshot = scoria_core::api::snapshot();
                self.screen = Screen::Home;
                self.show_toast(
                    Tone::Success,
                    self.tf(
                        "Joining {server} on {version}.",
                        &[
                            ("server", &server),
                            ("version", started["version"].as_str().unwrap_or("the game")),
                        ],
                    ),
                    cx,
                );
                if self.playing_now() {
                    self.poll_playing(cx);
                }
            }
            Err(error) => self.show_toast(Tone::Danger, self.td(&error), cx),
        }
        cx.notify();
    }

    fn use_account(&mut self, name: String, cx: &mut Context<Self>) {
        let mut accounts = self.accounts.clone();
        let result = accounts
            .play_as(Player::Offline(name.clone()))
            .and_then(|()| accounts.save());
        match result {
            Ok(()) => {
                self.accounts = accounts;
                self.name_message =
                    Some((true, self.tf("Now playing as {name}.", &[("name", &name)])));
            }
            Err(error) => self.name_message = Some((false, self.td(&error))),
        }
        cx.notify();
    }

    fn remove_account(&mut self, name: String, cx: &mut Context<Self>) {
        let mut accounts = self.accounts.clone();
        let result = accounts
            .remove_offline(&name)
            .and_then(|()| accounts.save());
        match result {
            Ok(()) => {
                self.accounts = accounts;
                self.name_message = Some((true, self.tf("Removed {name}.", &[("name", &name)])));
            }
            Err(error) => self.name_message = Some((false, self.td(&error))),
        }
        cx.notify();
    }

    fn save_name(&mut self, cx: &mut Context<Self>) {
        let name = self.name.read(cx).as_str().trim().to_string();
        if !scoria_core::account::valid_offline_name(&name) {
            self.name_message = Some((false, self.t("3 to 16 letters, digits or _ (any script).")));
            cx.notify();
            return;
        }
        let mut accounts = self.accounts.clone();
        let result = accounts
            .play_as(Player::Offline(name.clone()))
            .and_then(|()| accounts.save());
        self.name_message = Some(match result {
            Ok(()) => {
                self.accounts = accounts;
                self.name.update(cx, |state, cx| state.emplace("", cx));
                (true, self.tf("Now playing as {name}.", &[("name", &name)]))
            }
            Err(error) => (false, self.td(&error)),
        });
        cx.notify();
    }

    fn set_appearance(&mut self, cx: &mut Context<Self>, appearance: Appearance) {
        self.settings.appearance = appearance;
        self.theme = match appearance {
            Appearance::Light => Theme::light(),
            _ => Theme::dark(),
        };
        self.settings.save();
        self.appearance_open = false;
        self.fade = 0.45;
        cx.notify();
        cx.spawn(async move |this, cx| {
            for _ in 0..9 {
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(20))
                    .await;
                this.update(cx, |this, cx| {
                    this.fade = (this.fade - 0.05).max(0.0);
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    fn set_language(&mut self, cx: &mut Context<Self>, language: Option<Language>) {
        self.settings.language = language;
        self.settings.save();
        scoria_core::i18n::set(self.language());
        self.language_open = false;
        cx.notify();
    }
}

pub(crate) fn install_crash_log() {
    let dir = scoria_core::backend::data_dir();
    std::panic::set_hook(Box::new(move |info| {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or_default();
        let report = format!(
            "Scoria {} on {} {}\nat {}s: {}\n{}\n",
            env!("CARGO_PKG_VERSION"),
            std::env::consts::OS,
            std::env::consts::ARCH,
            stamp,
            info,
            std::backtrace::Backtrace::force_capture(),
        );
        eprintln!("{report}");
        if std::fs::create_dir_all(&dir).is_ok() {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(dir.join("crash.log"))
            {
                let _ = file.write_all(report.as_bytes());
            }
        }
    }));
}

/// A dev flag for reproducible screenshots: `--screen mods --query "compass"`.
pub(crate) fn start_screen() -> (Screen, Option<String>, bool, Option<(f32, f32)>) {
    let mut args = std::env::args().skip(1);
    let mut screen = Screen::Home;
    let mut query = None;
    let mut fullscreen = false;
    let mut window_size: Option<(f32, f32)> = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--screen" => {
                screen = match args.next().as_deref() {
                    Some("mods") => Screen::Mods,
                    Some("servers") => Screen::Servers,
                    Some("settings") => Screen::Settings,
                    _ => Screen::Home,
                };
            }
            "--query" => query = args.next(),
            "--fullscreen" => fullscreen = true,
            "--size" => {
                if let Some(value) = args.next() {
                    if let Some((w, h)) = value.split_once('x') {
                        if let (Ok(w), Ok(h)) = (w.parse(), h.parse()) {
                            window_size = Some((w, h));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    (screen, query, fullscreen, window_size)
}

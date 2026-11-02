//! Scoria's launcher window: one process, gpui-ce, no Bun and no native addon.
// The release build is a GUI app: no console window behind the launcher on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod i18n;
use std::{
    borrow::Cow,
    fs::OpenOptions,
    io::Write,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use gpui::{
    div, img, linear_color_stop, linear_gradient, point, prelude::*, px, rgb, rgba, size, Bounds,
    Context, Image, ImageFormat, ImageSource, IntoElement, MouseButton, Render, ResizeEdge, Window,
    WindowBounds, WindowControlArea, WindowDecorations, WindowOptions,
};
use gpui_ce_elements::editable_text::{
    actions::{default_bindings, DEFAULT_INPUT_CONTEXT},
    text_input, EditableTextState, StringStorage,
};
use gpui_ce_platform::application;
use scoria_core::account::{Accounts, Player};
use scoria_core::backend::{Appearance, Settings};
use scoria_core::i18n::Language;
use scoria_ui_kit::{
    badge, card, soft_card, toast, tone_for_support, Button, ButtonSize, ButtonVariant, Theme, Tone,
};

/// The bundled OFL fonts: Medeo Sans for text, Medeo Mono for versions and numbers.
const FONTS: [&[u8]; 6] = [
    include_bytes!("../assets/fonts/MedeoSans-Regular.ttf"),
    include_bytes!("../assets/fonts/MedeoSans-Medium.ttf"),
    include_bytes!("../assets/fonts/MedeoSans-SemiBold.ttf"),
    include_bytes!("../assets/fonts/MedeoSans-Bold.ttf"),
    include_bytes!("../assets/fonts/MedeoMono-Regular.ttf"),
    include_bytes!("../assets/fonts/MedeoMono-Medium.ttf"),
];
const MARK: &[u8] = include_bytes!("../assets/scoria-mark.png");
const BACKDROP: &[u8] = include_bytes!("../assets/backdrop.jpg");
const BACKDROP_LIGHT: &[u8] = include_bytes!("../assets/backdrop-light.jpg");
const ICON_PLAY: &[u8] = include_bytes!("../assets/icons/play.svg");
const ICON_MODS: &[u8] = include_bytes!("../assets/icons/mods.svg");
const ICON_SERVERS: &[u8] = include_bytes!("../assets/icons/servers.svg");
const ICON_SETTINGS: &[u8] = include_bytes!("../assets/icons/settings.svg");
const ICON_SEARCH: &[u8] = include_bytes!("../assets/icons/search.svg");
const ICON_WIN_MIN: &[u8] = include_bytes!("../assets/icons/win-min.svg");
const ICON_WIN_MAX: &[u8] = include_bytes!("../assets/icons/win-max.svg");
const ICON_WIN_CLOSE: &[u8] = include_bytes!("../assets/icons/win-close.svg");
const ICON_MENU: &[u8] = include_bytes!("../assets/icons/menu.svg");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModsSource {
    Horizon,
    Modrinth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Screen {
    Home,
    Mods,
    Servers,
    Settings,
}

impl Screen {
    const ALL: [Screen; 4] = [
        Screen::Home,
        Screen::Mods,
        Screen::Servers,
        Screen::Settings,
    ];

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
fn segment(
    theme: Theme,
    id: &'static str,
    label: &'static str,
    selected: bool,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .items_center()
        .h(px(28.0))
        .px(px(theme.spacing.md))
        .rounded(px(theme.radii.md))
        .bg(if selected {
            if theme.dark {
                rgba(0xFFFFFF12)
            } else {
                theme.colors.surface
            }
        } else {
            rgba(0x00000000)
        })
        .text_color(if selected {
            theme.colors.foreground
        } else {
            theme.colors.muted
        })
        .text_size(px(theme.type_scale.body))
        .cursor_pointer()
        .on_click(on_click)
        .child(label)
}

fn label(theme: &Theme, text: &str) -> impl IntoElement {
    div()
        .text_size(px(theme.type_scale.subtitle))
        .child(text.to_string())
}

struct Shell {
    theme: Theme,
    screen: Screen,
    settings: Settings,
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
    servers: Result<serde_json::Value, String>,
    server_name: gpui::Entity<EditableTextState>,
    server_ip: gpui::Entity<EditableTextState>,
    toast: Option<(Tone, String)>,
    toast_generation: u64,
}

impl Shell {
    fn new(cx: &mut Context<Self>, screen: Screen, query: Option<String>) -> Self {
        let settings = Settings::load();
        let theme = match settings.appearance {
            Appearance::Light => Theme::light(),
            _ => Theme::dark(),
        };
        let current = match Accounts::load().player() {
            Player::Offline(name) => name,
            Player::GameName => String::new(),
        };
        let name = cx.new(|cx| EditableTextState::new(StringStorage::from(current), cx));
        let mut shell = Self {
            theme,
            screen,
            settings,
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
            servers: scoria_core::api::saved_servers(),
            server_name: cx.new(|cx| EditableTextState::new(StringStorage::default(), cx)),
            server_ip: cx.new(|cx| EditableTextState::new(StringStorage::default(), cx)),
            toast: None,
            toast_generation: 0,
        };
        if screen == Screen::Mods && query.is_none() {
            shell.search_mods(cx);
        }
        if std::env::var("SCORIA_AUTO_PREPARE").is_ok() {
            shell.prepare(cx);
        }
        if let Some(query) = query {
            if !query.is_empty() {
                shell
                    .mods_query
                    .update(cx, |state, cx| state.emplace(&query, cx));
            }
            shell.search_mods(cx);
        }
        shell
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
        i18n::tr(self.language(), english).to_string()
    }

    fn toggle_rail(&mut self, cx: &mut Context<Self>) {
        self.settings.rail_collapsed = !self.settings.rail_collapsed;
        self.settings.save();
        cx.notify();
    }

    fn show_toast(&mut self, tone: Tone, message: impl Into<String>, cx: &mut Context<Self>) {
        self.toast_generation += 1;
        let generation = self.toast_generation;
        self.toast = Some((tone, message.into()));
        cx.notify();
        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_secs(3))
                .await;
            this.update(cx, |this, cx| {
                if this.toast_generation == generation {
                    this.toast = None;
                    cx.notify();
                }
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
                self.show_toast(Tone::Success, format!("Added {name}."), cx);
            }
            Err(error) => self.show_toast(Tone::Danger, error, cx),
        }
    }

    fn remove_server(&mut self, index: usize, cx: &mut Context<Self>) {
        match scoria_core::api::remove_saved_server(index) {
            Ok(list) => {
                self.servers = Ok(list);
                self.show_toast(Tone::Success, "Removed.", cx);
            }
            Err(error) => self.show_toast(Tone::Danger, error, cx),
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
                    Ok(_) => {
                        this.show_toast(Tone::Success, "Installed into the selected instance.", cx)
                    }
                    Err(error) => this.show_toast(Tone::Danger, error, cx),
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

        let needs_bundle = self.snapshot.is_err();
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
                            this.show_toast(Tone::Danger, error, cx);
                        }
                        None => {
                            this.log_line("prepare: done");
                            this.show_toast(Tone::Success, "26.3 is prepared.", cx);
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

    fn play(&mut self, cx: &mut Context<Self>) {
        self.play_message = Some(match scoria_core::api::start_game(None) {
            Ok(started) => (
                true,
                format!(
                    "Started {} (pid {}).",
                    started["version"].as_str().unwrap_or("the game"),
                    started["pid"].as_u64().unwrap_or(0)
                ),
            ),
            Err(error) => (false, error),
        });
        self.snapshot = scoria_core::api::snapshot();
        cx.notify();
    }

    fn save_name(&mut self, cx: &mut Context<Self>) {
        let name = self.name.read(cx).as_str().trim().to_string();
        if !scoria_core::account::valid_offline_name(&name) {
            self.name_message = Some((
                false,
                "3 to 16 letters, digits or _ (any script)".to_string(),
            ));
            cx.notify();
            return;
        }
        let result = (|| {
            let mut accounts = Accounts::load();
            accounts.play_as(Player::Offline(name.clone()))?;
            accounts.save()
        })();
        self.name_message = Some(match result {
            Ok(()) => (true, format!("Now playing as {name}.")),
            Err(error) => (false, error),
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
        self.language_open = false;
        cx.notify();
    }

    fn mark() -> impl IntoElement {
        img(ImageSource::Image(Arc::new(Image::from_bytes(
            ImageFormat::Png,
            MARK.to_vec(),
        ))))
        .w(px(26.0))
        .h(px(26.0))
    }

    fn titlebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let _ = cx;
        let control =
            |id: &'static str, icon: &'static [u8], hover: gpui::Rgba, action: fn(&mut Window)| {
                div()
                    .id(id)
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(34.0))
                    .h(px(28.0))
                    .rounded(px(theme.radii.md))
                    .cursor_pointer()
                    .hover(move |style| style.bg(hover))
                    .on_click(move |_, window, _| action(window))
                    .child(
                        gpui::svg()
                            .data(icon)
                            .size(px(12.0))
                            .text_color(theme.colors.muted),
                    )
            };
        div()
            .flex()
            .items_center()
            .justify_between()
            .h(px(40.0))
            .px(px(theme.spacing.md))
            .child(
                div()
                    .id("titlebar-drag")
                    .flex()
                    .items_center()
                    .gap(px(theme.spacing.sm))
                    .flex_1()
                    .h_full()
                    .window_control_area(WindowControlArea::Drag)
                    .on_mouse_down(MouseButton::Left, |_, window, _| window.start_window_move())
                    .child(Self::mark())
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("Scoria"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(2.0))
                    .child(control(
                        "tb-min",
                        ICON_WIN_MIN,
                        theme.colors.surface_tertiary,
                        |window| window.minimize_window(),
                    ))
                    .child(control(
                        "tb-max",
                        ICON_WIN_MAX,
                        theme.colors.surface_tertiary,
                        |window| window.zoom_window(),
                    ))
                    .child(control(
                        "tb-close",
                        ICON_WIN_CLOSE,
                        rgba(0xE5484D44),
                        |window| window.remove_window(),
                    )),
            )
    }

    fn rail(&self, cx: &mut Context<Self>, compact: bool) -> impl IntoElement {
        let theme = self.theme;
        let mut rail = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .w(px(if compact { 64.0 } else { 196.0 }))
            .m(px(theme.spacing.md))
            .p(px(theme.spacing.md))
            .rounded(px(theme.radii.xl))
            .bg(if theme.dark {
                rgba(0x121014A8)
            } else {
                rgba(0xFFFBF6D9)
            })
            .backdrop_blur(px(24.0))
            .border_1()
            .border_color(if theme.dark {
                rgba(0xFFFFFF14)
            } else {
                rgba(0x1D151010)
            })
            .child(div().h(px(theme.spacing.xs)));

        for screen in Screen::ALL {
            let selected = screen == self.screen;
            let on_click = cx.listener(move |this, _: &gpui::ClickEvent, _window, cx| {
                this.screen = screen;
                if screen == Screen::Home {
                    this.snapshot = scoria_core::api::snapshot();
                }
                if screen == Screen::Servers {
                    this.refresh_servers();
                }
                if screen == Screen::Mods && this.mods_results.is_empty() && !this.mods_loading {
                    this.search_mods(cx);
                }
                cx.notify();
            });
            let tint = if selected {
                theme.colors.accent
            } else {
                theme.colors.muted
            };
            let mut item = div()
                .id(("rail", screen as usize))
                .flex()
                .items_center()
                .justify_start()
                .gap(px(theme.spacing.sm))
                .h(px(34.0))
                .px(px(theme.spacing.sm))
                .when(compact, |item| {
                    item.w(px(40.0))
                        .h(px(40.0))
                        .px(px(0.0))
                        .justify_center()
                        .self_center()
                })
                .rounded(px(theme.radii.md))
                .bg(if selected {
                    rgba(0xFF8A4C1F)
                } else {
                    rgba(0x00000000)
                })
                .text_color(if selected {
                    theme.colors.foreground
                } else {
                    theme.colors.muted
                })
                .text_size(px(theme.type_scale.body))
                .cursor_pointer()
                .transitions(|transitions| transitions.bg(std::time::Duration::from_millis(140)))
                .on_click(on_click)
                .child(
                    gpui::svg()
                        .data(screen.icon())
                        .size(px(15.0))
                        .text_color(tint),
                )
                .children((!compact).then(|| self.t(screen.label())));
            if !selected {
                let hover = if theme.dark {
                    rgba(0xFFFFFF0A)
                } else {
                    rgba(0x1D15100A)
                };
                item = item.hover(move |style| style.bg(hover));
            }
            rail = rail.child(item);
        }
        rail = rail.child(div().flex_1()).child(
            div()
                .id("rail-toggle")
                .flex()
                .items_center()
                .justify_center()
                .h(px(30.0))
                .rounded(px(theme.radii.md))
                .text_color(theme.colors.muted)
                .cursor_pointer()
                .hover(move |style| {
                    style.bg(if theme.dark {
                        rgba(0xFFFFFF0A)
                    } else {
                        rgba(0x1D15100A)
                    })
                })
                .on_click(cx.listener(|this, _, _window, cx| this.toggle_rail(cx)))
                .child(
                    gpui::svg()
                        .data(ICON_MENU)
                        .size(px(14.0))
                        .text_color(theme.colors.muted),
                ),
        );
        rail.child(
            div()
                .when(compact, |line| line.hidden())
                .px(px(theme.spacing.sm))
                .pb(px(theme.spacing.sm))
                .text_size(px(theme.type_scale.caption))
                .text_color(theme.colors.muted)
                .child(format!(
                    "v{} · {} {}",
                    env!("SCORIA_VERSION"),
                    env!("SCORIA_TARGET_OS"),
                    env!("SCORIA_TARGET_ARCH")
                )),
        )
    }

    fn home(&self, cx: &mut Context<Self>, compact: bool) -> impl IntoElement {
        let theme = self.theme;
        let (version, prepared, playing) = match &self.snapshot {
            Ok(snapshot) => (
                snapshot["version"].as_str().unwrap_or("—").to_string(),
                snapshot["prepared"].as_bool().unwrap_or(false),
                snapshot["playing"].as_bool().unwrap_or(false),
            ),
            Err(_) => ("No version installed".to_string(), false, false),
        };
        let worlds: Vec<serde_json::Value> = match &self.snapshot {
            Ok(snapshot) => snapshot["worlds"].as_array().cloned().unwrap_or_default(),
            Err(_) => Vec::new(),
        };

        let play = Button::new(
            "play",
            if playing {
                self.t("Playing")
            } else {
                self.t("Play")
            },
        )
        .variant(if prepared {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Secondary
        })
        .size(ButtonSize::Lg)
        .disabled(playing || !prepared)
        .render(
            &theme,
            Some(cx.listener(|this, _: &gpui::ClickEvent, _window, cx| this.play(cx))),
        );
        let prepare = Button::new(
            "prepare",
            if self.preparing {
                self.t("Preparing…")
            } else {
                self.t("Download 26.3")
            },
        )
        .variant(if prepared {
            ButtonVariant::Secondary
        } else {
            ButtonVariant::Primary
        })
        .size(ButtonSize::Lg)
        .disabled(self.preparing || prepared)
        .render(
            &theme,
            Some(cx.listener(|this, _: &gpui::ClickEvent, _window, cx| this.prepare(cx))),
        );

        let mut chips = div().flex().items_center().gap(px(theme.spacing.xs));
        chips = chips
            .child(badge(&theme, version.clone(), Tone::Neutral))
            .child(badge(
                &theme,
                if prepared {
                    self.t("Prepared")
                } else {
                    self.t("Not prepared")
                },
                if prepared {
                    Tone::Success
                } else {
                    Tone::Warning
                },
            ));

        let mut tiles = div().flex().gap(px(theme.spacing.md));
        for world in worlds.iter().take(4) {
            let name = world["name"].as_str().unwrap_or("World");
            let played = world["lastPlayed"].as_str().unwrap_or("");
            tiles = tiles.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(theme.spacing.xs))
                    .w(px(156.0))
                    .child(
                        div()
                            .w_full()
                            .h(px(92.0))
                            .rounded(px(theme.radii.lg))
                            .bg(linear_gradient(
                                160.0,
                                linear_color_stop(rgb(0x3A2116), 0.0),
                                linear_color_stop(rgb(0x141114), 1.0),
                            ))
                            .border_1()
                            .border_color(rgba(0xFFFFFF10)),
                    )
                    .child(
                        div()
                            .text_size(px(theme.type_scale.caption))
                            .child(name.to_string()),
                    )
                    .child(
                        div()
                            .text_size(px(theme.type_scale.caption))
                            .text_color(theme.colors.muted)
                            .child(played.to_string()),
                    ),
            );
        }

        let mut status = div().flex().flex_col().gap(px(theme.spacing.xs));
        if let Some((ok, text)) = &self.play_message {
            status = status.child(
                div()
                    .text_size(px(theme.type_scale.caption))
                    .text_color(if *ok {
                        theme.colors.success
                    } else {
                        theme.colors.danger
                    })
                    .child(text.clone()),
            );
        }

        div()
            .flex()
            .flex_col()
            .gap(px(theme.spacing.xl))
            .max_w(px(920.0))
            .child(
                div().flex().flex_col().gap(px(theme.spacing.sm)).child(
                    div()
                        .text_size(px(if compact { 30.0 } else { 44.0 }))
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child("Minecraft 26.3"),
                ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(theme.spacing.sm))
                    .child(prepare)
                    .child(play),
            )
            .child(chips)
            .children(self.preparing.then(|| {
                div().flex().items_center().gap(px(theme.spacing.sm)).child(
                    div()
                        .text_size(px(theme.type_scale.caption))
                        .text_color(theme.colors.muted)
                        .child(format!(
                            "{} {}",
                            self.prepare_status.clone().unwrap_or_default(),
                            self.prepare_percent
                                .map(|p| format!("{:.0}%", p * 100.0))
                                .unwrap_or_default()
                        )),
                )
            }))
            .child(status)
            .children((!worlds.is_empty()).then(|| {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(theme.spacing.sm))
                    .child(
                        div()
                            .text_size(px(theme.type_scale.subtitle))
                            .child("Your worlds"),
                    )
                    .child(tiles)
            }))
    }

    fn mods(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let search = Button::new("search-mods", self.t("Search"))
            .variant(ButtonVariant::Secondary)
            .size(ButtonSize::Lg)
            .render(
                &theme,
                Some(cx.listener(|this, _: &gpui::ClickEvent, _window, cx| this.search_mods(cx))),
            );

        let mut results = div().flex().flex_col().gap(px(theme.spacing.sm));
        if self.mods_loading {
            results = results.child(
                div()
                    .text_size(px(theme.type_scale.body))
                    .text_color(theme.colors.muted)
                    .child("Searching Modrinth…"),
            );
        }
        if let Some(error) = &self.mods_error {
            results = results.child(
                div()
                    .text_size(px(theme.type_scale.body))
                    .text_color(theme.colors.danger)
                    .child(error.clone()),
            );
        }
        if self.mods_results.is_empty() && !self.mods_loading && self.mods_error.is_none() {
            results = results.child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(theme.spacing.sm))
                    .py(px(theme.spacing.xl))
                    .child(
                        gpui::svg()
                            .data(ICON_SEARCH)
                            .size(px(28.0))
                            .text_color(theme.colors.muted),
                    )
                    .child(
                        div()
                            .text_size(px(theme.type_scale.subtitle))
                            .child(self.t("Nothing yet")),
                    )
                    .child(
                        div()
                            .text_size(px(theme.type_scale.body))
                            .text_color(theme.colors.muted)
                            .child(self.t("Search the registry or type a mod name.")),
                    ),
            );
        }
        let wanted = match self.mods_source {
            ModsSource::Horizon => "Horizon",
            ModsSource::Modrinth => "Modrinth",
        };
        for project in self
            .mods_results
            .iter()
            .filter(|project| project["source"].as_str().unwrap_or("").contains(wanted))
        {
            let title = project["title"].as_str().unwrap_or("Untitled");
            let summary = project["summary"].as_str().unwrap_or("");
            let author = project["author"].as_str().unwrap_or("");
            let support = project["support"].as_str().unwrap_or("");
            let source = project["source"].as_str().unwrap_or("catalogue");
            let id = project["id"].as_str().unwrap_or("").to_string();
            let installable = project["installable"].as_bool().unwrap_or(false);
            let installing = self.installing.as_deref() == Some(id.as_str());
            results = results.child(soft_card(
                &theme,
                div()
                    .id(id.clone())
                    .flex()
                    .flex_col()
                    .gap(px(theme.spacing.xs))
                    .cursor_pointer()
                    .hover(move |style| style.bg(theme.colors.surface_tertiary))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_size(px(theme.type_scale.subtitle))
                                    .child(title.to_string()),
                            )
                            .child(badge(
                                &theme,
                                self.support_label(support),
                                tone_for_support(support),
                            )),
                    )
                    .child(
                        div()
                            .text_size(px(theme.type_scale.body))
                            .text_color(theme.colors.muted)
                            .child(summary.to_string()),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_size(px(theme.type_scale.caption))
                                    .text_color(theme.colors.muted)
                                    .child(format!("by {author} · {source}")),
                            )
                            .child(if installing {
                                div()
                                    .text_size(px(theme.type_scale.caption))
                                    .text_color(theme.colors.muted)
                                    .child(self.t("Installing…"))
                                    .into_any_element()
                            } else if installable {
                                let id = id.clone();
                                Button::new(format!("install-{id}"), self.t("Install"))
                                    .variant(ButtonVariant::Secondary)
                                    .size(ButtonSize::Sm)
                                    .render(
                                        &theme,
                                        Some(cx.listener(move |this, _, _window, cx| {
                                            this.install(id.clone(), cx)
                                        })),
                                    )
                                    .into_any_element()
                            } else {
                                div()
                                    .text_size(px(theme.type_scale.caption))
                                    .text_color(theme.colors.muted)
                                    .child(self.t("Not installable"))
                                    .into_any_element()
                            }),
                    ),
            ));
        }

        card(
            &theme,
            div()
                .flex()
                .flex_col()
                .gap(px(theme.spacing.lg))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(theme.spacing.xs))
                        .child(
                            div()
                                .text_size(px(theme.type_scale.title))
                                .child(self.t("Mods")),
                        )
                        .child(
                            div()
                                .text_size(px(theme.type_scale.body))
                                .text_color(theme.colors.muted)
                                .child(
                                    self.t("Horizon's registry first, then Modrinth. Every result says how it runs on Scoria."),
                                ),
                        ),
                )
                .child(catalogue_tabs(self, cx))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(theme.spacing.sm))
                        .child(
                            text_input("mods-query")
                                .state(self.mods_query.downgrade())
                                .flex_1()
                                .px(px(theme.spacing.md))
                                .py(px(theme.spacing.sm))
                                .placeholder_color(theme.colors.muted)
                                .rounded(px(theme.radii.field))
                                .border_1()
                                .border_color(theme.colors.border)
                                .bg(theme.colors.surface_secondary)
                                .text_color(theme.colors.foreground)
                                .text_size(px(theme.type_scale.body))
                                .whitespace_nowrap()
                                .min_h_auto()
                                .max_h_auto()
                                .placeholder(self.t("Search mods")),
                        )
                        .child(search),
                )
                .child(results),
        )
        .max_w(px(880.0))
    }

    fn servers_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let (list, error) = match &self.servers {
            Ok(value) => (
                value["servers"].as_array().cloned().unwrap_or_default(),
                None,
            ),
            Err(error) => (Vec::new(), Some(error.clone())),
        };

        let add = Button::new("add-server", self.t("Add"))
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Lg)
            .render(
                &theme,
                Some(cx.listener(|this, _: &gpui::ClickEvent, _window, cx| this.add_server(cx))),
            );

        let mut rows = div().flex().flex_col().gap(px(theme.spacing.sm));
        if let Some(error) = &error {
            rows = rows.child(
                div()
                    .text_size(px(theme.type_scale.body))
                    .text_color(theme.colors.danger)
                    .child(error.clone()),
            );
        }
        if list.is_empty() && error.is_none() {
            rows = rows.child(
                div()
                    .text_size(px(theme.type_scale.body))
                    .text_color(theme.colors.muted)
                    .child(
                        self.t("No saved servers yet. Add the address of an offline-mode server."),
                    ),
            );
        }
        for server in &list {
            let index = server["index"].as_u64().unwrap_or(0) as usize;
            let name = server["name"].as_str().unwrap_or("Server");
            let ip = server["ip"].as_str().unwrap_or("");
            rows = rows.child(soft_card(
                &theme,
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_size(px(theme.type_scale.body))
                                    .child(name.to_string()),
                            )
                            .child(
                                div()
                                    .text_size(px(theme.type_scale.caption))
                                    .text_color(theme.colors.muted)
                                    .child(ip.to_string()),
                            ),
                    )
                    .child(
                        Button::new(format!("remove-{index}"), self.t("Remove"))
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Sm)
                            .render(
                                &theme,
                                Some(cx.listener(move |this, _, _window, cx| {
                                    this.remove_server(index, cx)
                                })),
                            ),
                    ),
            ));
        }

        card(
            &theme,
            div()
                .flex()
                .flex_col()
                .gap(px(theme.spacing.lg))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(theme.spacing.xs))
                        .child(
                            div()
                                .text_size(px(theme.type_scale.title))
                                .child(self.t("Servers")),
                        )
                        .child(
                            div()
                                .text_size(px(theme.type_scale.body))
                                .text_color(theme.colors.muted)
                                .child(
                                    self.t("Saved servers for this instance. LAN worlds appear inside the game."),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(theme.spacing.sm))
                        .child(
                            text_input("server-name")
                                .state(self.server_name.downgrade())
                                .flex_1()
                                .px(px(theme.spacing.md))
                                .py(px(theme.spacing.sm))
                                .placeholder(self.t("Name"))
                                .placeholder_color(theme.colors.muted)
                                .rounded(px(theme.radii.field))
                                .border_1()
                                .border_color(theme.colors.border)
                                .bg(theme.colors.surface_secondary)
                                .text_color(theme.colors.foreground)
                                .text_size(px(theme.type_scale.body))
                                .whitespace_nowrap()
                                .min_h_auto()
                                .max_h_auto(),
                        )
                        .child(
                            text_input("server-ip")
                                .state(self.server_ip.downgrade())
                                .flex_1()
                                .px(px(theme.spacing.md))
                                .py(px(theme.spacing.sm))
                                .placeholder(self.t("host:port"))
                                .placeholder_color(theme.colors.muted)
                                .rounded(px(theme.radii.field))
                                .border_1()
                                .border_color(theme.colors.border)
                                .bg(theme.colors.surface_secondary)
                                .text_color(theme.colors.foreground)
                                .text_size(px(theme.type_scale.body))
                                .whitespace_nowrap()
                                .min_h_auto()
                                .max_h_auto(),
                        )
                        .child(add),
                )
                .child(rows),
        )
        .max_w(px(880.0))
    }

    fn settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;

        let appearance_label = match self.settings.appearance {
            Appearance::Dark => self.t("Dark"),
            Appearance::Light => self.t("Light"),
            Appearance::System => self.t("System"),
        };
        let mut appearance_menu = div()
            .absolute()
            .top(px(38.0))
            .left(px(0.0))
            .w(px(220.0))
            .flex()
            .flex_col()
            .gap(px(2.0))
            .p(px(4.0))
            .rounded(px(theme.radii.xl))
            .bg(theme.colors.overlay)
            .border_1()
            .border_color(theme.colors.border);
        for (id, value, label) in [
            ("opt-dark", Appearance::Dark, "Dark"),
            ("opt-light", Appearance::Light, "Light"),
            ("opt-appearance-system", Appearance::System, "System"),
        ] {
            let selected = self.settings.appearance == value;
            let on_click = cx.listener(move |this, _: &gpui::ClickEvent, _window, cx| {
                this.set_appearance(cx, value)
            });
            appearance_menu =
                appearance_menu.child(menu_item(theme, id, self.t(label), selected, on_click));
        }
        let mut appearance = div().relative().w(px(220.0)).child(
            Button::new("appearance-trigger", format!("{appearance_label}   ▾"))
                .variant(ButtonVariant::Secondary)
                .size(ButtonSize::Md)
                .render(
                    &theme,
                    Some(cx.listener(|this, _, _window, cx| {
                        this.appearance_open = !this.appearance_open;
                        this.language_open = false;
                        cx.notify();
                    })),
                ),
        );
        if self.appearance_open {
            appearance = appearance.child(appearance_menu);
        }

        let language_label = match self.settings.language {
            None => self.t("System"),
            Some(Language::English) => "English".to_string(),
            Some(Language::Russian) => "Русский".to_string(),
            Some(Language::French) => "Français".to_string(),
        };
        let mut language_menu = div()
            .absolute()
            .top(px(38.0))
            .left(px(0.0))
            .w(px(220.0))
            .flex()
            .flex_col()
            .gap(px(2.0))
            .p(px(4.0))
            .rounded(px(theme.radii.xl))
            .bg(theme.colors.overlay)
            .border_1()
            .border_color(theme.colors.border);
        for (id, value, label) in [
            ("opt-language-system", None, "System"),
            ("opt-english", Some(Language::English), "English"),
            ("opt-russian", Some(Language::Russian), "Русский"),
        ] {
            let selected = self.settings.language == value;
            let on_click = cx.listener(move |this, _: &gpui::ClickEvent, _window, cx| {
                this.set_language(cx, value)
            });
            let text = if label == "System" {
                self.t("System")
            } else {
                label.to_string()
            };
            language_menu = language_menu.child(menu_item(theme, id, text, selected, on_click));
        }
        let mut language = div().relative().w(px(220.0)).child(
            Button::new("language-trigger", format!("{language_label}   ▾"))
                .variant(ButtonVariant::Secondary)
                .size(ButtonSize::Md)
                .render(
                    &theme,
                    Some(cx.listener(|this, _, _window, cx| {
                        this.language_open = !this.language_open;
                        this.appearance_open = false;
                        cx.notify();
                    })),
                ),
        );
        if self.language_open {
            language = language.child(language_menu);
        }

        card(
            &theme,
            div()
                .flex()
                .flex_col()
                .gap(px(theme.spacing.lg))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(theme.spacing.xs))
                        .child(
                            div()
                                .text_size(px(theme.type_scale.title))
                                .child(self.t("Settings")),
                        )
                        .child(
                            div()
                                .text_size(px(theme.type_scale.body))
                                .text_color(theme.colors.muted)
                                .child(self.t(
                                    "Appearance and language are saved to the launcher's settings.",
                                )),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(theme.spacing.sm))
                        .child(label(&theme, &self.t("Appearance")))
                        .child(appearance),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(theme.spacing.sm))
                        .child(label(&theme, &self.t("Language")))
                        .child(language),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(theme.spacing.sm))
                        .child(label(&theme, &self.t("Nickname")))
                        .child(
                            div()
                                .text_size(px(theme.type_scale.caption))
                                .text_color(theme.colors.muted)
                                .child(
                                    self.t("No Microsoft account: this name is what servers see."),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(theme.spacing.sm))
                                .child(
                                    text_input("offline-name")
                                        .state(self.name.downgrade())
                                        .w(px(280.0))
                                        .px(px(theme.spacing.md))
                                        .py(px(theme.spacing.sm))
                                        .placeholder_color(theme.colors.muted)
                                        .rounded(px(theme.radii.field))
                                        .border_1()
                                        .border_color(theme.colors.border)
                                        .bg(theme.colors.surface_secondary)
                                        .text_color(theme.colors.foreground)
                                        .text_size(px(theme.type_scale.body))
                                        .whitespace_nowrap()
                                        .min_h_auto()
                                        .max_h_auto()
                                        .placeholder(self.t("Your offline name")),
                                )
                                .child(
                                    Button::new("save-name", self.t("Save"))
                                        .variant(ButtonVariant::Primary)
                                        .size(ButtonSize::Lg)
                                        .render(
                                            &theme,
                                            Some(cx.listener(|this, _, _window, cx| {
                                                this.save_name(cx)
                                            })),
                                        ),
                                ),
                        )
                        .child(match &self.name_message {
                            Some((ok, text)) => div()
                                .text_size(px(theme.type_scale.caption))
                                .text_color(if *ok {
                                    theme.colors.success
                                } else {
                                    theme.colors.danger
                                })
                                .child(text.clone()),
                            None => div(),
                        }),
                ),
        )
        .max_w(px(880.0))
    }
}

impl Render for Shell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        // Below ~1080px the rail drops its labels and the hero shrinks.
        let compact = window.viewport_size().width < px(1080.0) || self.settings.rail_collapsed;
        let body = match self.screen {
            Screen::Home => div().child(self.home(cx, compact)),
            Screen::Mods => div().child(self.mods(cx)),
            Screen::Servers => div().child(self.servers_screen(cx)),
            Screen::Settings => div().child(self.settings(cx)),
        };

        let mut root = div()
            .relative()
            .size_full()
            .font_family(theme.type_scale.sans)
            .bg(rgb(0x09080A))
            .text_color(theme.colors.foreground)
            .child(
                img(ImageSource::Image(Arc::new(Image::from_bytes(
                    ImageFormat::Jpeg,
                    if theme.dark {
                        BACKDROP.to_vec()
                    } else {
                        BACKDROP_LIGHT.to_vec()
                    },
                ))))
                .absolute()
                .top(px(0.0))
                .left(px(0.0))
                .w_full()
                .h_full(),
            )
            .child(
                div()
                    .absolute()
                    .top(px(0.0))
                    .left(px(0.0))
                    .w_full()
                    .h_full()
                    .bg(if theme.dark {
                        rgba(0x09080A4D)
                    } else {
                        rgba(0xFFFFFF2E)
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .overflow_hidden()
                    .child(self.titlebar(cx))
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .min_h(px(0.0))
                            .overflow_hidden()
                            .child(self.rail(cx, compact))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .min_h(px(0.0))
                                    .id("screen")
                                    .p(px(if compact { 20.0 } else { 32.0 }))
                                    .gap(px(theme.spacing.lg))
                                    .overflow_y_scroll()
                                    .child(body),
                            ),
                    ),
            );
        let edge = |id: &'static str, edge: ResizeEdge| {
            div()
                .id(id)
                .absolute()
                .on_mouse_down(MouseButton::Left, move |_, window, _| {
                    window.start_window_resize(edge)
                })
        };
        root = root
            .child(
                edge("rz-n", ResizeEdge::Top)
                    .top(px(0.))
                    .left(px(8.))
                    .right(px(8.))
                    .h(px(6.)),
            )
            .child(
                edge("rz-s", ResizeEdge::Bottom)
                    .bottom(px(0.))
                    .left(px(8.))
                    .right(px(8.))
                    .h(px(6.)),
            )
            .child(
                edge("rz-w", ResizeEdge::Left)
                    .left(px(0.))
                    .top(px(8.))
                    .bottom(px(8.))
                    .w(px(6.)),
            )
            .child(
                edge("rz-e", ResizeEdge::Right)
                    .right(px(0.))
                    .top(px(8.))
                    .bottom(px(8.))
                    .w(px(6.)),
            )
            .child(
                edge("rz-nw", ResizeEdge::TopLeft)
                    .top(px(0.))
                    .left(px(0.))
                    .w(px(10.))
                    .h(px(10.)),
            )
            .child(
                edge("rz-ne", ResizeEdge::TopRight)
                    .top(px(0.))
                    .right(px(0.))
                    .w(px(10.))
                    .h(px(10.)),
            )
            .child(
                edge("rz-sw", ResizeEdge::BottomLeft)
                    .bottom(px(0.))
                    .left(px(0.))
                    .w(px(10.))
                    .h(px(10.)),
            )
            .child(
                edge("rz-se", ResizeEdge::BottomRight)
                    .bottom(px(0.))
                    .right(px(0.))
                    .w(px(10.))
                    .h(px(10.)),
            );

        if self.fade > 0.0 {
            root = root.child(
                div()
                    .absolute()
                    .top(px(0.0))
                    .left(px(0.0))
                    .w_full()
                    .h_full()
                    .bg(theme.colors.background)
                    .opacity(self.fade),
            );
        }

        if let Some((tone, message)) = &self.toast {
            root = root.child(
                div()
                    .absolute()
                    .bottom(px(theme.spacing.xl))
                    .right(px(theme.spacing.xl))
                    .child(toast(&theme, message.clone(), *tone)),
            );
        }
        root
    }
}

/// A crash leaves a file next to the settings, because a window that disappears is not a bug
/// report. `--crash-test` panics on purpose so the log can be checked without breaking anything.
fn install_crash_log() {
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
fn start_screen() -> (Screen, Option<String>, bool, Option<(f32, f32)>) {
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

fn main() {
    install_crash_log();
    if std::env::args().any(|arg| arg == "--crash-test") {
        panic!("crash-test: the crash log must contain this line");
    }

    let (screen, query, fullscreen, window_size) = start_screen();
    application().run(move |cx| {
        cx.text_system()
            .add_fonts(FONTS.iter().map(|font| Cow::Borrowed(*font)).collect())
            .expect("the bundled Medeo fonts");

        cx.bind_keys(default_bindings().as_keybindings(Some(DEFAULT_INPUT_CONTEXT)));

        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        // A fixed origin keeps headless screenshots reproducible.
        let window_bounds = if fullscreen {
            let display = cx
                .primary_display()
                .map(|display| display.bounds())
                .unwrap_or_else(|| {
                    Bounds::new(point(px(0.0), px(0.0)), size(px(1280.0), px(720.0)))
                });
            WindowBounds::Fullscreen(display)
        } else {
            WindowBounds::Windowed(Bounds::new(
                point(px(0.0), px(0.0)),
                match window_size {
                    Some((w, h)) => size(px(w), px(h)),
                    None => size(px(1280.0), px(720.0)),
                },
            ))
        };
        cx.open_window(
            WindowOptions::new()
                .window_bounds(Some(window_bounds))
                .window_decorations(WindowDecorations::Client)
                .focus(true)
                .titlebar(None),
            move |_, cx| cx.new(|cx| Shell::new(cx, screen, query.clone())),
        )
        .expect("the launcher window");
        cx.activate(true);
    });
}

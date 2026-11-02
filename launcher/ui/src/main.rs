//! Scoria's launcher window: one process, gpui-ce, no Bun and no native addon.
// The release build is a GUI app: no console window behind the launcher on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{
    borrow::Cow,
    fs::OpenOptions,
    io::Write,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use gpui::{
    div, img, linear_color_stop, linear_gradient, point, prelude::*, px, rgb, rgba, size, Bounds,
    Context, Image, ImageFormat, ImageSource, IntoElement, MouseButton, Render, Window,
    WindowBounds, WindowDecorations, WindowOptions,
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
const ICON_PLAY: &[u8] = include_bytes!("../assets/icons/play.svg");
const ICON_MODS: &[u8] = include_bytes!("../assets/icons/mods.svg");
const ICON_SERVERS: &[u8] = include_bytes!("../assets/icons/servers.svg");
const ICON_SETTINGS: &[u8] = include_bytes!("../assets/icons/settings.svg");
const ICON_SEARCH: &[u8] = include_bytes!("../assets/icons/search.svg");

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

/// A segmented-control option: the selected one carries the accent.
fn segment(
    theme: Theme,
    label: &'static str,
    selected: bool,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    div()
        .id(label)
        .flex()
        .items_center()
        .h(px(32.0))
        .px(px(theme.spacing.md))
        .rounded(px(theme.radii.lg))
        .border_1()
        .border_color(if selected {
            theme.colors.accent
        } else {
            theme.colors.border
        })
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
        .text_size(px(theme.type_scale.body))
        .cursor_pointer()
        .on_click(on_click)
        .child(label)
}

fn label(theme: &Theme, text: &'static str) -> impl IntoElement {
    div().text_size(px(theme.type_scale.subtitle)).child(text)
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
            servers: scoria_core::api::saved_servers(),
            server_name: cx.new(|cx| EditableTextState::new(StringStorage::default(), cx)),
            server_ip: cx.new(|cx| EditableTextState::new(StringStorage::default(), cx)),
            toast: None,
            toast_generation: 0,
        };
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

    fn prepare(&mut self, cx: &mut Context<Self>) {
        if self.preparing {
            return;
        }
        self.preparing = true;
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_spawn(async { scoria_core::api::prepare_game() })
                .await;
            this.update(cx, |this, cx| {
                this.preparing = false;
                this.snapshot = scoria_core::api::snapshot();
                match result {
                    Ok(_) => this.show_toast(Tone::Success, "26.3 is prepared.", cx),
                    Err(error) => this.show_toast(Tone::Danger, error, cx),
                }
            })
            .ok();
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
        cx.notify();
    }

    fn set_language(&mut self, cx: &mut Context<Self>, language: Option<Language>) {
        self.settings.language = language;
        self.settings.save();
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
        let control = |id: &'static str, label: &'static str, control: fn(&mut Window)| {
            div()
                .id(id)
                .flex()
                .items_center()
                .justify_center()
                .w(px(34.0))
                .h(px(28.0))
                .rounded(px(theme.radii.md))
                .text_color(theme.colors.muted)
                .text_size(px(theme.type_scale.caption))
                .cursor_pointer()
                .hover(move |style| style.bg(theme.colors.surface_tertiary))
                .on_click(move |_, window, _| control(window))
                .child(label)
        };
        let _ = cx;
        div()
            .flex()
            .items_center()
            .justify_between()
            .h(px(40.0))
            .px(px(theme.spacing.md))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(theme.spacing.sm))
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
                    .gap(px(4.0))
                    .child(control("tb-min", "—", |window| window.minimize_window()))
                    .child(control("tb-max", "□", |window| window.zoom_window()))
                    .child(control("tb-close", "×", |window| window.remove_window())),
            )
    }

    fn rail(&self, cx: &mut Context<Self>, compact: bool) -> impl IntoElement {
        let theme = self.theme;
        let mut rail = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .w(px(if compact { 72.0 } else { 248.0 }))
            .h_full()
            .m(px(theme.spacing.md))
            .p(px(theme.spacing.md))
            .rounded(px(theme.radii.xl))
            .bg(rgba(0x121014A8))
            .backdrop_blur(px(24.0))
            .border_1()
            .border_color(rgba(0xFFFFFF14))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(theme.spacing.sm))
                    .px(px(theme.spacing.sm))
                    .py(px(theme.spacing.md))
                    .child(Self::mark())
                    .children((!compact).then(|| {
                        div()
                            .text_size(px(18.0))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("Scoria")
                    })),
            )
            .child(div().h(px(theme.spacing.sm)));

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
                .gap(px(theme.spacing.sm))
                .h(px(40.0))
                .px(px(theme.spacing.sm))
                .rounded(px(theme.radii.lg))
                .bg(if selected {
                    theme.colors.accent_soft
                } else {
                    theme.colors.surface_secondary
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
                        .size(px(16.0))
                        .text_color(tint),
                )
                .children((!compact).then(|| screen.label()));
            if !selected {
                item = item.hover(move |style| style.bg(theme.colors.surface_tertiary));
            }
            rail = rail.child(item);
        }
        rail.child(div().flex_1()).child(
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

        let play = Button::new("play", if playing { "Playing" } else { "Play" })
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
                "Preparing…"
            } else {
                "Download 26.3"
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
                if prepared { "Prepared" } else { "Not prepared" },
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
        let search = Button::new("search-mods", "Search")
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
                            .child("Nothing yet"),
                    )
                    .child(
                        div()
                            .text_size(px(theme.type_scale.body))
                            .text_color(theme.colors.muted)
                            .child("Search the registry or type a mod name."),
                    ),
            );
        }
        for project in self.mods_results.iter() {
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
                            .child(badge(&theme, support, tone_for_support(support))),
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
                                    .child("Installing…")
                                    .into_any_element()
                            } else if installable {
                                let id = id.clone();
                                Button::new(format!("install-{id}"), "Install")
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
                                    .child("Not installable")
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
                        .child(div().text_size(px(theme.type_scale.title)).child("Mods"))
                        .child(
                            div()
                                .text_size(px(theme.type_scale.body))
                                .text_color(theme.colors.muted)
                                .child(
                                    "Horizon's registry first, then Modrinth. Every result says how it runs on Scoria.",
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(theme.spacing.sm))
                        .child(
                            text_input("mods-query")
                                .state(self.mods_query.downgrade())
                                .w(px(360.0))
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
                                .placeholder("Search mods"),
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

        let add = Button::new("add-server", "Add")
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
                    .child("No saved servers yet. Add the address of an offline-mode server."),
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
                        Button::new(format!("remove-{index}"), "Remove")
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
                        .child(div().text_size(px(theme.type_scale.title)).child("Servers"))
                        .child(
                            div()
                                .text_size(px(theme.type_scale.body))
                                .text_color(theme.colors.muted)
                                .child(
                                    "Saved servers for this instance. LAN worlds appear inside the game.",
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
                                .w(px(200.0))
                                .px(px(theme.spacing.md))
                                .py(px(theme.spacing.sm))
                                .placeholder("Name")
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
                                .w(px(240.0))
                                .px(px(theme.spacing.md))
                                .py(px(theme.spacing.sm))
                                .placeholder("host:port")
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

        let mut appearance = div().flex().gap(px(theme.spacing.xs));
        for (value, label) in [
            (Appearance::Dark, "Dark"),
            (Appearance::Light, "Light"),
            (Appearance::System, "System"),
        ] {
            let selected = self.settings.appearance == value;
            let on_click = cx.listener(move |this, _: &gpui::ClickEvent, _window, cx| {
                this.set_appearance(cx, value)
            });
            appearance = appearance.child(segment(theme, label, selected, on_click));
        }

        let mut language = div().flex().gap(px(theme.spacing.xs));
        for (value, label) in [
            (None, "System"),
            (Some(Language::English), "English"),
            (Some(Language::Russian), "Русский"),
        ] {
            let selected = self.settings.language == value;
            let on_click = cx.listener(move |this, _: &gpui::ClickEvent, _window, cx| {
                this.set_language(cx, value)
            });
            language = language.child(segment(theme, label, selected, on_click));
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
                                .child("Settings"),
                        )
                        .child(
                            div()
                                .text_size(px(theme.type_scale.body))
                                .text_color(theme.colors.muted)
                                .child(
                                    "Appearance and language are saved to the launcher's settings.",
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(theme.spacing.sm))
                        .child(label(&theme, "Appearance"))
                        .child(appearance),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(theme.spacing.sm))
                        .child(label(&theme, "Language"))
                        .child(language),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(theme.spacing.sm))
                        .child(label(&theme, "Offline name"))
                        .child(
                            div()
                                .text_size(px(theme.type_scale.caption))
                                .text_color(theme.colors.muted)
                                .child("No Microsoft account: this name is what servers see."),
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
                                        .placeholder("Your offline name"),
                                )
                                .child(
                                    Button::new("save-name", "Save")
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
                            None => div()
                                .text_size(px(theme.type_scale.caption))
                                .text_color(theme.colors.muted)
                                .child("3 to 16 letters, digits or _ (any script)."),
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
        let compact = window.viewport_size().width < px(1080.0);
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
                    BACKDROP.to_vec(),
                ))))
                .absolute()
                .top(px(0.0))
                .left(px(0.0))
                .w_full()
                .h_full(),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .child(self.titlebar(cx))
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .min_h(px(0.0))
                            .child(self.rail(cx, compact))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .h_full()
                                    .m(px(theme.spacing.md))
                                    .ml(px(0.0))
                                    .p(px(if compact { 20.0 } else { 32.0 }))
                                    .gap(px(theme.spacing.lg))
                                    .rounded(px(theme.radii.xl))
                                    .bg(rgba(0x1210149E))
                                    .backdrop_blur(px(28.0))
                                    .border_1()
                                    .border_color(rgba(0xFFFFFF10))
                                    .child(body),
                            ),
                    ),
            );
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

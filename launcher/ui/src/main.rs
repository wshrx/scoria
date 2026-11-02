//! Scoria's launcher window: one process, gpui-ce, no Bun and no native addon.
use std::{
    borrow::Cow,
    fs::OpenOptions,
    io::Write,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use gpui::{
    div, img, point, prelude::*, px, size, Bounds, Context, Image, ImageFormat, ImageSource,
    IntoElement, Render, Window, WindowBounds, WindowOptions,
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
    badge, card, soft_card, tone_for_support, Button, ButtonSize, ButtonVariant, Theme,
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

    fn rail(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let mut rail = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .w(px(248.0))
            .h_full()
            .p(px(theme.spacing.md))
            .bg(theme.colors.surface_secondary)
            .border_r_1()
            .border_color(theme.colors.separator)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(theme.spacing.sm))
                    .px(px(theme.spacing.sm))
                    .py(px(theme.spacing.md))
                    .child(Self::mark())
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("Scoria"),
                    ),
            )
            .child(div().h(px(theme.spacing.sm)));

        for screen in Screen::ALL {
            let selected = screen == self.screen;
            let on_click = cx.listener(move |this, _: &gpui::ClickEvent, _window, cx| {
                this.screen = screen;
                if screen == Screen::Home {
                    this.snapshot = scoria_core::api::snapshot();
                }
                cx.notify();
            });
            let indicator = div()
                .w(px(3.0))
                .h(px(16.0))
                .rounded(px(2.0))
                .bg(if selected {
                    theme.colors.accent
                } else {
                    theme.colors.surface_secondary
                });
            rail = rail.child(
                div()
                    .id(("rail", screen as usize))
                    .flex()
                    .items_center()
                    .gap(px(theme.spacing.sm))
                    .h(px(40.0))
                    .px(px(theme.spacing.sm))
                    .rounded(px(theme.radii.lg))
                    .bg(if selected {
                        theme.colors.surface_tertiary
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
                    .hover(move |style| style.bg(theme.colors.surface_tertiary))
                    .on_click(on_click)
                    .child(indicator)
                    .child(screen.label()),
            );
        }
        rail.child(div().flex_1())
    }

    fn home(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let (version, prepared, playing) = match &self.snapshot {
            Ok(snapshot) => (
                snapshot["version"].as_str().unwrap_or("—").to_string(),
                snapshot["prepared"].as_bool().unwrap_or(false),
                snapshot["playing"].as_bool().unwrap_or(false),
            ),
            Err(_) => ("No version installed".to_string(), false, false),
        };
        let worlds = match &self.snapshot {
            Ok(snapshot) => snapshot["worlds"].as_array().map(Vec::len).unwrap_or(0),
            Err(_) => 0,
        };
        let problem = match &self.snapshot {
            Ok(_) if prepared => None,
            Ok(_) => Some("Press Download and prepare once to fetch 26.3.".to_string()),
            Err(error) => Some(error.clone()),
        };
        let play = Button::new("play", if playing { "Playing" } else { "Play" })
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Md)
            .disabled(playing || !prepared)
            .render(
                &theme,
                Some(cx.listener(|this, _: &gpui::ClickEvent, _window, cx| this.play(cx))),
            );

        let row = |name: &'static str, value: String, tone: gpui::Rgba| {
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_size(px(theme.type_scale.body))
                        .text_color(theme.colors.muted)
                        .child(name),
                )
                .child(
                    div()
                        .text_size(px(theme.type_scale.body))
                        .text_color(tone)
                        .child(value),
                )
        };
        let mut status = div().flex().flex_col().gap(px(theme.spacing.sm)).child(
            div()
                .text_size(px(theme.type_scale.subtitle))
                .child(if playing { "Running" } else { "Ready" }),
        );
        status = status
            .child(row("Version", version, theme.colors.foreground))
            .child(row(
                "Resources",
                if prepared {
                    "Prepared".to_string()
                } else {
                    "Not prepared".to_string()
                },
                if prepared {
                    theme.colors.success
                } else {
                    theme.colors.warning
                },
            ))
            .child(row("Worlds", worlds.to_string(), theme.colors.foreground));
        if let Some(problem) = problem {
            status = status.child(
                div()
                    .text_size(px(theme.type_scale.body))
                    .text_color(theme.colors.muted)
                    .child(problem),
            );
        }
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
            .gap(px(theme.spacing.lg))
            .max_w(px(880.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(theme.spacing.xs))
                    .child(
                        div()
                            .text_size(px(theme.type_scale.display))
                            .child("Minecraft: Java Edition 26.3"),
                    )
                    .child(
                        div()
                            .text_size(px(theme.type_scale.body))
                            .text_color(theme.colors.muted)
                            .child("Offline by default. No Microsoft account, no telemetry."),
                    ),
            )
            .child(div().w(px(160.0)).child(play))
            .child(card(&theme, status))
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
                    .text_size(px(theme.type_scale.body))
                    .text_color(theme.colors.muted)
                    .child("Nothing yet. Search the registry or type a mod name."),
            );
        }
        for project in self.mods_results.iter().take(8) {
            let title = project["title"].as_str().unwrap_or("Untitled");
            let summary = project["summary"].as_str().unwrap_or("");
            let author = project["author"].as_str().unwrap_or("");
            let support = project["support"].as_str().unwrap_or("");
            let source = project["source"].as_str().unwrap_or("catalogue");
            results = results.child(soft_card(
                &theme,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(theme.spacing.xs))
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
                            .text_size(px(theme.type_scale.caption))
                            .text_color(theme.colors.muted)
                            .child(format!("by {author} · {source}")),
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

    fn placeholder(&self, title: &'static str) -> impl IntoElement {
        let theme = self.theme;
        card(
            &theme,
            div()
                .flex()
                .flex_col()
                .gap(px(theme.spacing.xs))
                .child(div().text_size(px(theme.type_scale.title)).child(title))
                .child(
                    div()
                        .text_color(theme.colors.muted)
                        .child("This screen is ported in the next slice."),
                ),
        )
    }
}

impl Render for Shell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let body = match self.screen {
            Screen::Home => div().child(self.home(cx)),
            Screen::Mods => div().child(self.mods(cx)),
            Screen::Servers => div().child(self.placeholder("Servers")),
            Screen::Settings => div().child(self.settings(cx)),
        };

        div()
            .flex()
            .size_full()
            .font_family(theme.type_scale.sans)
            .bg(theme.colors.background)
            .text_color(theme.colors.foreground)
            .child(self.rail(cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .h_full()
                    .p(px(theme.spacing.xl))
                    .gap(px(theme.spacing.lg))
                    .child(body),
            )
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
fn start_screen() -> (Screen, Option<String>, bool) {
    let mut args = std::env::args().skip(1);
    let mut screen = Screen::Home;
    let mut query = None;
    let mut fullscreen = false;
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
            _ => {}
        }
    }
    (screen, query, fullscreen)
}

fn main() {
    install_crash_log();
    if std::env::args().any(|arg| arg == "--crash-test") {
        panic!("crash-test: the crash log must contain this line");
    }

    let (screen, query, fullscreen) = start_screen();
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
                size(px(1280.0), px(720.0)),
            ))
        };
        cx.open_window(
            WindowOptions::new()
                .window_bounds(Some(window_bounds))
                .focus(true)
                .titlebar(None),
            move |_, cx| cx.new(|cx| Shell::new(cx, screen, query.clone())),
        )
        .expect("the launcher window");
        cx.activate(true);
    });
}

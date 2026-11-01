//! Scoria's launcher window: one process, gpui-ce, no Bun and no native addon.
use std::{borrow::Cow, sync::Arc};

use gpui::{
    div, img, point, prelude::*, px, size, Bounds, Context, Image, ImageFormat, ImageSource,
    IntoElement, Render, Window, WindowBounds, WindowOptions,
};
use gpui_ce_platform::application;
use scoria_core::account::{Accounts, Player};
use scoria_core::backend::{Appearance, Settings};
use scoria_core::i18n::Language;
use scoria_ui_kit::{card, Button, ButtonSize, ButtonVariant, Theme};

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
}

impl Shell {
    fn new() -> Self {
        let settings = Settings::load();
        let theme = match settings.appearance {
            Appearance::Light => Theme::light(),
            _ => Theme::dark(),
        };
        Self {
            theme,
            screen: Screen::Home,
            settings,
        }
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
            .gap(px(theme.spacing.xs))
            .w(px(232.0))
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
                            .text_size(px(theme.type_scale.subtitle))
                            .child("Scoria"),
                    ),
            );

        for screen in Screen::ALL {
            let selected = screen == self.screen;
            let on_click = cx.listener(move |this, _: &gpui::ClickEvent, _window, cx| {
                this.screen = screen;
                cx.notify();
            });
            rail = rail.child(
                div()
                    .id(("rail", screen as usize))
                    .flex()
                    .items_center()
                    .h(px(36.0))
                    .px(px(theme.spacing.sm))
                    .rounded(px(theme.radii.lg))
                    .bg(if selected {
                        theme.colors.accent_soft
                    } else {
                        theme.colors.surface_secondary
                    })
                    .text_color(if selected {
                        theme.colors.accent
                    } else {
                        theme.colors.muted
                    })
                    .text_size(px(theme.type_scale.body))
                    .cursor_pointer()
                    .hover(move |style| style.bg(theme.colors.surface_tertiary))
                    .on_click(on_click)
                    .child(screen.label()),
            );
        }
        rail.child(div().flex_1())
    }

    fn home(&self) -> impl IntoElement {
        let theme = self.theme;
        let play = Button::new("play", "Play")
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Md)
            .render(
                &theme,
                None::<fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App)>,
            );

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
            .child(card(
                &theme,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(theme.spacing.xs))
                    .child(
                        div()
                            .text_size(px(theme.type_scale.subtitle))
                            .child("Ready"),
                    )
                    .child(
                        div()
                            .text_size(px(theme.type_scale.body))
                            .text_color(theme.colors.muted)
                            .child("The first slice has no game wiring yet."),
                    ),
            ))
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

        let player = match Accounts::load().player() {
            Player::Offline(name) => name,
            Player::GameName => "the game's own name".to_string(),
        };

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
                        .gap(px(theme.spacing.xs))
                        .child(label(&theme, "Player"))
                        .child(div().text_size(px(theme.type_scale.body)).child(player))
                        .child(
                            div()
                                .text_size(px(theme.type_scale.caption))
                                .text_color(theme.colors.muted)
                                .child("Editing the offline name arrives with the text input."),
                        ),
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
            Screen::Home => div().child(self.home()),
            Screen::Mods => div().child(self.placeholder("Mods")),
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

fn main() {
    application().run(|cx| {
        cx.text_system()
            .add_fonts(FONTS.iter().map(|font| Cow::Borrowed(*font)).collect())
            .expect("the bundled Medeo fonts");

        cx.activate(true);
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        // A fixed origin keeps headless screenshots reproducible.
        let bounds = Bounds::new(point(px(0.0), px(0.0)), size(px(1280.0), px(720.0)));
        cx.open_window(
            WindowOptions::new()
                .window_bounds(Some(WindowBounds::Windowed(bounds)))
                .titlebar(None),
            |_, cx| cx.new(|_| Shell::new()),
        )
        .expect("the launcher window");
    });
}

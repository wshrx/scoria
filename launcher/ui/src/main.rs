//! Scoria's launcher window: one process, gpui-ce, no Bun and no native addon.
use gpui::{
    div, prelude::*, px, size, Bounds, Context, IntoElement, Render, Window, WindowBounds,
    WindowOptions,
};
use gpui_ce_platform::application;
use scoria_ui_kit::{card, Button, ButtonSize, ButtonVariant, Theme};

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

struct Shell {
    theme: Theme,
    screen: Screen,
}

impl Shell {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
            screen: Screen::Home,
        }
    }

    fn rail(&self) -> impl IntoElement {
        let theme = self.theme;
        let mut rail = div()
            .flex()
            .flex_col()
            .gap(px(theme.spacing.xs))
            .w(px(220.0))
            .h_full()
            .p(px(theme.spacing.md))
            .bg(theme.colors.surface_secondary)
            .border_r_1()
            .border_color(theme.colors.separator)
            .child(
                div()
                    .px(px(theme.spacing.sm))
                    .py(px(theme.spacing.md))
                    .text_size(px(theme.type_scale.subtitle))
                    .child("Scoria"),
            );

        rail = rail.child(div().flex_1());
        // Navigation is static for now; clicks arrive with the screen state in the next slice.
        for screen in Screen::ALL {
            let selected = screen == self.screen;
            rail = rail.child(
                div()
                    .flex()
                    .items_center()
                    .h(px(36.0))
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
                    .child(screen.label()),
            );
        }
        rail
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let body = match self.screen {
            Screen::Home => div().child(self.home()),
            Screen::Mods => div().child(self.placeholder("Mods")),
            Screen::Servers => div().child(self.placeholder("Servers")),
            Screen::Settings => div().child(self.placeholder("Settings")),
        };

        div()
            .flex()
            .size_full()
            .bg(theme.colors.background)
            .text_color(theme.colors.foreground)
            .child(self.rail())
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
        cx.activate(true);
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        let bounds = Bounds::centered(None, size(px(1280.0), px(720.0)), cx);
        cx.open_window(
            WindowOptions::new()
                .window_bounds(Some(WindowBounds::Windowed(bounds)))
                .titlebar(None),
            |_, cx| cx.new(|_| Shell::new()),
        )
        .expect("the launcher window");
    });
}

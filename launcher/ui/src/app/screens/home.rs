//! Home: the D · Signal layout. A hero and one action on the left, the selected world's poster
//! on the right, and the world list under both. Icons stay single-colour SVG masks in fixed
//! slots, so a label change cannot stretch or move them.
use super::super::*;

impl Shell {
    fn download_info(&self) -> impl IntoElement {
        let theme = self.theme;
        div().w(px(472.0)).child(soft_card(
            &theme,
            div()
                .flex()
                .flex_col()
                .gap(px(theme.spacing.md))
                .child(
                    div()
                        .text_size(px(theme.type_scale.title))
                        .child(self.t("What Download does")),
                )
                .children(
                    [
                        "Fetches Minecraft 26.3 and the Horizon engine from Mojang.",
                        "One download, then the game starts offline.",
                        "Worlds, mods and settings stay in this instance.",
                    ]
                    .map(|line| {
                        div()
                            .flex()
                            .items_center()
                            .gap(px(theme.spacing.sm))
                            .child(
                                div()
                                    .w(px(4.0))
                                    .h(px(4.0))
                                    .rounded_full()
                                    .bg(theme.colors.accent),
                            )
                            .child(
                                div()
                                    .text_size(px(theme.type_scale.body))
                                    .text_color(theme.colors.muted)
                                    .child(self.t(line)),
                            )
                    }),
                ),
        ))
    }

    fn poster(
        &self,
        world: Option<&serde_json::Value>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.theme;
        let name = world
            .and_then(|world| world["name"].as_str())
            .unwrap_or("Minecraft 26.3");
        let mode = world
            .and_then(|world| world["mode"].as_str())
            .unwrap_or("Survival");
        let played = world.map(|world| self.ago(world)).unwrap_or_default();
        let folder = world
            .and_then(|world| world["folder"].as_str())
            .map(str::to_string);
        let open = Button::new("open-world", self.t("Open world"))
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Sm)
            .icon_after(ICON_ARROW_UP_RIGHT)
            .render(
                &theme,
                Some(cx.listener(move |this, _, _window, cx| {
                    if let Some(folder) = folder.clone() {
                        this.play_world(Some(folder), cx);
                    }
                })),
            );

        div()
            .relative()
            .w(px(472.0))
            .h(px(560.0))
            .rounded(px(theme.radii.xl))
            .overflow_hidden()
            .bg(theme.colors.surface_secondary)
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
                .h_full()
                .rounded(px(theme.radii.xl))
                .object_fit(ObjectFit::Cover),
            )
            .child(
                div()
                    .absolute()
                    .left(px(0.0))
                    .right(px(0.0))
                    .bottom(px(0.0))
                    .h(px(260.0))
                    .bg(linear_gradient(
                        180.0,
                        linear_color_stop(
                            if theme.dark {
                                rgba(0x0B0B0D00)
                            } else {
                                rgba(0xF1F1F100)
                            },
                            0.0,
                        ),
                        linear_color_stop(
                            if theme.dark {
                                rgba(0x0B0B0DE6)
                            } else {
                                rgba(0xF1F1F1E6)
                            },
                            1.0,
                        ),
                    )),
            )
            .child(
                div()
                    .absolute()
                    .top(px(24.0))
                    .left(px(24.0))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .h(px(32.0))
                    .px(px(12.0))
                    .rounded(px(theme.radii.md))
                    .bg(if theme.dark {
                        rgba(0x0B0B0DCC)
                    } else {
                        rgba(0xFFFFFFE6)
                    })
                    .text_size(px(theme.type_scale.caption))
                    .child(
                        div()
                            .w(px(6.0))
                            .h(px(6.0))
                            .rounded_full()
                            .bg(theme.colors.accent),
                    )
                    .child(self.t("Selected world")),
            )
            .child(
                div()
                    .absolute()
                    .left(px(24.0))
                    .right(px(24.0))
                    .bottom(px(24.0))
                    .flex()
                    .flex_col()
                    .gap(px(theme.spacing.sm))
                    .p(px(20.0))
                    .rounded(px(theme.radii.lg))
                    .border_1()
                    .border_color(theme.colors.border)
                    .bg(if theme.dark {
                        rgba(0x0B0B0DEE)
                    } else {
                        rgba(0xFFFFFFF2)
                    })
                    .backdrop_blur(px(24.0))
                    .child(
                        div()
                            .text_size(px(28.0))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child(name.to_string()),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .text_size(px(theme.type_scale.caption))
                            .text_color(theme.colors.muted)
                            .child(
                                gpui::svg()
                                    .data(ICON_CLOCK)
                                    .size(px(14.0))
                                    .text_color(theme.colors.muted),
                            )
                            .child(format!("{} · {}", self.t(mode_key(mode)), played)),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .h(px(28.0))
                                    .px(px(9.0))
                                    .rounded(px(theme.radii.sm))
                                    .bg(theme.colors.surface_secondary)
                                    .text_size(px(10.0))
                                    .text_color(theme.colors.muted)
                                    .child(
                                        gpui::svg()
                                            .data(ICON_MAP_PIN)
                                            .size(px(12.0))
                                            .text_color(theme.colors.muted),
                                    )
                                    .child(self.t("Local world")),
                            )
                            .child(open),
                    ),
            )
    }

    fn world_row(
        &self,
        index: usize,
        world: &serde_json::Value,
        selected: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.theme;
        let name = world["name"]
            .as_str()
            .map(str::to_string)
            .unwrap_or_else(|| self.t("World"));
        let mode = world["mode"].as_str().unwrap_or("Survival");
        let played = self.ago(world);
        let folder = world["folder"].as_str().map(str::to_string);
        let initial = name
            .chars()
            .find(|character| character.is_alphanumeric())
            .map(|character| character.to_uppercase().to_string())
            .unwrap_or_else(|| "?".to_string());
        let on_click = cx.listener(move |this, _, _window, cx| {
            this.selected_world = folder.clone();
            cx.notify();
        });

        div()
            .id(("world-row", index))
            .relative()
            .flex()
            .items_center()
            .gap(px(16.0))
            .h(px(64.0))
            .px(px(12.0))
            .rounded(px(theme.radii.lg))
            .bg(if selected {
                theme.colors.surface_secondary
            } else {
                rgba(0x00000000)
            })
            .cursor_pointer()
            .hover(move |style| style.bg(theme.colors.surface_secondary))
            .on_click(on_click)
            .children(selected.then(|| {
                div()
                    .absolute()
                    .left(px(0.0))
                    .top(px(12.0))
                    .bottom(px(12.0))
                    .w(px(2.0))
                    .rounded_full()
                    .bg(theme.colors.accent)
            }))
            .child(
                div()
                    .relative()
                    .w(px(64.0))
                    .h(px(48.0))
                    .rounded(px(theme.radii.md))
                    .overflow_hidden()
                    .bg(linear_gradient(
                        160.0,
                        linear_color_stop(
                            if theme.dark {
                                rgb(0x3A2116)
                            } else {
                                rgb(0xDCDCDC)
                            },
                            0.0,
                        ),
                        linear_color_stop(
                            if theme.dark {
                                rgb(0x141114)
                            } else {
                                rgb(0xBFBFBF)
                            },
                            1.0,
                        ),
                    ))
                    .child(
                        div()
                            .absolute()
                            .top(px(6.0))
                            .left(px(8.0))
                            .text_size(px(22.0))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(if theme.dark {
                                rgba(0xFFFFFF38)
                            } else {
                                rgba(0x00000030)
                            })
                            .child(initial),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .flex_1()
                    .min_w(px(0.0))
                    .child(
                        div()
                            .truncate()
                            .text_size(px(theme.type_scale.subtitle))
                            .child(name),
                    )
                    .child(
                        div()
                            .truncate()
                            .text_size(px(theme.type_scale.caption))
                            .text_color(theme.colors.muted)
                            .child(format!("{} · {}", self.t(mode_key(mode)), played)),
                    ),
            )
            .child(
                gpui::svg()
                    .data(ICON_CLOCK)
                    .size(px(14.0))
                    .text_color(theme.colors.muted),
            )
            .child(
                div()
                    .font_family(theme.type_scale.mono)
                    .text_size(px(10.0))
                    .text_color(theme.colors.muted)
                    .child(played.clone()),
            )
            .child(
                gpui::svg()
                    .data(ICON_CHEVRON)
                    .size(px(14.0))
                    .text_color(theme.colors.muted),
            )
    }

    pub(super) fn home(
        &self,
        cx: &mut Context<Self>,
        compact: bool,
        wide: bool,
    ) -> impl IntoElement {
        let theme = self.theme;
        let (prepared, playing) = match &self.snapshot {
            Ok(snapshot) => (
                snapshot["prepared"].as_bool().unwrap_or(false),
                snapshot["playing"].as_bool().unwrap_or(false),
            ),
            Err(_) => (false, false),
        };
        let no_version = self.snapshot.is_err();
        let worlds: Vec<serde_json::Value> = match &self.snapshot {
            Ok(snapshot) => snapshot["worlds"].as_array().cloned().unwrap_or_default(),
            Err(_) => Vec::new(),
        };
        let selected = self
            .selected_world
            .as_deref()
            .and_then(|folder| {
                worlds
                    .iter()
                    .find(|world| world["folder"].as_str() == Some(folder))
            })
            .or_else(|| worlds.first());
        let selected_played = selected.map(|world| self.ago(world)).unwrap_or_default();
        let failed = matches!(&self.play_message, Some((false, _)));
        let percent = clamp_fraction(self.prepare_percent.unwrap_or(0.0));

        let (eyebrow, status_text, status_icon, status_tone) = if no_version {
            (
                self.t("Install").to_uppercase(),
                self.t("No version installed. Download 26.3 to install the engine."),
                ICON_DOWNLOAD,
                theme.colors.muted,
            )
        } else if self.preparing {
            (
                self.t("Preparing").to_uppercase(),
                self.t("Downloading 26.3"),
                ICON_DOWNLOAD,
                theme.colors.accent,
            )
        } else if playing {
            (
                self.t("In game now").to_uppercase(),
                self.t("In game"),
                ICON_PLAY,
                theme.colors.success,
            )
        } else if failed {
            (
                self.t("Error").to_uppercase(),
                self.t("The game did not start"),
                ICON_REFRESH,
                theme.colors.danger,
            )
        } else {
            (
                if selected_played.is_empty() {
                    self.t("Ready to play").to_uppercase()
                } else {
                    format!(
                        "{} · {}",
                        self.t("Last played").to_uppercase(),
                        selected_played
                    )
                },
                self.t("Ready to launch"),
                ICON_CHECK,
                theme.colors.success,
            )
        };

        let (label, variant, icon, progress) = if playing {
            (
                self.t("Stop"),
                ButtonVariant::AccentOutline,
                Some(ICON_STOP),
                None,
            )
        } else if self.preparing {
            (
                format!("{} {:.0}%", self.t("Preparing"), percent * 100.0),
                ButtonVariant::AccentOutline,
                None,
                Some(percent),
            )
        } else if failed {
            (
                self.t("Try again"),
                ButtonVariant::DangerOutline,
                Some(ICON_REFRESH),
                None,
            )
        } else if prepared {
            (
                self.t("Play"),
                ButtonVariant::Primary,
                Some(ICON_PLAY),
                None,
            )
        } else {
            (
                self.t("Download 26.3"),
                ButtonVariant::Primary,
                Some(ICON_DOWNLOAD),
                None,
            )
        };
        let mut action = Button::new("game-action", label)
            .variant(variant)
            .size(ButtonSize::Xl)
            .width(192.0);
        if let Some(icon) = icon {
            action = action.icon(icon);
        }
        if let Some(progress) = progress {
            action = action.progress(progress);
        }
        let action = action.render(
            &theme,
            Some(cx.listener(|this, _: &gpui::ClickEvent, _window, cx| this.game_action(cx))),
        );

        let settings = Button::new("home-settings", self.t("Settings"))
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Md)
            .icon_after(ICON_CHEVRON)
            .render(
                &theme,
                Some(cx.listener(|this, _, _window, cx| this.go(Screen::Settings, cx))),
            );

        let stats = if self.preparing {
            format!(
                "{:.0}% · {}",
                percent * 100.0,
                self.prepare_status
                    .as_deref()
                    .map(|line| scoria_core::i18n::tr_in(self.language(), line))
                    .unwrap_or("26.3")
            )
        } else {
            "26.3".to_string()
        };
        let cancel = (self.preparing).then(|| {
            Button::new("cancel-prepare", self.t("Cancel"))
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Sm)
                .render(
                    &theme,
                    Some(cx.listener(|this, _, _window, cx| {
                        scoria_core::api::cancel_prepare();
                        this.show_toast(Tone::Neutral, this.t("Cancelling…"), cx);
                    })),
                )
        });

        let mut rows = div().flex().flex_col().gap(px(2.0));
        if worlds.is_empty() {
            rows = rows.child(empty_state(
                &theme,
                ICON_PLAY,
                self.t("No worlds yet"),
                self.t("Start the game and create a world."),
            ));
        } else {
            for (index, world) in worlds.iter().take(4).enumerate() {
                let is_selected = selected
                    .and_then(|selected| selected["folder"].as_str())
                    .zip(world["folder"].as_str())
                    .is_some_and(|(a, b)| a == b);
                rows = rows.child(self.world_row(index, world, is_selected, cx));
            }
        }

        let hero = div()
            .flex()
            .flex_col()
            .gap(px(if compact { 16.0 } else { 20.0 }))
            .w(px(if wide { 640.0 } else { 0.0 }))
            .when(!wide, |hero| hero.w_full())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .font_family(theme.type_scale.mono)
                            .text_size(px(10.0))
                            .text_color(theme.colors.muted)
                            .child(eyebrow),
                    )
                    .child(
                        div()
                            .text_size(px(if compact { 48.0 } else { theme.type_scale.hero }))
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Minecraft 26.3"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .text_size(px(theme.type_scale.body_lg))
                            .text_color(theme.colors.muted)
                            .child(self.t("Your worlds, mods and servers."))
                            .child(self.t("No Microsoft account and no telemetry.")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(24.0))
                    .child(action)
                    .child(settings),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        gpui::svg()
                            .data(status_icon)
                            .size(px(14.0))
                            .text_color(status_tone),
                    )
                    .child(
                        div()
                            .truncate()
                            .text_size(px(theme.type_scale.body))
                            .child(status_text),
                    )
                    .child(
                        div()
                            .font_family(theme.type_scale.mono)
                            .text_size(px(10.0))
                            .text_color(theme.colors.muted)
                            .child(stats),
                    )
                    .children(cancel),
            )
            .child(div().h(px(1.0)).w_full().bg(theme.colors.separator));
        let worlds_block = div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        div()
                            .text_size(px(theme.type_scale.title))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child(self.t("Your worlds")),
                    )
                    .child(
                        div()
                            .font_family(theme.type_scale.mono)
                            .text_size(px(10.0))
                            .text_color(theme.colors.muted)
                            .child(worlds.len().to_string()),
                    )
                    .child(div().flex_1())
                    .child(
                        div()
                            .text_size(px(theme.type_scale.caption))
                            .text_color(theme.colors.muted)
                            .child(self.t("All worlds")),
                    ),
            )
            .child(rows);
        let hero = if no_version {
            hero
        } else {
            hero.child(worlds_block)
        };

        let poster: Option<gpui::AnyElement> = if no_version {
            wide.then(|| self.download_info().into_any_element())
        } else {
            wide.then(|| self.poster(selected, cx).into_any_element())
        };
        let inner = if wide {
            div()
                .flex()
                .items_start()
                .justify_center()
                .gap(px(40.0))
                .w_full()
                .child(hero)
                .children(poster)
        } else {
            div().w_full().child(hero)
        };
        div()
            .flex()
            .justify_center()
            .w_full()
            .child(inner.max_w(px(1152.0)))
    }
}

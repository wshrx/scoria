//! Home: the one game action, its download card, and the instance's worlds.
use super::super::*;

impl Shell {
    /// The download is a card, not a line of text: a label, the engine's last line, the
    /// percentage, a full-width track and a quiet cancel action.
    fn download_card(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let percent = clamp_fraction(self.prepare_percent.unwrap_or(0.0));
        let line = self
            .prepare_status
            .clone()
            .unwrap_or_else(|| self.t("Preparing game files"));
        // Engine log lines are English; known ones have a table entry.
        let line = scoria_core::i18n::tr_in(self.language(), &line).to_string();
        let cancel = Button::new("cancel-prepare", self.t("Cancel"))
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Sm)
            .render(
                &theme,
                Some(cx.listener(|this, _, _window, cx| {
                    scoria_core::api::cancel_prepare();
                    this.show_toast(Tone::Neutral, this.t("Cancelling…"), cx);
                })),
            );
        div().w(px(560.0)).child(soft_card(
            &theme,
            div()
                .flex()
                .flex_col()
                .gap(px(theme.spacing.md))
                .child(
                    div()
                        .flex()
                        .items_start()
                        .justify_between()
                        .gap(px(theme.spacing.lg))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(theme.spacing.xs))
                                .flex_1()
                                .min_w(px(0.0))
                                .child(
                                    div()
                                        .text_size(px(theme.type_scale.subtitle))
                                        .child(self.t("Downloading")),
                                )
                                .child(
                                    div()
                                        .truncate()
                                        .text_size(px(theme.type_scale.caption))
                                        .text_color(theme.colors.muted)
                                        .child(line),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_end()
                                .gap(px(theme.spacing.xs))
                                .child(
                                    div()
                                        .font_family(theme.type_scale.mono)
                                        .text_size(px(theme.type_scale.title))
                                        .child(format!("{:.0}%", percent * 100.0)),
                                )
                                .child(cancel),
                        ),
                )
                .child(progress_bar(&theme, percent)),
        ))
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

        let action_label = if playing {
            self.t("Playing")
        } else if self.preparing {
            self.t("Preparing…")
        } else if prepared {
            self.t("Play")
        } else {
            self.t("Download 26.3")
        };
        let action = Button::new("game-action", action_label)
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Lg)
            .disabled(playing || self.preparing)
            .render(
                &theme,
                Some(cx.listener(|this, _: &gpui::ClickEvent, _window, cx| this.game_action(cx))),
            );

        let mut chips = div().flex().items_center().gap(px(theme.spacing.xs));
        let (chip, chip_tone) = if no_version {
            (self.t("Not installed"), Tone::Danger)
        } else if playing {
            (self.t("Playing"), Tone::Accent)
        } else if prepared {
            (self.t("Prepared"), Tone::Success)
        } else {
            (self.t("Not prepared"), Tone::Warning)
        };
        if !playing {
            chips = chips.child(badge(&theme, chip, chip_tone));
        }

        let mut tiles = div().flex().gap(px(theme.spacing.md));
        for world in worlds.iter().take(4) {
            let name = world["name"]
                .as_str()
                .map(str::to_string)
                .unwrap_or_else(|| self.t("World"));
            let played = world["lastPlayed"].as_str().unwrap_or("");
            let mode = world["mode"].as_str().unwrap_or("Survival");
            let initial = name
                .chars()
                .find(|character| character.is_alphanumeric())
                .map(|character| character.to_uppercase().to_string())
                .unwrap_or_else(|| "?".to_string());
            tiles = tiles.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(theme.spacing.xs))
                    .w(px(156.0))
                    .child(
                        div()
                            .relative()
                            .w_full()
                            .h(px(92.0))
                            .rounded(px(theme.radii.lg))
                            .bg(linear_gradient(
                                160.0,
                                linear_color_stop(rgb(0x3A2116), 0.0),
                                linear_color_stop(rgb(0x141114), 1.0),
                            ))
                            .border_1()
                            .border_color(rgba(0xFFFFFF10))
                            .child(
                                div()
                                    .absolute()
                                    .top(px(10.0))
                                    .left(px(12.0))
                                    .text_size(px(30.0))
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(rgba(0xFFFFFF38))
                                    .child(initial),
                            )
                            .child(
                                div()
                                    .absolute()
                                    .bottom(px(8.0))
                                    .left(px(8.0))
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .rounded(px(theme.radii.sm))
                                    .bg(rgba(0x09080ACC))
                                    .text_size(px(10.0))
                                    .text_color(theme.colors.muted)
                                    .child(self.t(match mode {
                                        "Creative" => "Creative",
                                        "Hardcore" => "Hardcore",
                                        "Adventure" => "Adventure",
                                        _ => "Survival",
                                    })),
                            ),
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
            if *ok {
                status = status.child(
                    div()
                        .text_size(px(theme.type_scale.caption))
                        .text_color(theme.colors.success)
                        .child(text.clone()),
                );
            } else {
                let retry = Button::new("retry-game", self.t("Try again"))
                    .variant(ButtonVariant::Secondary)
                    .size(ButtonSize::Md)
                    .render(
                        &theme,
                        Some(cx.listener(|this, _: &gpui::ClickEvent, _window, cx| {
                            this.game_action(cx)
                        })),
                    );
                status = status.child(
                    div().w(px(480.0)).child(
                        error_panel(&theme, self.t("The game did not start"), self.td(text))
                            .child(div().self_start().child(retry)),
                    ),
                );
            }
        }

        let content = div()
            .flex()
            .flex_col()
            .gap(px(theme.spacing.xl))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(theme.spacing.sm))
                    .child(
                        div()
                            .text_size(px(if compact { 30.0 } else { 44.0 }))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("Minecraft 26.3"),
                    )
                    .child(
                        div()
                            .text_size(px(theme.type_scale.body))
                            .text_color(theme.colors.muted)
                            .child(if no_version {
                                self.t("No version installed. Download 26.3 to install the engine.")
                            } else {
                                self.t("Offline profile. No Microsoft account.")
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(theme.spacing.sm))
                    .child(action),
            )
            .children((!playing && !self.preparing).then(|| chips))
            .children(playing.then(|| {
                div()
                    .text_size(px(theme.type_scale.body))
                    .text_color(theme.colors.muted)
                    .child(self.t("The game is running in another window."))
            }))
            .children(no_version.then(|| {
                div().w(px(560.0)).child(soft_card(
                    &theme,
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(theme.spacing.sm))
                        .w(px(520.0))
                        .child(
                            div()
                                .text_size(px(theme.type_scale.subtitle))
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
            }))
            .children(self.preparing.then(|| self.download_card(cx)))
            .child(status);

        let worlds_section = (!no_version).then(|| {
            div()
                .flex()
                .flex_col()
                .gap(px(theme.spacing.sm))
                .child(
                    div()
                        .text_size(px(theme.type_scale.subtitle))
                        .child(self.t("Your worlds")),
                )
                .child(if worlds.is_empty() {
                    empty_state(
                        &theme,
                        ICON_PLAY,
                        self.t("No worlds yet"),
                        self.t("Start the game and create a world."),
                    )
                    .max_w(px(560.0))
                    .into_any_element()
                } else {
                    tiles.into_any_element()
                })
        });

        if wide {
            let mut layout = div()
                .flex()
                .items_start()
                .gap(px(48.0))
                .max_w(px(1320.0))
                .child(content.w(px(620.0)));
            if let Some(worlds) = worlds_section {
                layout = layout.child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_w(px(0.0))
                        .child(worlds),
                );
            }
            layout
        } else {
            content.max_w(px(920.0)).children(worlds_section)
        }
    }
}

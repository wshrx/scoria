//! Appearance, language and the offline nickname.
use super::super::*;

impl Shell {
    pub(super) fn settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;

        let appearance_label = match self.settings.appearance {
            Appearance::Dark => self.t("Dark"),
            Appearance::Light => self.t("Light"),
            Appearance::System => self.t("System"),
        };
        let mut appearance_menu = div()
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
        let mut appearance = div()
            .flex()
            .flex_col()
            .gap(px(theme.spacing.xs))
            .w(px(220.0))
            .child(
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
            ("opt-french", Some(Language::French), "Français"),
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
        let mut language = div()
            .flex()
            .flex_col()
            .gap(px(theme.spacing.xs))
            .w(px(220.0))
            .child(
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

        div().flex().justify_center().w_full().child(
            card(
                &theme,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(theme.spacing.lg))
                    .child(page_header(
                        &theme,
                        self.t("Settings"),
                        self.t("Appearance and language are saved to the launcher's settings."),
                    ))
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
                                        self.t(
                                            "No Microsoft account: this name is what servers see.",
                                        ),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(theme.spacing.sm))
                                    .child(
                                        field(
                                            &theme,
                                            "offline-name",
                                            self.name.downgrade(),
                                            self.t("Your offline name"),
                                        )
                                        .w(px(280.0)),
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
                    )
                    .child(
                        div()
                            .text_size(px(theme.type_scale.caption))
                            .text_color(theme.colors.muted)
                            .child(self.t("Ctrl+1–4 screens · Ctrl+Enter start · Esc close")),
                    ),
            )
            .w(px(880.0)),
        )
    }
}

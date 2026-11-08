//! Appearance, language and the offline nickname.
use super::super::*;
use scoria_ui_kit::field;

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
                    .size(ButtonSize::Lg)
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
                    .size(ButtonSize::Lg)
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

        div()
            .id("settings-scroll")
            .flex()
            .justify_center()
            .items_start()
            .w_full()
            .h_full()
            .overflow_y_scroll()
            .py(px(theme.spacing.md))
            .child(
            card(
                &theme,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(theme.spacing.md))
                    .child(page_header(
                        &theme,
                        self.t("Settings"),
                        self.t("Appearance and language are saved to the launcher's settings."),
                    ))
                    .child(
                        div()
                            .flex()
                            .gap(px(theme.spacing.lg))
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
                            ),
                    )
                    .child({
                        let mut microsoft = div()
                            .flex()
                            .flex_col()
                            .gap(px(theme.spacing.sm))
                            .child(label(&theme, &self.t("Microsoft account")))
                            .child(
                                div()
                                    .text_size(px(theme.type_scale.caption))
                                    .text_color(theme.colors.muted)
                                    .child(self.t(
                                        "Sign in to play online-mode servers with your name and skin.",
                                    )),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(theme.spacing.sm))
                                    .child(
                                        Button::new(
                                            "microsoft-sign-in",
                                            if self.sign_in_running {
                                                self.t("Cancel sign-in")
                                            } else {
                                                self.t("Sign in with Microsoft")
                                            },
                                        )
                                        .variant(if self.sign_in_running {
                                            ButtonVariant::Secondary
                                        } else {
                                            ButtonVariant::Primary
                                        })
                                        .size(ButtonSize::Lg)
                                        .render(
                                            &theme,
                                            Some(cx.listener(|this, _, _window, cx| {
                                                if this.sign_in_running {
                                                    this.cancel_sign_in(cx);
                                                } else {
                                                    this.sign_in_microsoft(cx);
                                                }
                                            })),
                                        ),
                                    )
                                    .children(self.sign_in_code.as_ref().map(|(code, uri)| {
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(theme.spacing.sm))
                                            .child(
                                                div()
                                                    .text_size(px(theme.type_scale.title))
                                                    .font_family(theme.type_scale.mono)
                                                    .child(code.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(theme.type_scale.caption))
                                                    .text_color(theme.colors.muted)
                                                    .child(uri.clone()),
                                            )
                                    })),
                            )
                            .child(match &self.sign_in_message {
                                Some((ok, text)) => div()
                                    .text_size(px(theme.type_scale.caption))
                                    .text_color(if *ok {
                                        theme.colors.muted
                                    } else {
                                        theme.colors.danger
                                    })
                                    .child(text.clone()),
                                None => div(),
                            });
                        if self.accounts.online.is_empty() {
                            microsoft = microsoft.child(
                                div()
                                    .text_size(px(theme.type_scale.caption))
                                    .text_color(theme.colors.muted)
                                    .child(self.t("No Microsoft accounts signed in.")),
                            );
                        } else {
                            let mut list = div().flex().flex_col().gap(px(4.0)).w(px(460.0));
                            for account in &self.accounts.online {
                                let selected = self.accounts.online_selected.as_deref()
                                    == Some(account.uuid.as_str());
                                let mut actions = div().flex().items_center().gap(px(4.0));
                                if !selected {
                                    let uuid = account.uuid.clone();
                                    actions = actions.child(
                                        Button::new(
                                            format!("use-online-{}", account.uuid),
                                            self.t("Use"),
                                        )
                                        .variant(ButtonVariant::Secondary)
                                        .size(ButtonSize::Sm)
                                        .render(
                                            &theme,
                                            Some(cx.listener(move |this, _, _window, cx| {
                                                this.use_online_account(uuid.clone(), cx)
                                            })),
                                        ),
                                    );
                                }
                                let uuid = account.uuid.clone();
                                actions = actions.child(
                                    Button::new(
                                        format!("sign-out-online-{}", account.uuid),
                                        self.t("Sign out"),
                                    )
                                    .variant(ButtonVariant::Ghost)
                                    .size(ButtonSize::Sm)
                                    .render(
                                        &theme,
                                        Some(cx.listener(move |this, _, _window, cx| {
                                            this.sign_out_online(uuid.clone(), cx)
                                        })),
                                    ),
                                );
                                list = list.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .h(px(44.0))
                                        .px(px(theme.spacing.md))
                                        .rounded(px(theme.radii.md))
                                        .bg(theme.colors.surface_secondary)
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(theme.type_scale.body))
                                                        .child(account.name.clone()),
                                                )
                                                .child(badge(
                                                    &theme,
                                                    self.t("Microsoft"),
                                                    Tone::Accent,
                                                ))
                                                .children(selected.then(|| {
                                                    badge(
                                                        &theme,
                                                        self.t("Selected"),
                                                        Tone::Success,
                                                    )
                                                })),
                                        )
                                        .child(actions),
                                );
                            }
                            microsoft = microsoft.child(list);
                        }
                        microsoft
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(theme.spacing.sm))
                            .child(label(&theme, &self.t("Offline account")))
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
                            .children((!self.accounts.offline.is_empty()).then(|| {
                                let mut list = div().flex().flex_col().gap(px(4.0)).w(px(460.0));
                                for account in &self.accounts.offline {
                                    let label = account.clone();
                                    let selected =
                                        self.accounts.offline_selected.as_deref() == Some(account);
                                    let mut actions = div().flex().items_center().gap(px(4.0));
                                    if !selected {
                                        let account = account.clone();
                                        actions = actions.child(
                                            Button::new(
                                                format!("use-account-{account}"),
                                                self.t("Use"),
                                            )
                                            .variant(ButtonVariant::Secondary)
                                            .size(ButtonSize::Sm)
                                            .render(
                                                &theme,
                                                Some(cx.listener(move |this, _, _window, cx| {
                                                    this.use_account(account.clone(), cx)
                                                })),
                                            ),
                                        );
                                    }
                                    let account = account.clone();
                                    actions = actions.child(
                                        Button::new(
                                            format!("remove-account-{account}"),
                                            self.t("Remove"),
                                        )
                                        .variant(ButtonVariant::Ghost)
                                        .size(ButtonSize::Sm)
                                        .render(
                                            &theme,
                                            Some(cx.listener(move |this, _, _window, cx| {
                                                this.remove_account(account.clone(), cx)
                                            })),
                                        ),
                                    );
                                    list = list.child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .h(px(44.0))
                                            .px(px(theme.spacing.md))
                                            .rounded(px(theme.radii.md))
                                            .bg(theme.colors.surface_secondary)
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(8.0))
                                                    .child(
                                                        div()
                                                            .text_size(px(theme.type_scale.body))
                                                            .child(label),
                                                    )
                                                    .children(selected.then(|| {
                                                        badge(
                                                            &theme,
                                                            self.t("Selected"),
                                                            Tone::Success,
                                                        )
                                                    })),
                                            )
                                            .child(actions),
                                    );
                                }
                                list
                            }))
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
                                            self.t("New offline name"),
                                        )
                                        .w(px(280.0)),
                                    )
                                    .child(
                                        Button::new("save-name", self.t("Add account"))
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
            )
            .w(px(880.0)),
        )
    }
}

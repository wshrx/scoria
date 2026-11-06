//! The mod catalogue: Horizon first, Modrinth second, with loading and error states.
use super::super::*;

impl Shell {
    pub(super) fn mods(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            for _ in 0..3 {
                results = results.child(soft_card(
                    &theme,
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(theme.spacing.sm))
                        .child(
                            div()
                                .h(px(16.0))
                                .w(px(180.0))
                                .rounded_full()
                                .bg(theme.colors.surface_tertiary),
                        )
                        .child(
                            div()
                                .h(px(10.0))
                                .w_full()
                                .rounded_full()
                                .bg(theme.colors.surface_secondary),
                        ),
                ));
            }
        }
        if let Some(error) = &self.mods_error {
            let retry = Button::new("retry-mods", self.t("Search"))
                .variant(ButtonVariant::Secondary)
                .size(ButtonSize::Md)
                .render(
                    &theme,
                    Some(cx.listener(|this, _, _window, cx| this.search_mods(cx))),
                );
            results = results.child(
                error_panel(
                    &theme,
                    self.t("Could not load the catalogue"),
                    self.td(error),
                )
                .child(div().self_start().child(retry)),
            );
        }
        if self.mods_results.is_empty() && !self.mods_loading && self.mods_error.is_none() {
            results = results.child(empty_state(
                &theme,
                ICON_SEARCH,
                self.t("Nothing yet"),
                self.t("Search the registry or type a mod name."),
            ));
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
            let title = project["title"]
                .as_str()
                .map(str::to_string)
                .unwrap_or_else(|| self.t("Untitled"));
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
                                    .child(title.clone()),
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
                                    .child(if author == source {
                                        self.tf("by {author}", &[("author", author)])
                                    } else {
                                        self.tf(
                                            "by {author} · {source}",
                                            &[("author", author), ("source", source)],
                                        )
                                    }),
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

        div().flex().justify_center().w_full().child(card(
            &theme,
            div()
                .flex()
                .flex_col()
                .gap(px(theme.spacing.lg))
                .child(
                    page_header(
                        &theme,
                        self.t("Mods"),
                        self.t("Horizon's registry first, then Modrinth. Every result says how it runs on Scoria."),
                    ),
                )
                .child(catalogue_tabs(self, cx))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(theme.spacing.sm))
                        .child(
                            field(
                                &theme,
                                "mods-query",
                                self.mods_query.downgrade(),
                                self.t("Search mods"),
                            )
                            .flex_1(),
                        )
                        .child(search),
                )
                .child(results),
        )
        .w_full()
        .max_w(px(1100.0)))
    }
}

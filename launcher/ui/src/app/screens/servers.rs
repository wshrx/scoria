//! Saved servers: add, join and remove, with their empty and error states.
use super::super::*;

impl Shell {
    pub(super) fn servers_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            let retry = Button::new("retry-servers", self.t("Try again"))
                .variant(ButtonVariant::Secondary)
                .size(ButtonSize::Md)
                .render(
                    &theme,
                    Some(cx.listener(|this, _, _window, cx| {
                        this.refresh_servers();
                        cx.notify();
                    })),
                );
            rows = rows.child(
                error_panel(&theme, self.t("Could not load the servers"), self.td(error))
                    .child(div().self_start().child(retry)),
            );
        }
        if list.is_empty() && error.is_none() {
            rows = rows.child(empty_state(
                &theme,
                ICON_SERVERS,
                self.t("No servers yet"),
                self.t("No saved servers yet. Add the address of an offline-mode server."),
            ));
        }
        for server in &list {
            let index = server["index"].as_u64().unwrap_or(0) as usize;
            let name = server["name"]
                .as_str()
                .map(str::to_string)
                .unwrap_or_else(|| self.t("Server"));
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
                                    .child(name.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(theme.type_scale.caption))
                                    .text_color(theme.colors.muted)
                                    .child(ip.to_string()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(theme.spacing.xs))
                            .child(
                                Button::new(format!("join-{index}"), self.t("Join"))
                                    .variant(ButtonVariant::Secondary)
                                    .size(ButtonSize::Sm)
                                    .render(
                                        &theme,
                                        Some(cx.listener({
                                            let ip = ip.to_string();
                                            move |this, _, _window, cx| {
                                                this.join_server(ip.clone(), cx)
                                            }
                                        })),
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
                    ),
            ));
        }

        card(
            &theme,
            div()
                .flex()
                .flex_col()
                .gap(px(theme.spacing.lg))
                .child(page_header(
                    &theme,
                    self.t("Servers"),
                    self.t("Saved servers for this instance. LAN worlds appear inside the game."),
                ))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(theme.spacing.sm))
                        .child(
                            field(
                                &theme,
                                "server-name",
                                self.server_name.downgrade(),
                                self.t("Name"),
                            )
                            .flex_1(),
                        )
                        .child(
                            field(
                                &theme,
                                "server-ip",
                                self.server_ip.downgrade(),
                                self.t("host:port"),
                            )
                            .flex_1(),
                        )
                        .child(add),
                )
                .child(rows),
        )
        .max_w(px(880.0))
    }
}

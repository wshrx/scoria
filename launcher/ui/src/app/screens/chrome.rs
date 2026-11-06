//! The window's topbar: brand, the three screens, search, status and the system controls.
//! There is no rail; the redesign gives the content the full width.
use super::super::*;

impl Shell {
    fn mark() -> impl IntoElement {
        img(ImageSource::Image(Arc::new(Image::from_bytes(
            ImageFormat::Png,
            MARK.to_vec(),
        ))))
        .w(px(22.0))
        .h(px(22.0))
    }

    fn status(&self) -> (String, gpui::Rgba) {
        let theme = self.theme;
        let playing = self
            .snapshot
            .as_ref()
            .ok()
            .and_then(|snapshot| snapshot["playing"].as_bool())
            .unwrap_or(false);
        if self.preparing {
            (self.t("Downloading"), theme.colors.accent)
        } else if playing {
            (self.t("In game"), theme.colors.success)
        } else if self.snapshot.is_err() {
            (self.t("Not installed"), theme.colors.danger)
        } else {
            (self.t("Offline"), theme.colors.success)
        }
    }

    pub(super) fn topbar(&self, cx: &mut Context<Self>, compact: bool) -> impl IntoElement {
        let theme = self.theme;
        let (status, status_color) = self.status();
        let control =
            |id: &'static str, icon: &'static [u8], hover: gpui::Rgba, action: fn(&mut Window)| {
                div()
                    .id(id)
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(34.0))
                    .h(px(28.0))
                    .rounded(px(theme.radii.md))
                    .cursor_pointer()
                    .hover(move |style| style.bg(hover))
                    .on_click(move |_, window, _| action(window))
                    .child(
                        gpui::svg()
                            .data(icon)
                            .size(px(12.0))
                            .text_color(theme.colors.muted),
                    )
            };

        let mut nav = div().flex().items_center().gap(px(4.0));
        for screen in [Screen::Home, Screen::Mods, Screen::Servers] {
            let selected = self.screen == screen;
            let on_click =
                cx.listener(move |this, _: &gpui::ClickEvent, _window, cx| this.go(screen, cx));
            let tint = if selected {
                theme.colors.accent
            } else {
                theme.colors.muted
            };
            nav = nav.child(
                div()
                    .id(("topbar-nav", screen as usize))
                    .relative()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(8.0))
                    .h(px(56.0))
                    .px(px(if compact { 10.0 } else { 14.0 }))
                    .cursor_pointer()
                    .text_color(if selected {
                        theme.colors.foreground
                    } else {
                        theme.colors.muted
                    })
                    .hover(move |style| style.text_color(theme.colors.foreground))
                    .on_click(on_click)
                    .child(
                        gpui::svg()
                            .data(screen.icon())
                            .size(px(14.0))
                            .text_color(tint),
                    )
                    .children((!compact).then(|| self.t(screen.label())))
                    .children(selected.then(|| {
                        div()
                            .absolute()
                            .bottom(px(0.0))
                            .left(px(14.0))
                            .right(px(14.0))
                            .h(px(2.0))
                            .rounded_full()
                            .bg(theme.colors.accent)
                    })),
            );
        }

        let search = (!compact).then(|| {
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .w(px(220.0))
                .h(px(32.0))
                .px(px(10.0))
                .rounded(px(theme.radii.md))
                .border_1()
                .border_color(theme.colors.border)
                .bg(theme.colors.surface_secondary)
                .text_color(theme.colors.muted)
                .text_size(px(11.0))
                .child(
                    gpui::svg()
                        .data(ICON_SEARCH)
                        .size(px(14.0))
                        .text_color(theme.colors.muted),
                )
                .child(
                    text_input("topbar-query")
                        .state(self.mods_query.downgrade())
                        .flex_1()
                        .min_w(px(0.0))
                        .px(px(0.0))
                        .py(px(0.0))
                        .placeholder_color(theme.colors.muted)
                        .text_color(theme.colors.foreground)
                        .text_size(px(11.0))
                        .whitespace_nowrap()
                        .min_h_auto()
                        .max_h_auto()
                        .placeholder(self.t("Search worlds and mods"))
                        .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, _, cx| {
                            if event.keystroke.key == "enter" {
                                this.go(Screen::Mods, cx);
                                this.search_mods(cx);
                            }
                        })),
                )
                .child(
                    div()
                        .font_family(theme.type_scale.mono)
                        .text_size(px(10.0))
                        .child("Ctrl K"),
                )
        });

        let settings = {
            let selected = self.screen == Screen::Settings;
            div()
                .id("topbar-settings")
                .flex()
                .items_center()
                .justify_center()
                .w(px(32.0))
                .h(px(32.0))
                .rounded(px(theme.radii.md))
                .bg(if selected {
                    theme.colors.surface_secondary
                } else {
                    rgba(0x00000000)
                })
                .text_color(if selected {
                    theme.colors.foreground
                } else {
                    theme.colors.muted
                })
                .cursor_pointer()
                .hover(move |style| style.bg(theme.colors.surface_secondary))
                .on_click(cx.listener(|this, _, _window, cx| this.go(Screen::Settings, cx)))
                .child(
                    gpui::svg()
                        .data(ICON_SETTINGS)
                        .size(px(14.0))
                        .text_color(if selected {
                            theme.colors.foreground
                        } else {
                            theme.colors.muted
                        }),
                )
        };

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(px(56.0))
                    .px(px(if compact { 16.0 } else { 32.0 }))
                    .child(
                        div()
                            .id("topbar-drag")
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .w(px(if compact { 150.0 } else { 230.0 }))
                            .h_full()
                            .window_control_area(WindowControlArea::Drag)
                            .on_mouse_down(MouseButton::Left, |_, window, _| {
                                window.start_window_move()
                            })
                            .children(theme.dark.then(|| Self::mark()))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child("Scoria"),
                            )
                            .child(
                                div()
                                    .font_family(theme.type_scale.mono)
                                    .text_size(px(10.0))
                                    .text_color(theme.colors.muted)
                                    .child(env!("SCORIA_VERSION")),
                            ),
                    )
                    .child(div().flex_1().h_full().flex().justify_center().child(nav))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .children(search)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(7.0))
                                    .h(px(32.0))
                                    .px(px(10.0))
                                    .rounded(px(theme.radii.md))
                                    .bg(theme.colors.surface_secondary)
                                    .text_size(px(theme.type_scale.caption))
                                    .text_color(theme.colors.foreground)
                                    .child(
                                        div().w(px(6.0)).h(px(6.0)).rounded_full().bg(status_color),
                                    )
                                    .child(status),
                            )
                            .child(settings)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(2.0))
                                    .child(control(
                                        "tb-min",
                                        ICON_WIN_MIN,
                                        theme.colors.surface_tertiary,
                                        |window| window.minimize_window(),
                                    ))
                                    .child(control(
                                        "tb-max",
                                        ICON_WIN_MAX,
                                        theme.colors.surface_tertiary,
                                        |window| window.zoom_window(),
                                    ))
                                    .child(control(
                                        "tb-close",
                                        ICON_WIN_CLOSE,
                                        rgba(0xE5484D44),
                                        |window| window.remove_window(),
                                    )),
                            ),
                    ),
            )
            .child(div().h(px(1.0)).w_full().bg(theme.colors.separator))
    }
}

//! Custom titlebar and the navigation rail that every screen shares.
use super::super::*;

impl Shell {
    fn mark() -> impl IntoElement {
        img(ImageSource::Image(Arc::new(Image::from_bytes(
            ImageFormat::Png,
            MARK.to_vec(),
        ))))
        .w(px(26.0))
        .h(px(26.0))
    }

    pub(super) fn titlebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let _ = cx;
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
        div()
            .flex()
            .items_center()
            .justify_between()
            .h(px(40.0))
            .px(px(theme.spacing.md))
            .child(
                div()
                    .id("titlebar-drag")
                    .flex()
                    .items_center()
                    .gap(px(theme.spacing.sm))
                    .flex_1()
                    .h_full()
                    .window_control_area(WindowControlArea::Drag)
                    .on_mouse_down(MouseButton::Left, |_, window, _| window.start_window_move())
                    .child(Self::mark())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(theme.spacing.xs))
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child("Scoria"),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.colors.muted)
                                    .child(env!("SCORIA_VERSION")),
                            ),
                    ),
            )
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
            )
    }

    pub(super) fn rail(&self, cx: &mut Context<Self>, compact: bool) -> impl IntoElement {
        let theme = self.theme;
        let mut rail = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .w(px(if compact { 64.0 } else { 196.0 }))
            .m(px(theme.spacing.md))
            .p(px(theme.spacing.md))
            .rounded(px(theme.radii.xl))
            .bg(if theme.dark {
                rgba(0x121014A8)
            } else {
                rgba(0xFFFBF6D9)
            })
            .backdrop_blur(px(24.0))
            .border_1()
            .border_color(if theme.dark {
                rgba(0xFFFFFF14)
            } else {
                rgba(0x1D151010)
            })
            .child(div().h(px(theme.spacing.xs)));

        for screen in Screen::ALL {
            let selected = screen == self.screen;
            let on_click =
                cx.listener(move |this, _: &gpui::ClickEvent, _window, cx| this.go(screen, cx));
            let tint = if selected {
                theme.colors.accent
            } else {
                theme.colors.muted
            };
            let mut item = div()
                .id(("rail", screen as usize))
                .flex()
                .items_center()
                .justify_start()
                .gap(px(theme.spacing.sm))
                .h(px(34.0))
                .px(px(theme.spacing.sm))
                .when(compact, |item| {
                    item.w(px(40.0))
                        .h(px(40.0))
                        .px(px(0.0))
                        .justify_center()
                        .self_center()
                })
                .rounded(px(theme.radii.md))
                .bg(if selected {
                    rgba(0xFF8A4C1F)
                } else {
                    rgba(0x00000000)
                })
                .text_color(if selected {
                    theme.colors.foreground
                } else {
                    theme.colors.muted
                })
                .text_size(px(theme.type_scale.body))
                .cursor_pointer()
                .transitions(|transitions| transitions.bg(std::time::Duration::from_millis(140)))
                .on_click(on_click)
                .child(
                    gpui::svg()
                        .data(screen.icon())
                        .size(px(15.0))
                        .text_color(tint),
                )
                .children((!compact).then(|| self.t(screen.label())));
            if !selected {
                let hover = if theme.dark {
                    rgba(0xFFFFFF0A)
                } else {
                    rgba(0x1D15100A)
                };
                item = item.hover(move |style| style.bg(hover));
            }
            rail = rail.child(item);
        }
        rail = rail.child(div().flex_1()).child(
            div()
                .id("rail-toggle")
                .flex()
                .items_center()
                .justify_center()
                .when(compact, |button| {
                    button.w(px(40.0)).h(px(40.0)).self_center()
                })
                .when(!compact, |button| {
                    button.w(px(34.0)).h(px(34.0)).self_start()
                })
                .rounded(px(theme.radii.md))
                .text_color(theme.colors.muted)
                .cursor_pointer()
                .hover(move |style| {
                    style.bg(if theme.dark {
                        rgba(0xFFFFFF0A)
                    } else {
                        rgba(0x1D15100A)
                    })
                })
                .on_click(cx.listener(|this, _, _window, cx| this.toggle_rail(cx)))
                .child(
                    gpui::svg()
                        .data(ICON_MENU)
                        .size(px(14.0))
                        .text_color(theme.colors.muted),
                ),
        );
        rail
    }
}

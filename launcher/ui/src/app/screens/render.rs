//! The window shell: backdrop, topbar + screen, resize grips, toast.
use super::super::*;

impl Render for Shell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        // Below ~1080px the rail drops its labels and the hero shrinks.
        let width = window.viewport_size().width;
        let compact = width < px(1080.0) || self.settings.rail_collapsed;
        let wide = width >= px(1280.0);
        let body = match self.screen {
            Screen::Home => div().child(self.home(cx, compact, wide)),
            Screen::Mods => div().child(self.mods(cx)),
            Screen::Servers => div().child(self.servers_screen(cx)),
            Screen::Settings => div().child(self.settings(cx)),
        }
        // A screen change fades and rises a few pixels instead of cutting. The id carries the
        // screen, so only a change restarts the animation; hover and polling do not.
        .with_animation(
            ("screen", self.screen as usize),
            Animation::new(std::time::Duration::from_millis(160)).with_easing(ease_out_quint()),
            |body, delta| body.opacity(delta).mt(px(6.0 * (1.0 - delta))),
        );

        let mut root = div()
            .relative()
            .size_full()
            .key_context("Scoria")
            .on_action(cx.listener(|this, _: &ShowHome, _, cx| this.go(Screen::Home, cx)))
            .on_action(cx.listener(|this, _: &ShowMods, _, cx| this.go(Screen::Mods, cx)))
            .on_action(cx.listener(|this, _: &ShowServers, _, cx| this.go(Screen::Servers, cx)))
            .on_action(cx.listener(|this, _: &ShowSettings, _, cx| this.go(Screen::Settings, cx)))
            .on_action(cx.listener(|this, _: &RunGame, _, cx| this.game_action(cx)))
            .on_action(cx.listener(|this, _: &CloseOverlay, _, cx| this.close_overlay(cx)))
            .font_family(theme.type_scale.sans)
            .bg(rgb(0x09080A))
            .text_color(theme.colors.foreground)
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
                .h_full(),
            )
            .child(
                div()
                    .absolute()
                    .top(px(0.0))
                    .left(px(0.0))
                    .w_full()
                    .h_full()
                    .bg(if theme.dark {
                        rgba(0x0B0B0DB8)
                    } else {
                        rgba(0xF1F1F1D9)
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .overflow_hidden()
                    .child(self.topbar(cx, compact))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_h(px(0.0))
                            .id("screen")
                            .p(px(if compact {
                                theme.spacing.xl
                            } else {
                                theme.spacing.page
                            }))
                            .pt(px(if compact {
                                theme.spacing.lg
                            } else {
                                theme.spacing.xl
                            }))
                            .pb(px(theme.spacing.xl))
                            .gap(px(theme.spacing.lg))
                            .overflow_y_scroll()
                            .child(body),
                    ),
            );
        let edge = |id: &'static str, edge: ResizeEdge| {
            div()
                .id(id)
                .absolute()
                .on_mouse_down(MouseButton::Left, move |_, window, _| {
                    window.start_window_resize(edge)
                })
        };
        root = root
            .child(
                edge("rz-n", ResizeEdge::Top)
                    .top(px(0.))
                    .left(px(8.))
                    .right(px(8.))
                    .h(px(6.)),
            )
            .child(
                edge("rz-s", ResizeEdge::Bottom)
                    .bottom(px(0.))
                    .left(px(8.))
                    .right(px(8.))
                    .h(px(6.)),
            )
            .child(
                edge("rz-w", ResizeEdge::Left)
                    .left(px(0.))
                    .top(px(8.))
                    .bottom(px(8.))
                    .w(px(6.)),
            )
            .child(
                edge("rz-e", ResizeEdge::Right)
                    .right(px(0.))
                    .top(px(8.))
                    .bottom(px(8.))
                    .w(px(6.)),
            )
            .child(
                edge("rz-nw", ResizeEdge::TopLeft)
                    .top(px(0.))
                    .left(px(0.))
                    .w(px(10.))
                    .h(px(10.)),
            )
            .child(
                edge("rz-ne", ResizeEdge::TopRight)
                    .top(px(0.))
                    .right(px(0.))
                    .w(px(10.))
                    .h(px(10.)),
            )
            .child(
                edge("rz-sw", ResizeEdge::BottomLeft)
                    .bottom(px(0.))
                    .left(px(0.))
                    .w(px(10.))
                    .h(px(10.)),
            )
            .child(
                edge("rz-se", ResizeEdge::BottomRight)
                    .bottom(px(0.))
                    .right(px(0.))
                    .w(px(10.))
                    .h(px(10.)),
            );

        if self.fade > 0.0 {
            root = root.child(
                div()
                    .absolute()
                    .top(px(0.0))
                    .left(px(0.0))
                    .w_full()
                    .h_full()
                    .bg(theme.colors.background)
                    .opacity(self.fade),
            );
        }

        if !self.toasts.is_empty() {
            root = root.child(
                div()
                    .absolute()
                    .bottom(px(theme.spacing.xl))
                    .right(px(theme.spacing.xl))
                    .flex()
                    .flex_col()
                    .items_end()
                    .gap(px(theme.spacing.sm))
                    .children(
                        self.toasts
                            .iter()
                            .map(|item| toast(&theme, item.message.clone(), item.tone)),
                    ),
            );
        }
        root
    }
}

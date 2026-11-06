//! A segmented control option. The caller lays out the track and owns the selection.
use gpui::{div, prelude::*, px, IntoElement, SharedString};

use crate::Theme;

pub fn segment(
    theme: Theme,
    id: &'static str,
    label: impl Into<SharedString>,
    selected: bool,
    on_click: impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    let label = label.into();
    div()
        .id(id)
        .flex()
        .items_center()
        .h(px(28.0))
        .px(px(theme.spacing.md))
        .rounded(px(theme.radii.md))
        .bg(if selected {
            if theme.dark {
                gpui::rgba(0xFFFFFF12)
            } else {
                theme.colors.surface
            }
        } else {
            gpui::rgba(0x00000000)
        })
        .text_color(if selected {
            theme.colors.foreground
        } else {
            theme.colors.muted
        })
        .text_size(px(theme.type_scale.body))
        .cursor_pointer()
        .on_click(on_click)
        .child(label)
}

//! A quiet toast: a surface card with a coloured edge, no shadow stacking, one message.
use gpui::{div, prelude::*, px, Div, SharedString};

use crate::{Theme, Tone};

pub fn toast(theme: &Theme, message: impl Into<SharedString>, tone: Tone) -> Div {
    let edge = match tone {
        Tone::Neutral => theme.colors.border,
        Tone::Accent => theme.colors.accent,
        Tone::Success => theme.colors.success,
        Tone::Warning => theme.colors.warning,
        Tone::Danger => theme.colors.danger,
    };
    div()
        .flex()
        .items_center()
        .gap(px(theme.spacing.sm))
        .max_w(px(420.0))
        .px(px(theme.spacing.lg))
        .py(px(theme.spacing.md))
        .rounded(px(theme.radii.xl))
        .border_2()
        .border_color(theme.colors.border)
        .bg(theme.colors.surface)
        .text_color(theme.colors.foreground)
        .text_size(px(theme.type_scale.body))
        .child(div().w(px(3.0)).h(px(18.0)).rounded(px(2.0)).bg(edge))
        .child(message.into())
}

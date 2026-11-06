//! Loading placeholders: the shape of the result before the result exists.
use gpui::{div, prelude::*, px, Div};

use crate::Theme;

/// A rounded bar of `width`; use `height` 10–16 for text and 28+ for a field.
pub fn skeleton(theme: &Theme, width: f32, height: f32) -> Div {
    div()
        .h(px(height))
        .w(px(width))
        .rounded_full()
        .bg(theme.colors.surface_tertiary)
}

/// A full-width skeleton line.
pub fn skeleton_line(theme: &Theme, height: f32) -> Div {
    div()
        .h(px(height))
        .w_full()
        .rounded_full()
        .bg(theme.colors.surface_secondary)
}

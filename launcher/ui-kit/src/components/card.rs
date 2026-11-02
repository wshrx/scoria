//! A surface card: hierarchy comes from surface levels and spacing, borders only separate.
use gpui::{div, prelude::*, px, Div, IntoElement};

use crate::Theme;

pub fn card(theme: &Theme, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(theme.spacing.md))
        .p(px(theme.spacing.lg))
        .rounded(px(theme.radii.xl))
        .border_1()
        .border_color(theme.colors.border)
        .bg(theme.colors.surface)
        .child(child)
}

/// A quieter card for nested regions.
pub fn soft_card(theme: &Theme, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(theme.spacing.sm))
        .p(px(theme.spacing.md))
        .rounded(px(theme.radii.lg))
        .bg(theme.colors.surface_secondary)
        .child(child)
}

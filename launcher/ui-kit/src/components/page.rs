//! Page-level headings and states. Screens differ in data, not in how a title, an empty list or
//! a failed request looks.
use gpui::{div, prelude::*, px, Div, SharedString};

use crate::{soft_card, Theme};

/// A screen's title and one supporting line.
pub fn page_header(
    theme: &Theme,
    title: impl Into<SharedString>,
    subtitle: impl Into<SharedString>,
) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(theme.spacing.xs))
        .child(
            div()
                .text_size(px(theme.type_scale.title))
                .child(title.into()),
        )
        .child(
            div()
                .text_size(px(theme.type_scale.body))
                .text_color(theme.colors.muted)
                .child(subtitle.into()),
        )
}

/// An empty list: icon, a short title and one sentence about what to do.
pub fn empty_state(
    theme: &Theme,
    icon: &'static [u8],
    title: impl Into<SharedString>,
    body: impl Into<SharedString>,
) -> Div {
    soft_card(
        theme,
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(theme.spacing.sm))
            .py(px(theme.spacing.xl))
            .child(
                gpui::svg()
                    .data(icon)
                    .size(px(28.0))
                    .text_color(theme.colors.muted),
            )
            .child(
                div()
                    .text_size(px(theme.type_scale.subtitle))
                    .child(title.into()),
            )
            .child(
                div()
                    .max_w(px(420.0))
                    .text_size(px(theme.type_scale.body))
                    .text_color(theme.colors.muted)
                    .child(body.into()),
            ),
    )
}

/// A failed request or a failed launch. The caller appends the retry action.
pub fn error_panel(
    theme: &Theme,
    title: impl Into<SharedString>,
    message: impl Into<SharedString>,
) -> Div {
    soft_card(
        theme,
        div()
            .flex()
            .flex_col()
            .gap(px(theme.spacing.sm))
            .child(
                div()
                    .text_size(px(theme.type_scale.subtitle))
                    .text_color(theme.colors.danger)
                    .child(title.into()),
            )
            .child(
                div()
                    .text_size(px(theme.type_scale.body))
                    .text_color(theme.colors.muted)
                    .child(message.into()),
            ),
    )
}

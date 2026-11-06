//! The launcher's text field styling. The caller owns the entity and the layout; this owns the
//! frame, colours, type size and height so every screen matches.
use gpui::{prelude::*, px, SharedString, WeakEntity};
use gpui_ce_elements::editable_text::{text_input, EditableTextElement, EditableTextState};

use crate::Theme;

pub fn field(
    theme: &Theme,
    id: &'static str,
    state: WeakEntity<EditableTextState>,
    placeholder: impl Into<SharedString>,
) -> EditableTextElement {
    text_input(id)
        .state(state)
        .h(px(40.0))
        .px(px(theme.spacing.md))
        .py(px(theme.spacing.sm))
        .placeholder_color(theme.colors.muted)
        .rounded(px(theme.radii.field))
        .border_1()
        .border_color(theme.colors.border)
        .bg(theme.colors.surface_secondary)
        .text_color(theme.colors.foreground)
        .text_size(px(theme.type_scale.body))
        .whitespace_nowrap()
        .min_h_auto()
        .max_h_auto()
        .placeholder(placeholder)
}

//! Small status pills: support labels, counts and states. Colour never carries meaning alone;
//! the text always says what it is.
use gpui::{div, prelude::*, px, Div, SharedString};

use crate::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    Neutral,
    Accent,
    Success,
    Warning,
    Danger,
}

pub fn badge(theme: &Theme, label: impl Into<SharedString>, tone: Tone) -> Div {
    let colors = theme.colors;
    let (background, foreground) = match tone {
        Tone::Neutral => (colors.surface_tertiary, colors.muted),
        Tone::Accent => (colors.accent_soft, colors.accent),
        Tone::Success => (colors.surface_tertiary, colors.success),
        Tone::Warning => (colors.surface_tertiary, colors.warning),
        Tone::Danger => (colors.surface_tertiary, colors.danger),
    };
    div()
        .flex()
        .items_center()
        .h(px(22.0))
        .px(px(theme.spacing.sm))
        .rounded_full()
        .bg(background)
        .text_color(foreground)
        .text_size(px(theme.type_scale.caption))
        .child(label.into())
}

/// Turns a support label from the engine (`Works`, `Partly`, `Doesn't run`) into a tone.
pub fn tone_for_support(label: &str) -> Tone {
    match label {
        "Works" => Tone::Success,
        "Partly" => Tone::Warning,
        "Doesn't run" => Tone::Danger,
        _ => Tone::Neutral,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_labels_map_to_their_tone() {
        assert_eq!(tone_for_support("Works"), Tone::Success);
        assert_eq!(tone_for_support("Partly"), Tone::Warning);
        assert_eq!(tone_for_support("Doesn't run"), Tone::Danger);
        assert_eq!(tone_for_support(""), Tone::Neutral);
        assert_eq!(tone_for_support("Unknown"), Tone::Neutral);
    }
}

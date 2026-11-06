//! Buttons follow HeroUI's variants: one primary action per context, quieter alternatives,
//! danger for destructive work.
use std::time::Duration;

use gpui::{div, prelude::*, px, relative, svg, ElementId, IntoElement, SharedString};

use crate::{Theme, Variant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
    /// The one hero action (52 px).
    Xl,
}

/// A button's visual weight. The caller owns the click handler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Tertiary,
    Outline,
    Ghost,
    Danger,
    /// Dark surface with the accent border: preparing and stop states.
    AccentOutline,
    /// Dark surface with the danger border: a failed action and retry.
    DangerOutline,
    /// Dark surface with accent text and border: a running game's Stop.
    AccentText,
}

pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    icon: Option<&'static [u8]>,
    icon_after: Option<&'static [u8]>,
    progress: Option<f32>,
    width: Option<f32>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            disabled: false,
            icon: None,
            icon_after: None,
            progress: None,
            width: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// A 12 px single-colour SVG before the label. Keep the same asset the old button used:
    /// the icon is a mask, so its colour follows the button's text.
    pub fn icon(mut self, icon: &'static [u8]) -> Self {
        self.icon = Some(icon);
        self
    }

    /// A 12 px icon after the label, for a chevron or an external-link arrow.
    pub fn icon_after(mut self, icon: &'static [u8]) -> Self {
        self.icon_after = Some(icon);
        self
    }

    /// A 3 px progress line inside the bottom edge, for the preparing state.
    pub fn progress(mut self, fraction: f32) -> Self {
        self.progress = Some(fraction);
        self
    }

    /// A fixed width, for the 192 px hero action.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Renders the button. `on_click` is optional so a button can be shown in a skeleton.
    pub fn render(
        self,
        theme: &Theme,
        on_click: Option<impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static>,
    ) -> impl IntoElement {
        let colors = theme.colors;
        let (background, foreground, border) = match self.variant {
            ButtonVariant::Primary => (colors.accent, colors.accent_foreground, colors.accent),
            ButtonVariant::Secondary => {
                (colors.surface_secondary, colors.foreground, colors.border)
            }
            ButtonVariant::Tertiary => (colors.surface, colors.foreground, colors.border),
            ButtonVariant::Outline => (colors.surface, colors.foreground, colors.border),
            ButtonVariant::Ghost => (gpui::rgba(0x00000000), colors.muted, gpui::rgba(0x00000000)),
            ButtonVariant::Danger => (colors.danger, colors.danger_foreground, colors.danger),
            ButtonVariant::AccentOutline => {
                (colors.surface_secondary, colors.foreground, colors.accent)
            }
            ButtonVariant::DangerOutline => {
                (colors.surface_secondary, colors.danger, colors.danger)
            }
            ButtonVariant::AccentText => (colors.surface_secondary, colors.accent, colors.accent),
        };
        let hover = match self.variant {
            ButtonVariant::Primary => colors.accent_hover,
            ButtonVariant::Danger => colors.danger,
            ButtonVariant::Ghost => colors.surface_secondary,
            ButtonVariant::AccentOutline => colors.accent_soft,
            ButtonVariant::DangerOutline => colors.surface_tertiary,
            ButtonVariant::AccentText => colors.accent_soft,
            _ => colors.surface_tertiary,
        };
        let pressed = match self.variant {
            ButtonVariant::Primary => colors.accent,
            ButtonVariant::Danger => colors.danger,
            ButtonVariant::AccentOutline => colors.accent_soft,
            ButtonVariant::DangerOutline => colors.surface_secondary,
            ButtonVariant::AccentText => colors.accent_soft,
            _ => colors.surface_secondary,
        };
        let (height, padding, text) = match self.size {
            ButtonSize::Sm => (28.0, 10.0, theme.type_scale.caption),
            ButtonSize::Md => (36.0, 14.0, theme.type_scale.body),
            ButtonSize::Lg => (40.0, 16.0, theme.type_scale.body),
            ButtonSize::Xl => (52.0, 20.0, theme.type_scale.subtitle),
        };

        let mut button = div()
            .id(self.id)
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .gap(px(12.0))
            .h(px(height))
            .px(px(padding))
            .when_some(self.width, |button, width| button.w(px(width)))
            .rounded(px(theme.radii.lg))
            .border_1()
            .border_color(border)
            .bg(background)
            .text_color(foreground)
            .text_size(px(text));
        if let Some(icon) = self.icon {
            button = button.child(svg().data(icon).size(px(12.0)).text_color(foreground));
        }
        button = button.child(self.label);
        if let Some(icon) = self.icon_after {
            button = button.child(svg().data(icon).size(px(12.0)).text_color(foreground));
        }
        if let Some(fraction) = self.progress {
            let fraction = fraction.clamp(0.0, 1.0);
            button = button.child(
                div()
                    .absolute()
                    .left(px(8.0))
                    .right(px(8.0))
                    .bottom(px(4.0))
                    .h(px(3.0))
                    .rounded_full()
                    .bg(colors.accent_soft)
                    .child(
                        div()
                            .h_full()
                            .w(relative(fraction))
                            .rounded_full()
                            .bg(colors.accent),
                    ),
            );
        }

        if self.disabled {
            button = button.opacity(0.55).cursor_default();
        } else {
            button = button
                .cursor_pointer()
                .hover(move |style| style.bg(hover))
                .active(move |style| style.bg(pressed))
                .transitions(|transitions| {
                    transitions
                        .bg(Duration::from_millis(120))
                        .border_color(Duration::from_millis(120))
                });
            if let Some(on_click) = on_click {
                button = button.on_click(on_click);
            }
        }
        button
    }
}

/// Convenience: a primary button with a handler.
pub fn primary(
    theme: &Theme,
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    on_click: impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    Button::new(id, label).render(theme, Some(on_click))
}

/// Keeps the `Variant` import meaningful for callers that switch on intent.
pub fn variant_of(variant: Variant) -> ButtonVariant {
    match variant {
        Variant::Primary => ButtonVariant::Primary,
        Variant::Secondary => ButtonVariant::Secondary,
        Variant::Ghost => ButtonVariant::Ghost,
        Variant::Danger => ButtonVariant::Danger,
    }
}

//! Buttons follow HeroUI's variants: one primary action per context, quieter alternatives,
//! danger for destructive work.
use gpui::{div, prelude::*, px, ElementId, IntoElement, SharedString};

use crate::{Theme, Variant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
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
}

pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            disabled: false,
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
            ButtonVariant::Ghost => (colors.surface, colors.muted, colors.surface),
            ButtonVariant::Danger => (colors.danger, colors.danger_foreground, colors.danger),
        };
        let hover = match self.variant {
            ButtonVariant::Primary => colors.accent_hover,
            ButtonVariant::Danger => colors.danger,
            ButtonVariant::Ghost => colors.surface_secondary,
            _ => colors.surface_tertiary,
        };
        let (height, padding, text) = match self.size {
            ButtonSize::Sm => (28.0, 10.0, theme.type_scale.caption),
            ButtonSize::Md => (36.0, 14.0, theme.type_scale.body),
            ButtonSize::Lg => (40.0, 16.0, theme.type_scale.body),
        };

        let mut button = div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .h(px(height))
            .px(px(padding))
            .rounded(px(theme.radii.lg))
            .border_1()
            .border_color(border)
            .bg(background)
            .text_color(foreground)
            .text_size(px(text))
            .child(self.label);

        if self.disabled {
            button = button.opacity(0.5).cursor_default();
        } else {
            button = button.cursor_pointer().hover(move |style| style.bg(hover));
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

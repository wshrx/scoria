//! One SVG mask, one fixed size. The window never stretches an icon to fit a label.
use gpui::{prelude::*, px, Rgba, Svg};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconSize {
    Sm,
    Md,
    Lg,
}

impl IconSize {
    pub fn px(self) -> f32 {
        match self {
            Self::Sm => 12.0,
            Self::Md => 14.0,
            Self::Lg => 16.0,
        }
    }
}

/// A single-colour SVG mask: `data` is the asset, `color` the mask tint.
pub fn icon(data: &'static [u8], size: IconSize, color: Rgba) -> Svg {
    gpui::svg().data(data).size(px(size.px())).text_color(color)
}

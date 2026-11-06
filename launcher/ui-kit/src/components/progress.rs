//! A determinate progress track. The caller owns the numbers; this only draws them, so a
//! download card can put the percentage where it reads best.
use gpui::{div, prelude::*, px, relative, Div};

use crate::Theme;

/// A finite fraction in 0..=1. A NaN from a missing `total` becomes an empty track instead of
/// an invalid width.
pub fn clamp_fraction(fraction: f32) -> f32 {
    if fraction.is_finite() {
        fraction.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// `fraction` of the track filled with the accent. Values outside 0..=1 are clamped, so a
/// rounding error from the engine cannot draw a bar past its track.
pub fn progress_bar(theme: &Theme, fraction: f32) -> Div {
    let fraction = clamp_fraction(fraction);
    div()
        .h(px(8.0))
        .w_full()
        .rounded_full()
        .bg(theme.colors.surface_tertiary)
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .w(relative(fraction))
                .rounded_full()
                .bg(theme.colors.accent),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fraction_is_clamped_to_the_track() {
        assert_eq!(clamp_fraction(0.42), 0.42);
        assert_eq!(clamp_fraction(-1.0), 0.0);
        assert_eq!(clamp_fraction(2.0), 1.0);
        assert_eq!(clamp_fraction(f32::NAN), 0.0);
    }
}

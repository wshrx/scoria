//! Scoria's design tokens: basalt surfaces, one ember accent, an 8px radius scale and a 4px
//! spacing rhythm. The structure follows HeroUI's semantic tokens; only the palette is Scoria's.
use gpui::{rgb, Rgba};

/// Semantic colours. Components take them from the active [`Theme`](crate::Theme) and never
/// hard-code a value.
#[derive(Clone, Copy, Debug)]
pub struct Colors {
    /// Page background.
    pub background: Rgba,
    /// Cards and non-floating containers.
    pub surface: Rgba,
    /// Nested or lower-emphasis regions.
    pub surface_secondary: Rgba,
    /// Stronger surface separation.
    pub surface_tertiary: Rgba,
    /// Popovers, menus and dialogs.
    pub overlay: Rgba,
    /// Dimming behind an overlay.
    pub backdrop: Rgba,
    pub foreground: Rgba,
    pub muted: Rgba,
    pub border: Rgba,
    pub separator: Rgba,
    /// Primary actions, focus and emphasis. Scoria's ember.
    pub accent: Rgba,
    pub accent_foreground: Rgba,
    pub accent_hover: Rgba,
    pub accent_soft: Rgba,
    pub success: Rgba,
    pub warning: Rgba,
    pub danger: Rgba,
    pub danger_foreground: Rgba,
    pub focus_ring: Rgba,
}

/// Corner radii, in pixels. Cards use `lg`, fields use `field`.
#[derive(Clone, Copy, Debug)]
pub struct Radii {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub field: f32,
}

/// The 4px spacing rhythm.
#[derive(Clone, Copy, Debug)]
pub struct Spacing {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

/// Type sizes and weights.
#[derive(Clone, Copy, Debug)]
pub struct Typography {
    /// The family registered from `launcher/ui/assets/fonts`.
    pub sans: &'static str,
    /// Monospace, for versions, paths and numbers.
    pub mono: &'static str,
    pub caption: f32,
    pub body: f32,
    pub subtitle: f32,
    pub title: f32,
    pub display: f32,
}

/// Motion timings. Long enough to read as a transition, short enough not to wait.
#[derive(Clone, Copy, Debug)]
pub struct Motion {
    pub fast: std::time::Duration,
    pub base: std::time::Duration,
}

impl Motion {
    pub const fn new() -> Self {
        Self {
            fast: std::time::Duration::from_millis(120),
            base: std::time::Duration::from_millis(180),
        }
    }
}

impl Default for Motion {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub dark: bool,
    /// Flat surfaces: no photographic backdrop, no glass, no blur.
    pub flat: bool,
    pub colors: Colors,
    pub radii: Radii,
    pub spacing: Spacing,
    pub type_scale: Typography,
    pub motion: Motion,
}

impl Theme {
    /// Dark is Scoria's default: a basalt window with warm paper text.
    pub fn dark() -> Self {
        Self {
            dark: true,
            flat: false,
            colors: Colors {
                background: rgb(0x0B0A0C),
                surface: rgb(0x141114),
                surface_secondary: rgb(0x1A161A),
                surface_tertiary: rgb(0x211C20),
                overlay: rgb(0x1A161A),
                backdrop: rgb(0x000000),
                foreground: rgb(0xF7EFE7),
                muted: rgb(0xB3A293),
                border: rgb(0x2A2328),
                separator: rgb(0x221C20),
                accent: rgb(0xFF8A4C),
                accent_foreground: rgb(0x1B0D06),
                accent_hover: rgb(0xFFA46E),
                accent_soft: rgb(0x3A2116),
                success: rgb(0x6FBF8B),
                warning: rgb(0xD8B26A),
                danger: rgb(0xE5484D),
                danger_foreground: rgb(0x1B0D06),
                focus_ring: rgb(0xFF8A4C),
            },
            radii: Radii {
                sm: 4.0,
                md: 6.0,
                lg: 8.0,
                xl: 12.0,
                field: 12.0,
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 16.0,
                xl: 24.0,
            },
            type_scale: Typography {
                sans: "Medeo Sans",
                mono: "Medeo Mono",
                caption: 12.0,
                body: 14.0,
                subtitle: 16.0,
                title: 20.0,
                display: 32.0,
            },
            motion: Motion::new(),
        }
    }

    /// Light keeps the same surfaces and accent, inverted around a warm paper canvas.
    pub fn light() -> Self {
        Self {
            dark: false,
            flat: false,
            colors: Colors {
                background: rgb(0xFBF5EE),
                surface: rgb(0xFFFFFF),
                surface_secondary: rgb(0xF4EDE5),
                surface_tertiary: rgb(0xEDE4DA),
                overlay: rgb(0xFFFFFF),
                backdrop: rgb(0x000000),
                foreground: rgb(0x1D1510),
                muted: rgb(0x6B5C50),
                border: rgb(0xE2D6C9),
                separator: rgb(0xECE2D7),
                accent: rgb(0xB84E1B),
                accent_foreground: rgb(0xFFFFFF),
                accent_hover: rgb(0xA84513),
                accent_soft: rgb(0xFBE3D4),
                success: rgb(0x2F8F57),
                warning: rgb(0xA9741C),
                danger: rgb(0xC63B40),
                danger_foreground: rgb(0xFFFFFF),
                focus_ring: rgb(0xB84E1B),
            },
            radii: Radii {
                sm: 4.0,
                md: 6.0,
                lg: 8.0,
                xl: 12.0,
                field: 12.0,
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 16.0,
                xl: 24.0,
            },
            type_scale: Typography {
                sans: "Medeo Sans",
                mono: "Medeo Mono",
                caption: 12.0,
                body: 14.0,
                subtitle: 16.0,
                title: 20.0,
                display: 32.0,
            },
            motion: Motion::new(),
        }
    }

    /// The redesign experiment: a flat drafting-table palette, sharp corners and mono metadata.
    /// No backdrop image, no glass, no shadows.
    pub fn blueprint(dark: bool) -> Self {
        let colors = if dark {
            Colors {
                background: rgb(0x081218),
                surface: rgb(0x0F1D24),
                surface_secondary: rgb(0x132630),
                surface_tertiary: rgb(0x19313C),
                overlay: rgb(0x132630),
                backdrop: rgb(0x000000),
                foreground: rgb(0xE8F1F5),
                muted: rgb(0x93AAB7),
                border: rgb(0x29434F),
                separator: rgb(0x1D333D),
                accent: rgb(0xFF5C2B),
                accent_foreground: rgb(0x1A0A04),
                accent_hover: rgb(0xFF7448),
                accent_soft: rgb(0x3A1B12),
                success: rgb(0x48B98C),
                warning: rgb(0xE0A03A),
                danger: rgb(0xE05252),
                danger_foreground: rgb(0x1A0A04),
                focus_ring: rgb(0xFF5C2B),
            }
        } else {
            Colors {
                background: rgb(0xE7EBEE),
                surface: rgb(0xF8FAFB),
                surface_secondary: rgb(0xEFF3F5),
                surface_tertiary: rgb(0xE3E9EC),
                overlay: rgb(0xFFFFFF),
                backdrop: rgb(0x000000),
                foreground: rgb(0x0C1B24),
                muted: rgb(0x4F6472),
                border: rgb(0xAEBEC9),
                separator: rgb(0xCBD7DE),
                accent: rgb(0xD23A10),
                accent_foreground: rgb(0xFFFFFF),
                accent_hover: rgb(0xB8310C),
                accent_soft: rgb(0xF7D9CE),
                success: rgb(0x0B6B47),
                warning: rgb(0x8A5A00),
                danger: rgb(0xB3261E),
                danger_foreground: rgb(0xFFFFFF),
                focus_ring: rgb(0xD23A10),
            }
        };
        Self {
            dark,
            flat: true,
            colors,
            radii: Radii {
                sm: 2.0,
                md: 3.0,
                lg: 4.0,
                xl: 6.0,
                field: 4.0,
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 16.0,
                xl: 24.0,
            },
            type_scale: Typography {
                sans: "Medeo Sans",
                mono: "Medeo Mono",
                caption: 11.0,
                body: 14.0,
                subtitle: 15.0,
                title: 22.0,
                display: 40.0,
            },
            motion: Motion::new(),
        }
    }

    /// The arcade/Brawl-inspired variant: saturated yellow and navy, chunky radii and a red
    /// primary. Original palette and shapes; no Supercell assets.
    pub fn arcade(dark: bool) -> Self {
        let colors = if dark {
            Colors {
                background: rgb(0x14213D),
                surface: rgb(0x1D3557),
                surface_secondary: rgb(0x223A5E),
                surface_tertiary: rgb(0x2B4A73),
                overlay: rgb(0x1D3557),
                backdrop: rgb(0x000000),
                foreground: rgb(0xFFF6D8),
                muted: rgb(0xA9B8D0),
                border: rgb(0x0B1220),
                separator: rgb(0x2B4A73),
                accent: rgb(0xFFD23F),
                accent_foreground: rgb(0x14213D),
                accent_hover: rgb(0xFFE066),
                accent_soft: rgb(0x4A3B1A),
                success: rgb(0x57CC99),
                warning: rgb(0xFFB703),
                danger: rgb(0xFF5D8F),
                danger_foreground: rgb(0x14213D),
                focus_ring: rgb(0xFFD23F),
            }
        } else {
            Colors {
                background: rgb(0xFFD23F),
                surface: rgb(0xFFF6D8),
                surface_secondary: rgb(0xFFE9A8),
                surface_tertiary: rgb(0xFFDF7A),
                overlay: rgb(0xFFF6D8),
                backdrop: rgb(0x000000),
                foreground: rgb(0x14213D),
                muted: rgb(0x4A5578),
                border: rgb(0x14213D),
                separator: rgb(0xD9A400),
                accent: rgb(0xD62828),
                accent_foreground: rgb(0xFFFFFF),
                accent_hover: rgb(0xB51F1F),
                accent_soft: rgb(0xFFD6D9),
                success: rgb(0x1F7A6B),
                warning: rgb(0x8A5A00),
                danger: rgb(0xD62828),
                danger_foreground: rgb(0xFFFFFF),
                focus_ring: rgb(0x1D3557),
            }
        };
        Self {
            dark,
            flat: true,
            colors,
            radii: Radii {
                sm: 10.0,
                md: 14.0,
                lg: 18.0,
                xl: 24.0,
                field: 14.0,
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 16.0,
                xl: 24.0,
            },
            type_scale: Typography {
                sans: "Medeo Sans",
                mono: "Medeo Mono",
                caption: 12.0,
                body: 14.0,
                subtitle: 16.0,
                title: 22.0,
                display: 46.0,
            },
            motion: Motion::new(),
        }
    }

    pub fn arcade_dark() -> Self {
        Self::arcade(true)
    }

    pub fn arcade_light() -> Self {
        Self::arcade(false)
    }

    pub fn blueprint_dark() -> Self {
        Self::blueprint(true)
    }

    pub fn blueprint_light() -> Self {
        Self::blueprint(false)
    }

    pub fn px(value: f32) -> gpui::Pixels {
        gpui::px(value)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// WCAG relative luminance for an sRGB colour.
    fn luminance(color: Rgba) -> f32 {
        let channel = |value: f32| {
            if value <= 0.039_28 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * channel(color.color.red)
            + 0.7152 * channel(color.color.green)
            + 0.0722 * channel(color.color.blue)
    }

    fn contrast(a: Rgba, b: Rgba) -> f32 {
        let (lighter, darker) = {
            let (a, b) = (luminance(a), luminance(b));
            if a > b {
                (a, b)
            } else {
                (b, a)
            }
        };
        (lighter + 0.05) / (darker + 0.05)
    }

    fn assert_readable(theme: &Theme, label: &str, foreground: Rgba, background: Rgba) {
        let ratio = contrast(foreground, background);
        assert!(
            ratio >= 4.5,
            "{label}: contrast {ratio:.2} is below 4.5:1 in the {} theme",
            if theme.dark { "dark" } else { "light" }
        );
    }

    #[test]
    fn text_is_readable_in_both_themes() {
        for theme in [
            Theme::dark(),
            Theme::light(),
            Theme::blueprint_dark(),
            Theme::blueprint_light(),
        ] {
            let colors = theme.colors;
            assert_readable(
                &theme,
                "foreground on background",
                colors.foreground,
                colors.background,
            );
            assert_readable(
                &theme,
                "foreground on surface",
                colors.foreground,
                colors.surface,
            );
            assert_readable(
                &theme,
                "muted on background",
                colors.muted,
                colors.background,
            );
            assert_readable(&theme, "muted on surface", colors.muted, colors.surface);
            assert_readable(
                &theme,
                "accent label on accent",
                colors.accent_foreground,
                colors.accent,
            );
            assert_readable(
                &theme,
                "danger label on danger",
                colors.danger_foreground,
                colors.danger,
            );
        }
    }

    #[test]
    fn the_scale_stays_on_the_4px_rhythm_and_the_radius_scale() {
        for theme in [
            Theme::dark(),
            Theme::light(),
            Theme::blueprint_dark(),
            Theme::blueprint_light(),
        ] {
            for value in [
                theme.spacing.xs,
                theme.spacing.sm,
                theme.spacing.md,
                theme.spacing.lg,
                theme.spacing.xl,
            ] {
                assert_eq!(value % 4.0, 0.0, "spacing {value} leaves the 4px rhythm");
            }
            assert!(theme.radii.sm < theme.radii.md);
            assert!(theme.radii.md < theme.radii.lg);
            assert!(theme.radii.lg < theme.radii.xl);
        }
    }
}

//! Scoria's design tokens: basalt surfaces, one ember accent, an 8px radius scale and a 4px
//! spacing rhythm. The structure follows HeroUI's semantic tokens; only the palette is Scoria's.
use gpui::{rgb, Rgba};
use std::fmt::Write as _;

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
    pub xxl: f32,
    /// Page gutter on a wide window.
    pub page: f32,
}

/// Fixed component sizes. One place for control heights, rows and page widths.
#[derive(Clone, Copy, Debug)]
pub struct Sizes {
    pub control: f32,
    pub control_lg: f32,
    pub field: f32,
    pub row: f32,
    pub topbar: f32,
    pub select: f32,
    pub page_max: f32,
    pub button_sm: f32,
    pub button_md: f32,
    pub button_lg: f32,
    pub button_xl: f32,
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
    pub body_lg: f32,
    pub subtitle: f32,
    pub title: f32,
    /// Page titles on the redesigned screens.
    pub heading: f32,
    /// The Home hero.
    pub hero: f32,
    pub display: f32,
}

/// Motion timings. Long enough to read as a transition, short enough not to wait.
#[derive(Clone, Copy, Debug)]
pub struct Motion {
    /// Hover and press.
    pub fast: std::time::Duration,
    /// Colour and small state changes.
    pub base: std::time::Duration,
    /// Hover plates; the upstream `QUICK` step.
    pub quick: std::time::Duration,
    /// Selection and page changes; the upstream `MOVE` step.
    pub r#move: std::time::Duration,
    /// Content arriving; the upstream `ENTER` step.
    pub enter: std::time::Duration,
    /// Delay between list rows arriving.
    pub stagger: std::time::Duration,
    /// How far content rises while entering, in pixels.
    pub rise: f32,
}

impl Motion {
    pub const fn new() -> Self {
        Self {
            fast: std::time::Duration::from_millis(120),
            base: std::time::Duration::from_millis(180),
            quick: std::time::Duration::from_millis(150),
            r#move: std::time::Duration::from_millis(240),
            enter: std::time::Duration::from_millis(600),
            stagger: std::time::Duration::from_millis(40),
            rise: 8.0,
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
    pub colors: Colors,
    pub radii: Radii,
    pub spacing: Spacing,
    pub sizes: Sizes,
    pub type_scale: Typography,
    pub motion: Motion,
}

impl Theme {
    /// Dark is Scoria's default: a basalt window with warm paper text.
    pub fn dark() -> Self {
        Self {
            dark: true,
            colors: Colors {
                background: rgb(0x0B0B0D),
                surface: rgb(0x141416),
                surface_secondary: rgb(0x1A1A1D),
                surface_tertiary: rgb(0x202024),
                overlay: rgb(0x1A1A1D),
                backdrop: rgb(0x000000),
                foreground: rgb(0xF4F0EA),
                muted: rgb(0x9C948C),
                border: rgb(0x26262A),
                separator: rgb(0x202024),
                accent: rgb(0xFF6A3D),
                accent_foreground: rgb(0x1A0A04),
                accent_hover: rgb(0xFF7B52),
                accent_soft: rgb(0x2A1710),
                success: rgb(0x6FBF8B),
                warning: rgb(0xD8B26A),
                danger: rgb(0xE5484D),
                danger_foreground: rgb(0x1A0A04),
                focus_ring: rgb(0xFF6A3D),
            },
            radii: Radii {
                sm: 6.0,
                md: 8.0,
                lg: 12.0,
                xl: 20.0,
                field: 12.0,
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 16.0,
                xl: 24.0,
                xxl: 40.0,
                page: 64.0,
            },
            sizes: Sizes {
                control: 32.0,
                control_lg: 40.0,
                field: 40.0,
                row: 44.0,
                topbar: 56.0,
                select: 220.0,
                page_max: 1100.0,
                button_sm: 28.0,
                button_md: 36.0,
                button_lg: 40.0,
                button_xl: 52.0,
            },
            type_scale: Typography {
                sans: "Medeo Sans",
                mono: "Medeo Mono",
                caption: 12.0,
                body: 14.0,
                body_lg: 15.0,
                subtitle: 16.0,
                title: 20.0,
                heading: 40.0,
                hero: 72.0,
                display: 32.0,
            },
            motion: Motion::new(),
        }
    }

    /// Light keeps the same surfaces and accent, inverted around a warm paper canvas.
    pub fn light() -> Self {
        Self {
            dark: false,
            colors: Colors {
                background: rgb(0xF1F1F1),
                surface: rgb(0xFFFFFF),
                surface_secondary: rgb(0xF7F7F7),
                surface_tertiary: rgb(0xEDEDED),
                overlay: rgb(0xFFFFFF),
                backdrop: rgb(0x000000),
                foreground: rgb(0x111214),
                muted: rgb(0x64676B),
                border: rgb(0xE3E3E3),
                separator: rgb(0xECECEC),
                accent: rgb(0x111214),
                accent_foreground: rgb(0xFFFFFF),
                accent_hover: rgb(0x2A2A2A),
                accent_soft: rgb(0xF2F2F2),
                success: rgb(0x111214),
                warning: rgb(0x8A5A12),
                danger: rgb(0xB42318),
                danger_foreground: rgb(0xFFFFFF),
                focus_ring: rgb(0x111214),
            },
            radii: Radii {
                sm: 6.0,
                md: 8.0,
                lg: 12.0,
                xl: 20.0,
                field: 12.0,
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 16.0,
                xl: 24.0,
                xxl: 40.0,
                page: 64.0,
            },
            sizes: Sizes {
                control: 32.0,
                control_lg: 40.0,
                field: 40.0,
                row: 44.0,
                topbar: 56.0,
                select: 220.0,
                page_max: 1100.0,
                button_sm: 28.0,
                button_md: 36.0,
                button_lg: 40.0,
                button_xl: 52.0,
            },
            type_scale: Typography {
                sans: "Medeo Sans",
                mono: "Medeo Mono",
                caption: 12.0,
                body: 14.0,
                body_lg: 15.0,
                subtitle: 16.0,
                title: 20.0,
                heading: 40.0,
                hero: 72.0,
                display: 32.0,
            },
            motion: Motion::new(),
        }
    }

    pub fn px(value: f32) -> gpui::Pixels {
        gpui::px(value)
    }

    /// The design tokens as CSS custom properties. GPUI reads the Rust values; this file is the
    /// same contract for the website, docs and tools, and the test below keeps the two in sync.
    pub fn variables_css(&self) -> String {
        let mut css = String::from(
            "/* Generated from launcher/ui-kit/src/tokens.rs; do not edit by hand. */\n:root {\n",
        );
        let c = self.colors;
        for (name, value) in [
            ("background", c.background),
            ("surface", c.surface),
            ("surface-2", c.surface_secondary),
            ("surface-3", c.surface_tertiary),
            ("overlay", c.overlay),
            ("foreground", c.foreground),
            ("muted", c.muted),
            ("border", c.border),
            ("separator", c.separator),
            ("accent", c.accent),
            ("accent-foreground", c.accent_foreground),
            ("accent-hover", c.accent_hover),
            ("accent-soft", c.accent_soft),
            ("success", c.success),
            ("warning", c.warning),
            ("danger", c.danger),
            ("danger-foreground", c.danger_foreground),
            ("focus-ring", c.focus_ring),
        ] {
            let _ = writeln!(css, "  --color-{name}: {};", css_hex(value));
        }
        let sp = self.spacing;
        for (name, value) in [
            ("xs", sp.xs),
            ("sm", sp.sm),
            ("md", sp.md),
            ("lg", sp.lg),
            ("xl", sp.xl),
            ("xxl", sp.xxl),
            ("page", sp.page),
        ] {
            let _ = writeln!(css, "  --space-{name}: {value}px;");
        }
        let r = self.radii;
        for (name, value) in [
            ("sm", r.sm),
            ("md", r.md),
            ("lg", r.lg),
            ("xl", r.xl),
            ("field", r.field),
        ] {
            let _ = writeln!(css, "  --radius-{name}: {value}px;");
        }
        let z = self.sizes;
        for (name, value) in [
            ("control", z.control),
            ("control-lg", z.control_lg),
            ("field", z.field),
            ("row", z.row),
            ("topbar", z.topbar),
            ("select", z.select),
            ("page-max", z.page_max),
            ("button-sm", z.button_sm),
            ("button-md", z.button_md),
            ("button-lg", z.button_lg),
            ("button-xl", z.button_xl),
        ] {
            let _ = writeln!(css, "  --size-{name}: {value}px;");
        }
        let t = self.type_scale;
        let _ = writeln!(css, "  --font-sans: \"{}\";", t.sans);
        let _ = writeln!(css, "  --font-mono: \"{}\";", t.mono);
        for (name, value) in [
            ("caption", t.caption),
            ("body", t.body),
            ("body-lg", t.body_lg),
            ("subtitle", t.subtitle),
            ("title", t.title),
            ("heading", t.heading),
            ("hero", t.hero),
            ("display", t.display),
        ] {
            let _ = writeln!(css, "  --type-{name}: {value}px;");
        }
        let m = self.motion;
        for (name, value) in [
            ("fast", m.fast),
            ("base", m.base),
            ("quick", m.quick),
            ("move", m.r#move),
            ("enter", m.enter),
            ("stagger", m.stagger),
        ] {
            let _ = writeln!(css, "  --motion-{name}: {}ms;", value.as_millis());
        }
        let _ = writeln!(css, "  --motion-rise: {}px;", m.rise);
        css.push_str("}\n\n:root[data-theme=\"light\"] {\n");
        let light = Theme::light().colors;
        for (name, value) in [
            ("background", light.background),
            ("surface", light.surface),
            ("surface-2", light.surface_secondary),
            ("surface-3", light.surface_tertiary),
            ("overlay", light.overlay),
            ("foreground", light.foreground),
            ("muted", light.muted),
            ("border", light.border),
            ("separator", light.separator),
            ("accent", light.accent),
            ("accent-foreground", light.accent_foreground),
            ("accent-hover", light.accent_hover),
            ("accent-soft", light.accent_soft),
            ("success", light.success),
            ("warning", light.warning),
            ("danger", light.danger),
            ("danger-foreground", light.danger_foreground),
            ("focus-ring", light.focus_ring),
        ] {
            let _ = writeln!(css, "  --color-{name}: {};", css_hex(value));
        }
        css.push_str("}\n");
        css
    }
}

fn css_hex(color: Rgba) -> String {
    let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    let rgb = color.color;
    if color.alpha >= 0.999 {
        format!(
            "#{:02X}{:02X}{:02X}",
            channel(rgb.red),
            channel(rgb.green),
            channel(rgb.blue)
        )
    } else {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            channel(rgb.red),
            channel(rgb.green),
            channel(rgb.blue),
            channel(color.alpha)
        )
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
    fn variables_css_is_in_sync() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/_variables.css");
        let expected = Theme::dark().variables_css();
        if std::env::var_os("WRITE_VARIABLES").is_some() {
            std::fs::write(path, &expected).expect("write _variables.css");
        }
        assert_eq!(
            std::fs::read_to_string(path).expect("_variables.css exists"),
            expected,
            "run WRITE_VARIABLES=1 cargo test -p scoria-ui-kit variables_css_is_in_sync"
        );
    }

    #[test]
    fn text_is_readable_in_both_themes() {
        for theme in [Theme::dark(), Theme::light()] {
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
        for theme in [Theme::dark(), Theme::light()] {
            for value in [
                theme.spacing.xs,
                theme.spacing.sm,
                theme.spacing.md,
                theme.spacing.lg,
                theme.spacing.xl,
                theme.spacing.xxl,
                theme.spacing.page,
            ] {
                assert_eq!(value % 4.0, 0.0, "spacing {value} leaves the 4px rhythm");
            }
            assert!(theme.radii.sm < theme.radii.md);
            assert!(theme.radii.md < theme.radii.lg);
            assert!(theme.radii.lg < theme.radii.xl);
        }
    }
}

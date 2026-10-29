import { flintDark, flintLight, type FlintTheme } from "@/lib/flint"

/**
 * Scoria: ember over basalt.
 *
 * One accent, one surface language: a basalt window, warm paper text, and the
 * same ember that runs through the Scoria mark for anything a player can press.
 * Every control is a pill or a soft rounded panel; nothing has a hard corner.
 * The blur belongs to the launcher's own procedural backdrop, never a game shot.
 */
const nightly = {
  background: "#09080A",
  foreground: "#F7EFE7",
  card: "#121014",
  cardForeground: "#F7EFE7",
  popover: "#171310",
  popoverForeground: "#F7EFE7",
  primary: "#FF8A4C",
  primaryForeground: "#1B0D06",
  primaryHover: "#FFA46E",
  secondary: "#221B17",
  secondaryForeground: "#F3E9DF",
  secondaryHover: "#2C231D",
  muted: "#100D0C",
  mutedForeground: "#B3A293",
  accent: "#261C16",
  accentForeground: "#FFF4EA",
  destructive: "#E5484D",
  destructiveForeground: "#FFF4F5",
  destructiveHover: "#F05A60",
  success: "#7FD8A4",
  successForeground: "#04160B",
  successBorder: "#2C6346",
  border: "#2E241E",
  input: "#4A382C",
  ring: "#FF9A5E",
  overlay: "#070404CC",
  selection: "#FF8A4C44",
  caret: "#FF9A5E",
}

const ember = {
  background: "#FBF5EE",
  foreground: "#1D1510",
  card: "#FFFFFF",
  cardForeground: "#1D1510",
  popover: "#FFFFFF",
  popoverForeground: "#1D1510",
  primary: "#B7410E",
  primaryForeground: "#FFFFFF",
  primaryHover: "#9C360B",
  secondary: "#F3E4D6",
  secondaryForeground: "#1D1510",
  secondaryHover: "#EAD6C4",
  muted: "#F6ECE1",
  mutedForeground: "#756456",
  accent: "#F3E0CE",
  accentForeground: "#1D1510",
  destructive: "#B4232A",
  destructiveForeground: "#FFF4F5",
  destructiveHover: "#C62D34",
  success: "#2C6E4F",
  successForeground: "#F4FFF8",
  successBorder: "#A5C8B4",
  border: "#E8D8C8",
  input: "#B9A08B",
  ring: "#B7410E",
  overlay: "#1D151066",
  selection: "#B7410E33",
  caret: "#B7410E",
}

const radius = { xs: 6, sm: 8, md: 10, lg: 14, xl: 18, full: 999 }
const text = { xs: 10, sm: 11, base: 13, md: 14, lg: 17, xl: 22, "2xl": 28, "3xl": 36 }
const leading = { tight: 1.1, normal: 1.45, relaxed: 1.6 }

export const scoriaDark: FlintTheme = {
  ...flintDark,
  colors: { ...flintDark.colors, ...nightly },
  radius,
  text,
  leading,
  shadow: {
    sm: [{ offsetX: 0, offsetY: 2, blurRadius: 8, spreadRadius: -2, color: "#08040366" }],
    md: [{ offsetX: 0, offsetY: 12, blurRadius: 32, spreadRadius: -10, color: "#08040399" }],
    lg: [{ offsetX: 0, offsetY: 30, blurRadius: 80, spreadRadius: -20, color: "#080403CC" }],
  },
}

export const scoriaLight: FlintTheme = {
  ...flintLight,
  colors: { ...flintLight.colors, ...ember },
  radius,
  text,
  leading,
  shadow: {
    sm: [{ offsetX: 0, offsetY: 2, blurRadius: 8, spreadRadius: -2, color: "#3A211414" }],
    md: [{ offsetX: 0, offsetY: 12, blurRadius: 32, spreadRadius: -10, color: "#3A211422" }],
    lg: [{ offsetX: 0, offsetY: 30, blurRadius: 80, spreadRadius: -20, color: "#3A211433" }],
  },
}

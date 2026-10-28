import { flintDark, flintLight, type FlintTheme } from "@/lib/flint"

/**
 * Scoria: violet light over a blurred world.
 *
 * One accent, one surface language: a deep indigo window, lavender text, and a
 * single iris accent for anything a player can press. Every control is a pill or
 * a soft rounded panel; nothing has a hard corner. The blur belongs to the
 * launcher's own procedural backdrop, never to a game screenshot.
 */
const nightly = {
  background: "#0A0712",
  foreground: "#F4EFFF",
  card: "#151020",
  cardForeground: "#F4EFFF",
  popover: "#1A1428",
  popoverForeground: "#F4EFFF",
  primary: "#8B5CF6",
  primaryForeground: "#FFFFFF",
  primaryHover: "#A78BFA",
  secondary: "#211A31",
  secondaryForeground: "#EDE8F8",
  secondaryHover: "#2B2240",
  muted: "#120E1B",
  mutedForeground: "#A49BB8",
  accent: "#251D37",
  accentForeground: "#F8F5FF",
  destructive: "#E5484D",
  destructiveForeground: "#FFF4F5",
  destructiveHover: "#F05A60",
  success: "#7FD8A4",
  successForeground: "#04160B",
  successBorder: "#2C6346",
  border: "#2B2340",
  input: "#6F6486",
  ring: "#A78BFA",
  overlay: "#07040ECC",
  selection: "#8B5CF644",
  caret: "#A78BFA",
}

const lilac = {
  background: "#EEEAF6",
  foreground: "#171020",
  card: "#FAF7FF",
  cardForeground: "#171020",
  popover: "#FFFFFF",
  popoverForeground: "#171020",
  primary: "#6D28D9",
  primaryForeground: "#FFFFFF",
  primaryHover: "#7C3AED",
  secondary: "#E4DDF2",
  secondaryForeground: "#171020",
  secondaryHover: "#D8CEEE",
  muted: "#E9E3F4",
  mutedForeground: "#6D647D",
  accent: "#E0D8EF",
  accentForeground: "#171020",
  destructive: "#B4232A",
  destructiveForeground: "#FFF4F5",
  destructiveHover: "#C62D34",
  success: "#2C6E4F",
  successForeground: "#F4FFF8",
  successBorder: "#A5C8B4",
  border: "#D4CBE6",
  input: "#968AA9",
  ring: "#6D28D9",
  overlay: "#17102066",
  selection: "#6D28D933",
  caret: "#6D28D9",
}

const radius = { xs: 10, sm: 14, md: 18, lg: 22, xl: 28, full: 999 }
const text = { xs: 10, sm: 11, base: 13, md: 14, lg: 17, xl: 22, "2xl": 30, "3xl": 48 }
const leading = { tight: 1.1, normal: 1.45, relaxed: 1.6 }

export const scoriaDark: FlintTheme = {
  ...flintDark,
  colors: { ...flintDark.colors, ...nightly },
  radius,
  text,
  leading,
  shadow: {
    sm: [{ offsetX: 0, offsetY: 2, blurRadius: 8, spreadRadius: -2, color: "#05030A66" }],
    md: [{ offsetX: 0, offsetY: 12, blurRadius: 32, spreadRadius: -10, color: "#05030A99" }],
    lg: [{ offsetX: 0, offsetY: 30, blurRadius: 80, spreadRadius: -20, color: "#05030ACC" }],
  },
}

export const scoriaLight: FlintTheme = {
  ...flintLight,
  colors: { ...flintLight.colors, ...lilac },
  radius,
  text,
  leading,
  shadow: {
    sm: [{ offsetX: 0, offsetY: 2, blurRadius: 8, spreadRadius: -2, color: "#2A1A4414" }],
    md: [{ offsetX: 0, offsetY: 12, blurRadius: 32, spreadRadius: -10, color: "#2A1A4422" }],
    lg: [{ offsetX: 0, offsetY: 30, blurRadius: 80, spreadRadius: -20, color: "#2A1A4433" }],
  },
}

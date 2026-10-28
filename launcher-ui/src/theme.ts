import { flintDark, flintLight, type FlintTheme } from "@/lib/flint"

/**
 * Scoria: basalt and ember. Warm black rock, ash text, one hot accent.
 * Everything below reads these tokens; no component carries a colour literal.
 */
const basalt = {
  background: "#080605",
  foreground: "#F7F1E8",
  card: "#100D0C",
  cardForeground: "#F7F1E8",
  popover: "#171211",
  popoverForeground: "#F7F1E8",
  primary: "#FF6B35",
  primaryForeground: "#180A03",
  primaryHover: "#FF8A5C",
  secondary: "#1B1512",
  secondaryForeground: "#F7F1E8",
  secondaryHover: "#27201C",
  muted: "#171210",
  mutedForeground: "#A99D91",
  accent: "#221A16",
  accentForeground: "#FFF6EE",
  destructive: "#E5484D",
  destructiveForeground: "#FFF4F4",
  destructiveHover: "#F2555A",
  success: "#8FD3A8",
  successForeground: "#0C1F14",
  successBorder: "#2C5A3F",
  border: "#2C2420",
  input: "#6B5F55",
  ring: "#FF6B35",
  overlay: "#000000CC",
  selection: "#FF6B3540",
  caret: "#FF6B35",
}

const paper = {
  background: "#F3EDE4",
  foreground: "#1A1512",
  card: "#FBF7F0",
  cardForeground: "#1A1512",
  popover: "#FFFDF8",
  popoverForeground: "#1A1512",
  primary: "#D24E1B",
  primaryForeground: "#FFF8F2",
  primaryHover: "#E2602C",
  secondary: "#E9E0D3",
  secondaryForeground: "#1A1512",
  secondaryHover: "#DED3C4",
  muted: "#EEE7DC",
  mutedForeground: "#6D6257",
  accent: "#E7DCCC",
  accentForeground: "#1A1512",
  destructive: "#C0323A",
  destructiveForeground: "#FFF4F4",
  destructiveHover: "#CC3A42",
  success: "#3E7A57",
  successForeground: "#F2FFF7",
  successBorder: "#8FBFA2",
  border: "#D8CCBD",
  input: "#9A8B7B",
  ring: "#D24E1B",
  overlay: "#1A151266",
  selection: "#D24E1B33",
  caret: "#D24E1B",
}

export const scoriaDark: FlintTheme = {
  ...flintDark,
  colors: { ...flintDark.colors, ...basalt },
  radius: { xs: 4, sm: 6, md: 10, lg: 14, xl: 18, full: 999 },
  text: { xs: 11, sm: 12, base: 13, md: 14, lg: 16, xl: 20, "2xl": 24, "3xl": 36 },
  shadow: {
    sm: [{ offsetX: 0, offsetY: 1, blurRadius: 4, spreadRadius: 0, color: "#00000066" }],
    md: [{ offsetX: 0, offsetY: 10, blurRadius: 30, spreadRadius: -8, color: "#00000099" }],
    lg: [{ offsetX: 0, offsetY: 28, blurRadius: 70, spreadRadius: -18, color: "#000000CC" }],
  },
}

export const scoriaLight: FlintTheme = {
  ...flintLight,
  colors: { ...flintLight.colors, ...paper },
  radius: { xs: 4, sm: 6, md: 10, lg: 14, xl: 18, full: 999 },
  text: { xs: 11, sm: 12, base: 13, md: 14, lg: 16, xl: 20, "2xl": 24, "3xl": 36 },
  shadow: {
    sm: [{ offsetX: 0, offsetY: 1, blurRadius: 4, spreadRadius: 0, color: "#00000022" }],
    md: [{ offsetX: 0, offsetY: 10, blurRadius: 30, spreadRadius: -8, color: "#3A2A1A33" }],
    lg: [{ offsetX: 0, offsetY: 28, blurRadius: 70, spreadRadius: -18, color: "#3A2A1A44" }],
  },
}

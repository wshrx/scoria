import { flintDark, flintLight, type FlintTheme } from "@/lib/flint"

/**
 * Scoria: blue fire on basalt.
 *
 * The palette comes from sulfur vents: cold black rock, bone-coloured ash, and one
 * electric blue that only appears where something is alive (the primary action,
 * a prepared install, the lava seam in the hero). Rust and gold are reserved for
 * warnings and for Minecraft's own metadata; green is never a brand colour.
 */
const basalt = {
  background: "#080A0E",
  foreground: "#EEE9DD",
  card: "#0D1017",
  cardForeground: "#EEE9DD",
  popover: "#131824",
  popoverForeground: "#EEE9DD",
  primary: "#6C9EFF",
  primaryForeground: "#06101F",
  primaryHover: "#8AB4FF",
  secondary: "#161C27",
  secondaryForeground: "#E7E2D7",
  secondaryHover: "#202838",
  muted: "#0B0E14",
  mutedForeground: "#8B93A4",
  accent: "#151B26",
  accentForeground: "#F3EEE2",
  destructive: "#E25B5B",
  destructiveForeground: "#FFF4F4",
  destructiveHover: "#F06A6A",
  success: "#8ED9B2",
  successForeground: "#07160D",
  successBorder: "#2E6047",
  border: "#212836",
  input: "#596478",
  ring: "#6C9EFF",
  overlay: "#05070BCC",
  selection: "#6C9EFF44",
  caret: "#8AB4FF",
}

const paper = {
  background: "#F0ECE3",
  foreground: "#191C22",
  card: "#F8F4EB",
  cardForeground: "#191C22",
  popover: "#FFFCF5",
  popoverForeground: "#191C22",
  primary: "#1F4FD8",
  primaryForeground: "#F7F9FF",
  primaryHover: "#2E60E9",
  secondary: "#E5DFD2",
  secondaryForeground: "#191C22",
  secondaryHover: "#D8D0C0",
  muted: "#EAE4D8",
  mutedForeground: "#6B6C66",
  accent: "#E1DACB",
  accentForeground: "#191C22",
  destructive: "#B3261E",
  destructiveForeground: "#FFF4F4",
  destructiveHover: "#C0332B",
  success: "#2C6E4F",
  successForeground: "#F4FFF8",
  successBorder: "#A5C8B4",
  border: "#D3CABB",
  input: "#948A79",
  ring: "#1F4FD8",
  overlay: "#191C2266",
  selection: "#1F4FD833",
  caret: "#1F4FD8",
}

const radius = { xs: 2, sm: 3, md: 4, lg: 6, xl: 10, full: 999 }
const text = { xs: 10, sm: 11, base: 13, md: 14, lg: 17, xl: 22, "2xl": 30, "3xl": 48 }
const leading = { tight: 1.15, normal: 1.45, relaxed: 1.6 }

export const scoriaDark: FlintTheme = {
  ...flintDark,
  colors: { ...flintDark.colors, ...basalt },
  radius,
  text,
  leading,
  shadow: {
    sm: [{ offsetX: 0, offsetY: 1, blurRadius: 3, spreadRadius: 0, color: "#00000066" }],
    md: [{ offsetX: 0, offsetY: 8, blurRadius: 24, spreadRadius: -6, color: "#00000099" }],
    lg: [{ offsetX: 0, offsetY: 24, blurRadius: 60, spreadRadius: -16, color: "#000000BB" }],
  },
}

export const scoriaLight: FlintTheme = {
  ...flintLight,
  colors: { ...flintLight.colors, ...paper },
  radius,
  text,
  leading,
  shadow: {
    sm: [{ offsetX: 0, offsetY: 1, blurRadius: 3, spreadRadius: 0, color: "#191C2214" }],
    md: [{ offsetX: 0, offsetY: 10, blurRadius: 30, spreadRadius: -8, color: "#191C2222" }],
    lg: [{ offsetX: 0, offsetY: 26, blurRadius: 70, spreadRadius: -20, color: "#191C222E" }],
  },
}

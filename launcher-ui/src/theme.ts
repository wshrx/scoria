import { flintDark, flintLight, type FlintTheme } from "@/lib/flint"

/**
 * Scoria's palette: flint's structure with an ember accent instead of sky.
 * Every colour in the app comes from here — no literals in components.
 */
const ember = {
  primary: "#F97316",
  primaryHover: "#FB923C",
  ring: "#F97316",
}

export const scoriaDark: FlintTheme = {
  ...flintDark,
  colors: {
    ...flintDark.colors,
    ...ember,
    primaryForeground: "#170A02",
    selection: "#F9731633",
  },
}

export const scoriaLight: FlintTheme = {
  ...flintLight,
  colors: {
    ...flintLight.colors,
    ...ember,
    primaryForeground: "#FFFFFF",
    selection: "#F9731622",
  },
}

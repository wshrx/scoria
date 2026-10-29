import { useTheme } from "@/lib/flint"

/**
 * A blocky skyline, built from blocks instead of a picture: the Nether's basalt
 * deltas, abstracted. Deterministic heights so the window is stable between
 * renders; the last strip catches the ember light.
 */
const HEIGHTS = [
  18, 26, 22, 34, 30, 42, 38, 52, 46, 40, 56, 62, 54, 48, 60, 72, 66, 58, 74, 82, 76, 68, 58,
  64, 70, 62, 54, 60, 52, 44, 50, 42, 36, 44, 38, 30, 36, 28, 22, 30, 24, 16, 22, 14,
]

export function Terrain({ height = 92 }: { height?: number }) {
  const theme = useTheme()
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        alignItems: "flex-end",
        gap: 2,
        width: "100%",
        height,
        overflow: "hidden",
      }}
    >
      {HEIGHTS.map((h, i) => {
        const hot = i > HEIGHTS.length - 9
        return (
          <div
            key={i}
            style={{
              flexGrow: 1,
              height: (h / 82) * height,
              backgroundColor: hot ? theme.colors.primary : theme.colors.border,
              opacity: hot ? 0.85 : i % 7 === 0 ? 0.9 : 0.55,
            }}
          />
        )
      })}
    </div>
  )
}

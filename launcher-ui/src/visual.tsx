import type { ReactNode } from "react"
import type { LinearGradientBackground } from "@gpuix/react"
import { useTheme } from "@/lib/flint"

export const gradient = (from: string, to: string, angle = 160): LinearGradientBackground => ({
  type: "linear-gradient",
  angle,
  stops: [
    { color: from, position: 0 },
    { color: to, position: 1 },
  ],
})

const RIDGE = [
  22, 34, 28, 46, 40, 58, 50, 70, 62, 54, 74, 84, 76, 66, 82, 96, 88, 74, 92, 104, 96, 86, 72,
  80, 88, 78, 66, 74, 64, 54, 62, 50, 44, 54, 46, 36, 44, 34, 26, 36, 28, 20, 26, 16,
]

/** Layered basalt ridges and an ember sun, drawn from blocks instead of an image. */
export function HeroScene({ height = 300 }: { height?: number }) {
  const theme = useTheme()

  return (
    <div style={{ position: "absolute", left: 0, right: 0, bottom: 0, height, overflow: "hidden" }}>
      <div
        style={{
          position: "absolute",
          right: 380,
          top: 26,
          width: 150,
          height: 150,
          borderRadius: theme.radius.full,
          backgroundColor: "#FF6B35",
          opacity: 0.16,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 120, spreadRadius: 20, color: "#FF6B3555" },
        }}
      />
      <div
        style={{
          position: "absolute",
          right: 416,
          top: 62,
          width: 78,
          height: 78,
          borderRadius: theme.radius.full,
          backgroundColor: "#FFB08A",
          opacity: 0.7,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: height * 0.72,
          display: "flex",
          flexDirection: "row",
          alignItems: "flex-end",
          gap: 1,
        }}
      >
        {RIDGE.map((h, i) => (
          <div
            key={i}
            style={{
              flexGrow: 1,
              height: (h / 104) * height * 0.72,
              backgroundColor: i % 5 === 0 ? "#33231C" : "#241A16",
              opacity: 0.85,
            }}
          />
        ))}
      </div>
      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: height * 0.46,
          display: "flex",
          flexDirection: "row",
          alignItems: "flex-end",
          gap: 1,
        }}
      >
        {RIDGE.slice(4).map((h, i) => (
          <div
            key={i}
            style={{
              flexGrow: 1,
              height: (h / 104) * height * 0.46,
              backgroundColor: i % 7 === 0 ? "#4A2E20" : "#171110",
              opacity: 0.95,
            }}
          />
        ))}
      </div>
      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: 120,
          background: gradient("#0B090800", "#0B0908", 0),
        }}
      />
    </div>
  )
}

/** A world or instance thumbnail: a gradient sky with a blocky horizon. */
export function Thumb({ seed = 0, width = 96, height = 62 }: { seed?: number; width?: number; height?: number }) {
  const theme = useTheme()
  const skies = [
    ["#3B2017", "#0E0B09"],
    ["#1C2A33", "#0B0E10"],
    ["#2A2333", "#0B090D"],
    ["#332A18", "#0C0A08"],
  ]
  const [from, to] = skies[seed % skies.length]
  return (
    <div
      style={{
        width,
        height,
        flexShrink: 0,
        position: "relative",
        overflow: "hidden",
        borderRadius: theme.radius.sm,
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: theme.colors.border,
        background: gradient(from, to, 150),
      }}
    >
      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: height * 0.42,
          display: "flex",
          flexDirection: "row",
          alignItems: "flex-end",
          gap: 1,
        }}
      >
        {[10, 18, 14, 22, 16, 26, 20, 28, 12, 18].map((h, i) => (
          <div
            key={i}
            style={{
              flexGrow: 1,
              height: h,
              backgroundColor: i % 3 === 0 ? theme.colors.primary : theme.colors.border,
              opacity: i % 3 === 0 ? 0.55 : 0.9,
            }}
          />
        ))}
      </div>
      <div
        style={{
          position: "absolute",
          right: 12,
          top: 10,
          width: 14,
          height: 14,
          borderRadius: theme.radius.full,
          backgroundColor: theme.colors.primary,
          opacity: 0.8,
        }}
      />
    </div>
  )
}

export function Panel({ children, style }: { children?: ReactNode; style?: Record<string, unknown> }) {
  const theme = useTheme()
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        borderRadius: theme.radius.lg,
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: "#FFFFFF14",
        background: gradient("#161211F2", "#0F0C0BF7", 160),
        boxShadow: theme.shadow.md,
        ...style,
      }}
    >
      {children}
    </div>
  )
}

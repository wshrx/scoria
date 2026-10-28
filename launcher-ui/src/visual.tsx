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

const NIGHT = "#080A0E"
const ASH = "#B9C3D6"
const BLUE = "#6C9EFF"
const BONE = "#EEE9DD"

/** The mark: a block of basalt with one live cell. */
export function Mark({ size = 30 }: { size?: number }) {
  const cell = Math.round(size / 3) - 1
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        width: size,
        height: size,
        gap: 1,
      }}
    >
      {[0, 1, 2].map((row) => (
        <div key={row} style={{ display: "flex", flexDirection: "row", gap: 1, flexGrow: 1 }}>
          {[0, 1, 2].map((col) => {
            const live = row === 1 && col === 1
            return (
              <div
                key={col}
                style={{
                  width: cell,
                  flexGrow: 1,
                  backgroundColor: live ? BLUE : "#2A3346",
                  boxShadow: live
                    ? { offsetX: 0, offsetY: 0, blurRadius: 14, spreadRadius: 1, color: "#6C9EFF88" }
                    : undefined,
                }}
              />
            )
          })}
        </div>
      ))}
    </div>
  )
}

/** The volcano: a voxel ridge with a cold blue seam burning through its crater. */
export function Vista({ height = 360 }: { height?: number }) {
  return (
    <div
      style={{
        position: "absolute",
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
        overflow: "hidden",
        background: gradient("#101728", "#06070B", 155),
      }}
    >
      {/* A faint survey grid over the sky. */}
      <div style={{ position: "absolute", left: 0, right: 0, top: 0, height: height * 0.58, display: "flex", flexDirection: "row" }}>
        {Array.from({ length: 18 }, (_, i) => (
          <div
            key={i}
            style={{
              flexGrow: 1,
              borderRightWidth: 1,
              borderStyle: "solid",
              borderColor: i === 17 ? "#00000000" : "#9FB6E00A",
            }}
          />
        ))}
      </div>

      {/* The moon, cold and close. */}
      <div
        style={{
          position: "absolute",
          right: 148,
          top: 40,
          width: 126,
          height: 126,
          borderRadius: 63,
          backgroundColor: "#E9E2CB",
          boxShadow: [
            { offsetX: 0, offsetY: 0, blurRadius: 90, spreadRadius: 12, color: "#6C9EFF33" },
            { offsetX: 0, offsetY: 0, blurRadius: 18, spreadRadius: -4, color: "#E9E2CB22" },
          ],
        }}
      >
        {[
          [28, 34, 14], [72, 26, 9], [64, 82, 17], [96, 58, 7], [38, 92, 8],
        ].map(([x, y, size], index) => (
          <div
            key={index}
            style={{
              position: "absolute",
              left: x,
              top: y,
              width: size,
              height: size,
              borderRadius: size / 2,
              backgroundColor: "#C9C0A6",
              opacity: 0.35,
            }}
          />
        ))}
      </div>
      <div
        style={{
          position: "absolute",
          right: 126,
          top: 18,
          width: 170,
          height: 170,
          borderRadius: 85,
          borderWidth: 1,
          borderStyle: "solid",
          borderColor: "#E9E2CB18",
        }}
      />

      {[
        [64, 52, 2], [114, 32, 1], [212, 76, 2], [286, 38, 1], [352, 92, 2],
        [420, 48, 1], [486, 108, 2], [558, 34, 1], [624, 72, 2], [698, 122, 1],
        [760, 56, 2], [834, 96, 1],
      ].map(([right, top, size], i) => (
        <div
          key={i}
          style={{
            position: "absolute",
            right,
            top,
            width: size,
            height: size,
            backgroundColor: i % 4 === 0 ? ASH : "#7E8CA6",
            opacity: 0.7,
          }}
        />
      ))}

      <Ridge
        heights={BACK}
        bottom={0}
        height={height * 0.78}
        color="#1B2740"
        top="#3A4C6E"
        opacity={1}
        shade="#1E2B47"
      />
      <Ridge
        heights={FRONT}
        bottom={0}
        height={height * 0.7}
        color="#0C1220"
        top="#40516F"
        opacity={1}
        shade="#0F1626"
        lava={[32, 33, 34, 35, 36]}
        lavaFloor={0.54}
      />
      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: height * 0.7 - 8,
          height: 22,
          background: gradient("#1B274000", "#6C9EFF14", 90),
          opacity: 0.8,
        }}
      />

      {/* Embers rising from the crater. */}
      {[
        [376, 150, 3], [414, 114, 2], [452, 184, 2], [492, 132, 3], [530, 206, 2],
      ].map(([right, top, size], i) => (
        <div
          key={i}
          style={{
            position: "absolute",
            right,
            top,
            width: size,
            height: size,
            backgroundColor: BLUE,
            boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 12, spreadRadius: 2, color: "#6C9EFFAA" },
            opacity: 0.8,
          }}
        />
      ))}

      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: height * 0.36,
          background: gradient("#080A0E00", NIGHT, 0),
        }}
      />
    </div>
  )
}

const BACK = [
  0.15, 0.17, 0.16, 0.19, 0.22, 0.20, 0.24, 0.27, 0.25, 0.29, 0.33, 0.31,
  0.35, 0.39, 0.37, 0.42, 0.47, 0.45, 0.50, 0.55, 0.52, 0.58, 0.63, 0.60,
  0.66, 0.72, 0.69, 0.75, 0.80, 0.77, 0.83, 0.88, 0.85, 0.80, 0.74, 0.78,
  0.84, 0.90, 0.86, 0.79, 0.71, 0.64, 0.56, 0.48, 0.40, 0.33, 0.26, 0.20,
]

const FRONT = [
  0.05, 0.06, 0.07, 0.08, 0.09, 0.08, 0.10, 0.11, 0.12, 0.13, 0.12, 0.14,
  0.15, 0.16, 0.15, 0.17, 0.18, 0.20, 0.19, 0.21, 0.23, 0.25, 0.30, 0.36,
  0.44, 0.53, 0.63, 0.74, 0.85, 0.94, 0.88, 0.78, 0.68, 0.60, 0.56, 0.55,
  0.58, 0.64, 0.72, 0.82, 0.89, 0.83, 0.73, 0.60, 0.46, 0.32, 0.20, 0.12,
]

function Ridge({
  heights,
  bottom,
  height,
  color,
  top,
  opacity,
  lava = [],
  lavaFloor,
  shade,
}: {
  heights: number[]
  bottom: number
  height: number
  color: string
  top: string
  opacity: number
  lava?: number[]
  lavaFloor?: number
  shade?: string
}) {
  return (
    <div
      style={{
        position: "absolute",
        left: 0,
        right: 0,
        bottom,
        height,
        display: "flex",
        flexDirection: "row",
        alignItems: "flex-end",
        gap: 0,
        opacity,
      }}
    >
      {heights.map((h, i) => {
        const burning = lava.includes(i)
        return (
          <div
            key={i}
            style={{
              flexGrow: 1,
              height: (burning ? lavaFloor ?? h : h) * height,
              display: "flex",
              flexDirection: "column",
            }}
          >
            <div
              style={{
                height: burning ? 3 : 2,
                backgroundColor: burning ? BLUE : top,
                boxShadow: burning
                  ? { offsetX: 0, offsetY: 0, blurRadius: 18, spreadRadius: 4, color: "#6C9EFFCC" }
                  : undefined,
              }}
            />
            <div
              style={{
                flexGrow: 1,
                background: gradient(shade && i % 4 === 0 ? shade : color, "#06080D", 180),
              }}
            />
          </div>
        )
      })}
    </div>
  )
}

/** A world thumbnail: the same ridge, small, with its own weather. */
export function StrataThumb({ seed = 0, width = 88, height = 56 }: { seed?: number; width?: number; height?: number }) {
  const skies = [
    ["#16233C", "#080B12"],
    ["#1A2C2A", "#080D0C"],
    ["#2A2138", "#0D0913"],
    ["#2E2418", "#100B07"],
  ]
  const [from, to] = skies[Math.abs(seed) % skies.length]
  const lit = Math.abs(seed) % 5 !== 0
  return (
    <div
      style={{
        width,
        height,
        flexShrink: 0,
        position: "relative",
        overflow: "hidden",
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: "#2A3346",
        background: gradient(from, to, 155),
      }}
    >
      <div
        style={{
          position: "absolute",
          right: 9,
          top: 8,
          width: 10,
          height: 10,
          borderRadius: 5,
          backgroundColor: lit ? "#E9E2CB" : "#4A5263",
        }}
      />
      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: height * 0.62,
          display: "flex",
          flexDirection: "row",
          alignItems: "flex-end",
          gap: 1,
        }}
      >
        {[9, 14, 11, 19, 15, 24, 20, 16, 22, 18, 13, 17].map((h, i) => (
          <div
            key={i}
            style={{
              flexGrow: 1,
              height: h,
              backgroundColor: i === 5 && lit ? "#101A2E" : "#0F1626",
              borderTopWidth: 1,
              borderStyle: "solid",
              borderColor: i === 5 && lit ? BLUE : "#34425E",
            }}
          />
        ))}
      </div>
    </div>
  )
}

/** A three-pixel meter: enough to read progress, not enough to become a dashboard. */
export function Meter({ value, total, height = 3 }: { value: number; total: number; height?: number }) {
  const theme = useTheme()
  const done = total > 0 ? Math.max(0, Math.min(1, value / total)) : 0
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        width: "100%",
        height,
        overflow: "hidden",
        backgroundColor: theme.colors.muted,
      }}
    >
      <div style={{ flexGrow: done * 1000, backgroundColor: theme.colors.primary }} />
      <div style={{ flexGrow: (1 - done) * 1000 }} />
    </div>
  )
}

export function Panel({
  children,
  style,
}: {
  children?: ReactNode
  style?: Record<string, unknown>
}) {
  const theme = useTheme()
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        borderRadius: theme.radius.md,
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: theme.colors.border,
        backgroundColor: theme.colors.card,
        ...style,
      }}
    >
      {children}
    </div>
  )
}

export function Kicker({ children }: { children?: ReactNode }) {
  return (
    <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
      <div style={{ width: 14, height: 1, backgroundColor: BLUE }} />
      <text
        style={{
          color: "#A9B6CE",
          fontFamily: "Medeo Mono",
          fontSize: 10,
          lineHeight: 14,
        }}
      >
        {children}
      </text>
    </div>
  )
}

export { BLUE, BONE, NIGHT }

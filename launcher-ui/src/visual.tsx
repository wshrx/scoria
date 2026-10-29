import { useEffect, useState, type ReactNode } from "react"
import { motion, type LinearGradientBackground } from "@gpuix/react"
import { Text, useTheme } from "@/lib/flint"

export const gradient = (from: string, to: string, angle = 160): LinearGradientBackground => ({
  type: "linear-gradient",
  angle,
  stops: [
    { color: from, position: 0 },
    { color: to, position: 1 },
  ],
})

/** A SaaS mesh: graphite, a faint survey grid, soft light and slow dust. */
export function Backdrop({ strong = true, dark = true }: { strong?: boolean; dark?: boolean }) {
  const p = dark
    ? {
        skyTop: "#0B0D13",
        skyBottom: "#07080B",
        line: "#FFFFFF",
        glowA: "#8B5CF62E",
        glowB: "#2CD3A922",
        glowC: "#5B6CFF1A",
        scrim: "#08090C",
      }
    : {
        skyTop: "#F8F4FB",
        skyBottom: "#EFEAF6",
        line: "#2A1A44",
        glowA: "#8B5CF624",
        glowB: "#2CD3A91C",
        glowC: "#5B6CFF14",
        scrim: "#F4F1F7",
      }

  return (
    <div
      style={{
        position: "absolute",
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
        overflow: "hidden",
        background: gradient(p.skyTop, p.skyBottom, 155),
      }}
    >
      <div
        style={{
          position: "absolute",
          left: -220,
          top: -240,
          width: 820,
          height: 660,
          borderRadius: 340,
          background: gradient("#00000000", p.glowA, 135),
          opacity: 0.9,
        }}
      />
      <div
        style={{
          position: "absolute",
          right: -240,
          top: -180,
          width: 760,
          height: 600,
          borderRadius: 320,
          background: gradient("#00000000", p.glowB, 225),
          opacity: 0.75,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: 260,
          bottom: -320,
          width: 980,
          height: 620,
          borderRadius: 420,
          background: gradient("#00000000", p.glowC, 90),
          opacity: 0.7,
        }}
      />

      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          top: 0,
          bottom: 0,
          display: "flex",
          flexDirection: "row",
          opacity: dark ? 0.045 : 0.035,
        }}
      >
        {Array.from({ length: 32 }, (_, index) => (
          <div
            key={index}
            style={{
              flexGrow: 1,
              borderRightWidth: 1,
              borderStyle: "solid",
              borderColor: `${p.line}${index === 31 ? "00" : ""}`,
            }}
          />
        ))}
      </div>
      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          top: 0,
          bottom: 0,
          display: "flex",
          flexDirection: "column",
          opacity: dark ? 0.04 : 0.025,
        }}
      >
        {Array.from({ length: 18 }, (_, index) => (
          <div
            key={index}
            style={{
              flexGrow: 1,
              borderBottomWidth: 1,
              borderStyle: "solid",
              borderColor: `${p.line}${index === 17 ? "00" : ""}`,
            }}
          />
        ))}
      </div>

      {strong ? (
        <div
          style={{
            position: "absolute",
            left: 0,
            top: 0,
            bottom: 0,
            width: "45%",
            background: gradient(`${p.scrim}F0`, `${p.scrim}00`, 90),
            opacity: 0.4,
          }}
        />
      ) : null}
    </div>
  )
}

const DUST = Array.from({ length: 28 }, (_, index) => ({
  x: ((index * 37 + 11) % 97) / 97,
  y: ((index * 53 + 7) % 89) / 89,
  size: 1.5 + ((index * 7) % 4) * 0.75,
  speed: 0.0009 + ((index * 11) % 7) * 0.00025,
  drift: 8 + ((index * 5) % 9),
  phase: index * 1.7,
  tone: index % 5 === 0 ? "#8FD8B4" : index % 3 === 0 ? "#FFFFFF" : "#B9A7FF",
}))

/** Slow dust over the scene: the one looping ambient animation, 20 frames a second. */
export function ParticleField({ dark = true }: { dark?: boolean }) {
  const [size, setSize] = useState({ width: 1280, height: 720 })
  const [tick, setTick] = useState(0)

  useEffect(() => {
    const timer = setInterval(() => setTick((value) => value + 1), 55)
    return () => clearInterval(timer)
  }, [])

  return (
    <div
      onLayout={(event) =>
        setSize({ width: event.width ?? 1280, height: event.height ?? 720 })
      }
      style={{
        position: "absolute",
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
        overflow: "hidden",
        pointerEvents: "none",
      }}
    >
      {DUST.map((dust, index) => {
        const y = (((dust.y - tick * dust.speed) % 1) + 1) % 1
        const x = dust.x * size.width + Math.sin((tick + dust.phase) / 42) * dust.drift
        const twinkle = (dark ? 0.12 : 0.08) + (dark ? 0.58 : 0.32) * (0.5 + 0.5 * Math.sin((tick + dust.phase) / 19))
        const s = dust.size
        const tone = dark ? dust.tone : dust.tone === "#FFFFFF" ? "#6D647D" : dust.tone === "#8FD8B4" ? "#2C6E4F" : "#5B3E9A"
        return (
          <div
            key={index}
            style={{
              position: "absolute",
              left: x,
              top: y * size.height,
              width: s,
              height: s,
              borderRadius: s,
              backgroundColor: tone,
              opacity: twinkle,
            }}
          >
            <div
              style={{
                position: "absolute",
                left: -s * 1.4,
                top: -s * 1.4,
                width: s * 3.8,
                height: s * 3.8,
                borderRadius: s * 2,
                backgroundColor: tone,
                opacity: 0.1,
              }}
            />
          </div>
        )
      })}
    </div>
  )
}

/** The wordmark: a soft violet tile with an S, deliberately not a game logo. */
export function Mark({ size = 40 }: { size?: number }) {
  return (
    <div
      style={{
        position: "relative",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        width: size,
        height: size,
        borderRadius: Math.round(size * 0.27),
        background: gradient("#B49CFF", "#5B21B6", 145),
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: "#FFFFFF2E",
      }}
    >
      <Text
        style={{
          color: "#FFFFFF",
          fontSize: Math.round(size * 0.5),
          lineHeight: Math.round(size * 0.54),
          fontWeight: "700",
        }}
      >
        S
      </Text>
      <div
        style={{
          position: "absolute",
          top: Math.round(size * 0.14),
          right: Math.round(size * 0.14),
          width: Math.max(3, Math.round(size * 0.11)),
          height: Math.max(3, Math.round(size * 0.11)),
          borderRadius: 99,
          background: gradient("#FFD9A0", "#F59E0B", 135),
        }}
      />
    </div>
  )
}

export function Avatar({
  name,
  size = 30,
  style,
}: {
  name: string
  size?: number
  style?: Record<string, unknown>
}) {
  const initial = name.trim().slice(0, 1).toUpperCase() || "S"
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        width: size,
        height: size,
        borderRadius: Math.round(size * 0.32),
        background: gradient("#C4B5FD", "#7C3AED", 150),
        ...style,
      }}
    >
      <Text
        style={{
          color: "#FFFFFF",
          fontSize: Math.round(size * 0.42),
          lineHeight: Math.round(size * 0.46),
          fontWeight: "700",
        }}
      >
        {initial}
      </Text>
    </div>
  )
}

/** A pill meter: the fill eases to each new value instead of jumping. */
export function Meter({ value, total, height = 6 }: { value: number; total: number; height?: number }) {
  const theme = useTheme()
  const [track, setTrack] = useState(0)
  const done = total > 0 ? Math.max(0, Math.min(1, value / total)) : 0
  return (
    <div
      onLayout={(event) => setTrack(event.width ?? 0)}
      style={{
        display: "flex",
        flexDirection: "row",
        width: "100%",
        height,
        overflow: "hidden",
        borderRadius: height,
        backgroundColor: theme.colors.muted,
      }}
    >
      <motion.div
        initial={{ width: 0 }}
        animate={{ width: Math.max(0, done * track) }}
        transition={{ duration: 0.32, ease: "easeOut" }}
        style={{
          height,
          borderRadius: height,
          backgroundColor: theme.colors.primary,
        }}
      />
    </div>
  )
}

/** The one content surface: a hairline card on the mesh. */
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
        borderRadius: 14,
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

/** A rounded page surface over the backdrop. */
export function Sheet({
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
        width: "100%",
        borderRadius: 14,
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

/** A small painted world for a card: sky, a square sun, two soft ridges. */
export function SceneThumb({
  seed = 0,
  width = 220,
  height = 118,
}: {
  seed?: number
  width?: number | string
  height?: number
}) {
  const theme = useTheme()
  const dark = theme.appearance === "dark"
  const skies = dark
    ? [
        ["#2B2348", "#0B0814"],
        ["#1E3A36", "#080D0C"],
        ["#3A2438", "#0E080F"],
        ["#3B2A1B", "#100B07"],
      ]
    : [
        ["#E8DFF6", "#D3E8EC"],
        ["#D8EFE6", "#C8DDE2"],
        ["#F0DDEE", "#E0D5EA"],
        ["#F2E4D2", "#E4DCD0"],
      ]
  const [from, to] = skies[Math.abs(seed) % skies.length]
  const moon = dark ? "#F0E8D8" : "#FFF4D2"
  const hillA = dark ? "#2A5A4B" : "#A6CBB9"
  const hillB = dark ? "#33204A" : "#C6B2DF"
  const fade = dark ? "#0A0712" : "#F4F1F7"
  return (
    <div
      style={{
        width,
        height,
        flexShrink: 0,
        position: "relative",
        overflow: "hidden",
        borderRadius: 12,
        background: gradient(from, to, 150),
      }}
    >
      <div
        style={{
          position: "absolute",
          right: 20,
          top: 16,
          width: 24,
          height: 24,
          borderRadius: 6,
          backgroundColor: moon,
          opacity: 0.9,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: -40,
          bottom: -70,
          width: "85%",
          height: "75%",
          borderRadius: 200,
          background: gradient(`${hillA}00`, `${hillA}AA`, 80),
          opacity: 0.8,
        }}
      />
      <div
        style={{
          position: "absolute",
          right: -60,
          bottom: -90,
          width: "80%",
          height: "80%",
          borderRadius: 200,
          background: gradient(`${hillB}00`, `${hillB}CC`, 100),
          opacity: 0.9,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: "36%",
          background: gradient(`${fade}00`, `${fade}DD`, 0),
        }}
      />
    </div>
  )
}

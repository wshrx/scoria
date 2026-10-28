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

const NIGHT = "#0A0712"
const VIOLET = "#8B5CF6"
const BONE = "#F4EFFF"

/** A blurred world, painted from soft shapes: no game art, no screenshot. */
export function Backdrop({ strong = true }: { strong?: boolean }) {
  return (
    <div
      style={{
        position: "absolute",
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
        overflow: "hidden",
        background: gradient("#171024", "#06040B", 155),
      }}
    >
      <div
        style={{
          position: "absolute",
          left: -140,
          bottom: -260,
          width: 900,
          height: 700,
          borderRadius: 450,
          backgroundColor: "#1D4038",
          opacity: 0.55,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 180, spreadRadius: 40, color: "#1D403899" },
        }}
      />
      <div
        style={{
          position: "absolute",
          right: -160,
          bottom: -240,
          width: 820,
          height: 620,
          borderRadius: 410,
          backgroundColor: "#2A1A4D",
          opacity: 0.65,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 170, spreadRadius: 30, color: "#2A1A4DCC" },
        }}
      />
      <div
        style={{
          position: "absolute",
          right: 130,
          top: 40,
          width: 300,
          height: 300,
          borderRadius: 150,
          backgroundColor: "#4A2E7A",
          opacity: 0.45,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 160, spreadRadius: 30, color: "#6D4AA0AA" },
        }}
      />

      <div
        style={{
          position: "absolute",
          right: 190,
          top: 62,
          width: 118,
          height: 118,
          borderRadius: 59,
          backgroundColor: "#F0E8D8",
          opacity: 0.9,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 80, spreadRadius: 12, color: "#F0E8D855" },
        }}
      />

      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: "46%",
          background: gradient("#0C0816", "#07040D", 0),
          opacity: 0.92,
        }}
      />

      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: "44%",
          display: "flex",
          flexDirection: "row",
          alignItems: "flex-end",
          gap: 2,
          opacity: 0.8,
        }}
      >
        {[
          44, 58, 52, 76, 64, 92, 80, 110, 96, 126, 112, 146, 132, 160, 142, 118, 136, 154, 170,
          148, 126, 108, 94, 116, 98, 82, 68, 56, 46, 36,
        ].map((height, index) => {
          const body =
            index % 7 === 0 ? "#33204A" : index % 3 === 0 ? "#1C3E37" : index % 2 === 0 ? "#1D2438" : "#251B36"
          const top = index % 3 === 0 ? "#2D5D50" : index % 2 === 0 ? "#2E3A55" : "#3A2B52"
          return (
            <div
              key={index}
              style={{
                flexGrow: 1,
                height,
                display: "flex",
                flexDirection: "column",
                borderRadius: 3,
                overflow: "hidden",
                boxShadow: {
                  offsetX: 0,
                  offsetY: 0,
                  blurRadius: 30,
                  spreadRadius: 4,
                  color: index % 3 === 0 ? "#1C3E3766" : "#2A1A4566",
                },
              }}
            >
              <div style={{ height: 3, backgroundColor: top, opacity: 0.85 }} />
              <div style={{ flexGrow: 1, backgroundColor: body }} />
            </div>
          )
        })}
      </div>

      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 190,
          height: 90,
          background: gradient("#8B5CF600", "#8B5CF624", 180),
          opacity: 0.8,
        }}
      />

      {strong ? (
        <div
          style={{
            position: "absolute",
            left: 0,
            top: 0,
            bottom: 0,
            width: "62%",
            background: gradient("#0A0712F2", "#0A071200", 90),
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
export function ParticleField() {
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
        const twinkle = 0.12 + 0.58 * (0.5 + 0.5 * Math.sin((tick + dust.phase) / 19))
        const s = dust.size
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
              backgroundColor: dust.tone,
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
                backgroundColor: dust.tone,
                opacity: 0.14,
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
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        width: size,
        height: size,
        borderRadius: Math.round(size * 0.34),
        background: gradient("#A78BFA", "#6D28D9", 145),
      }}
    >
      <Text
        style={{
          color: "#FFFFFF",
          fontSize: Math.round(size * 0.46),
          lineHeight: Math.round(size * 0.5),
          fontWeight: "700",
        }}
      >
        S
      </Text>
    </div>
  )
}

export function Avatar({ name, size = 30 }: { name: string; size?: number }) {
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
        borderRadius: theme.radius.xl,
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: theme.colors.border,
        backgroundColor: theme.appearance === "dark" ? "#110C1BE8" : theme.colors.card,
        ...style,
      }}
    >
      {children}
    </div>
  )
}

/** A small, soft illustration for an instance or a world. */
export function SceneThumb({
  seed = 0,
  width = 220,
  height = 118,
}: {
  seed?: number
  width?: number | string
  height?: number
}) {
  const skies = [
    ["#2B2348", "#0B0814"],
    ["#1E3A36", "#080D0C"],
    ["#3A2438", "#0E080F"],
    ["#3B2A1B", "#100B07"],
  ]
  const [from, to] = skies[Math.abs(seed) % skies.length]
  return (
    <div
      style={{
        width,
        height,
        flexShrink: 0,
        position: "relative",
        overflow: "hidden",
        borderRadius: 16,
        background: gradient(from, to, 150),
      }}
    >
      <div
        style={{
          position: "absolute",
          right: 22,
          top: 18,
          width: 26,
          height: 26,
          borderRadius: 13,
          backgroundColor: "#F0E8D8",
          opacity: 0.85,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 24, spreadRadius: 2, color: "#F0E8D844" },
        }}
      />
      <div
        style={{
          position: "absolute",
          left: -30,
          bottom: -40,
          width: "75%",
          height: "70%",
          borderRadius: 80,
          backgroundColor: "#1E4038",
          opacity: 0.7,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 40, spreadRadius: 8, color: "#1E403899" },
        }}
      />
      <div
        style={{
          position: "absolute",
          right: -40,
          bottom: -30,
          width: "68%",
          height: "62%",
          borderRadius: 70,
          backgroundColor: "#2A1A45",
          opacity: 0.8,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 36, spreadRadius: 6, color: "#2A1A45AA" },
        }}
      />
      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: "38%",
          background: gradient("#0A071200", "#0A0712DD", 0),
        }}
      />
    </div>
  )
}

export const CHROME_MUTED = "#A49BB8"
export { BONE, NIGHT, VIOLET }

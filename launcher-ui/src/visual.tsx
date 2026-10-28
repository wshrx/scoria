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

/** A painted world: sky, aurora, a square moon, blocky ridges, trees and a lake. */
export function Backdrop({ strong = true, dark = true }: { strong?: boolean; dark?: boolean }) {
  const p = dark
    ? {
        skyTop: "#121A33",
        skyBottom: "#07050D",
        blobA: "#1D4038",
        blobB: "#2A1A4D",
        blobC: "#4A2E7A",
        moon: "#F0E8D8",
        halo: "#6D4AA0AA",
        waterTop: "#16263A",
        waterBottom: "#070A12",
        auroraA: "#8B5CF633",
        auroraB: "#2CD3A933",
        body: ["#33204A", "#1C3E37", "#1D2438", "#251B36"],
        top: ["#2D5D50", "#2E3A55", "#3A2B52"],
        scrim: "#08060F",
        glint: "#F0E8D8",
      }
    : {
        skyTop: "#F0E7F7",
        skyBottom: "#D5E7EA",
        blobA: "#BCDCCB",
        blobB: "#D5C3EB",
        blobC: "#E8D9F7",
        moon: "#FFF2C9",
        halo: "#F2C94C55",
        waterTop: "#D3E6EC",
        waterBottom: "#A8C7D2",
        auroraA: "#8B5CF628",
        auroraB: "#2CD3A928",
        body: ["#C9B8E4", "#A9CDBD", "#B8C4DE", "#CBB9D8"],
        top: ["#E2D6F0", "#C7E4D6", "#D8E0EE"],
        scrim: "#F3ECF8",
        glint: "#FFFFFF",
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
          left: -80,
          top: 54,
          width: 760,
          height: 210,
          borderRadius: 220,
          background: gradient(p.auroraA, "#00000000", 100),
          opacity: 0.85,
        }}
      />
      <div
        style={{
          position: "absolute",
          right: 90,
          top: 140,
          width: 640,
          height: 180,
          borderRadius: 220,
          background: gradient(p.auroraB, "#00000000", 80),
          opacity: 0.75,
        }}
      />

      <div
        style={{
          position: "absolute",
          left: -140,
          bottom: -260,
          width: 900,
          height: 700,
          borderRadius: 450,
          backgroundColor: p.blobA,
          opacity: dark ? 0.55 : 0.7,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 180, spreadRadius: 40, color: p.blobA },
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
          backgroundColor: p.blobB,
          opacity: dark ? 0.65 : 0.7,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 170, spreadRadius: 30, color: p.blobB },
        }}
      />
      <div
        style={{
          position: "absolute",
          right: 120,
          top: 30,
          width: 320,
          height: 320,
          borderRadius: 160,
          backgroundColor: p.blobC,
          opacity: 0.5,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 160, spreadRadius: 30, color: p.blobC },
        }}
      />

      <div
        style={{
          position: "absolute",
          right: 196,
          top: 58,
          width: 116,
          height: 116,
          borderRadius: 18,
          backgroundColor: p.moon,
          opacity: 0.95,
          boxShadow: { offsetX: 0, offsetY: 0, blurRadius: 80, spreadRadius: 12, color: p.halo },
        }}
      />

      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: "24%",
          background: gradient(p.waterTop, p.waterBottom, 0),
          opacity: 0.95,
        }}
      />
      {[
        [780, 28, 110, 0.16],
        [940, 54, 70, 0.12],
        [700, 82, 150, 0.1],
        [1010, 104, 50, 0.14],
      ].map(([left, bottom, width, opacity], index) => (
        <div
          key={index}
          style={{
            position: "absolute",
            left,
            bottom,
            width,
            height: 2,
            borderRadius: 1,
            backgroundColor: p.glint,
            opacity,
          }}
        />
      ))}

      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: "42%",
          display: "flex",
          flexDirection: "row",
          alignItems: "flex-end",
          gap: 2,
          opacity: 0.92,
        }}
      >
        {[
          44, 58, 52, 76, 64, 92, 80, 110, 96, 126, 112, 146, 132, 160, 142, 118, 136, 154, 170,
          148, 126, 108, 94, 116, 98, 82, 68, 56, 46, 36,
        ].map((height, index) => {
          const body = p.body[index % p.body.length]
          const top = p.top[index % p.top.length]
          return (
            <div
              key={index}
              style={{
                flexGrow: 1 + (index % 4),
                height: height * 0.86,
                display: "flex",
                flexDirection: "column",
                borderRadius: 3,
                overflow: "hidden",
                boxShadow: {
                  offsetX: 0,
                  offsetY: 0,
                  blurRadius: 26,
                  spreadRadius: 3,
                  color: body,
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
          bottom: 0,
          height: 150,
          background: gradient(`${p.scrim}00`, `${p.scrim}CC`, 0),
          opacity: 0.55,
        }}
      />

      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 150,
          height: 80,
          background: gradient("#8B5CF600", dark ? "#8B5CF624" : "#8B5CF612", 180),
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
            background: gradient(`${p.scrim}F2`, `${p.scrim}00`, 90),
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

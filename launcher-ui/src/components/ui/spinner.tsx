import { useEffect, useState } from "react"
import { useTheme } from "@/lib/flint"

/** Three dots, one at a time: the only looping animation in the launcher. */
export function Spinner({
  size = 3,
  gap = 3,
  color,
}: {
  size?: number
  gap?: number
  color?: string
}) {
  const theme = useTheme()
  const [tick, setTick] = useState(0)

  useEffect(() => {
    const timer = setInterval(() => setTick((value) => value + 1), 180)
    return () => clearInterval(timer)
  }, [])

  return (
    <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap, flexShrink: 0 }}>
      {[0, 1, 2].map((index) => (
        <div
          key={index}
          style={{
            width: size,
            height: size,
            borderRadius: size,
            backgroundColor: color ?? theme.colors.primaryForeground,
            opacity: (tick + index) % 3 === 0 ? 1 : 0.25,
          }}
        />
      ))}
    </div>
  )
}

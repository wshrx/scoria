import type { Style } from "@/lib/flint/utils"
import { useTheme } from "@/lib/flint"

/** A quiet placeholder for a panel that is still loading. */
export function Skeleton({
  width = "100%",
  height = 12,
  radius = 6,
  style,
}: {
  width?: number | string
  height?: number
  radius?: number
  style?: Style
}) {
  const theme = useTheme()
  return (
    <div
      style={{
        width,
        height,
        borderRadius: radius,
        backgroundColor: theme.colors.muted,
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: theme.colors.border,
        ...style,
      }}
    />
  )
}

/** Rows for a list that has not arrived yet; the height matches a real row. */
export function SkeletonList({ rows = 5, height = 52, gap = 8 }: { rows?: number; height?: number; gap?: number }) {
  return (
    <div style={{ display: "flex", flexDirection: "column", gap, width: "100%" }}>
      {Array.from({ length: rows }, (_, index) => (
        <Skeleton key={index} height={height} radius={14} />
      ))}
    </div>
  )
}

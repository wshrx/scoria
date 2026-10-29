import type { ReactNode } from "react"
import { Text, useTheme } from "@/lib/flint"

/** One heading rhythm for every screen. */
export function PageHeader({
  title,
  subtitle,
  actions,
}: {
  title: string
  subtitle?: string
  actions?: ReactNode
}) {
  const theme = useTheme()
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        alignItems: "flex-end",
        gap: 16,
        paddingBottom: 4,
        borderBottomWidth: 1,
        borderStyle: "solid",
        borderColor: theme.colors.border,
      }}
    >
      <div style={{ display: "flex", flexDirection: "column", gap: 4, flexGrow: 1, minWidth: 0 }}>
        <Text style={{ fontSize: 28, lineHeight: 32, fontWeight: "700" }}>{title}</Text>
        {subtitle ? (
          <Text size="sm" tone="muted">
            {subtitle}
          </Text>
        ) : null}
      </div>
      {actions}
    </div>
  )
}

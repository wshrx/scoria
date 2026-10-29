import type { ReactNode } from "react"
import { Icon, Text, useTheme, type IconName } from "@/lib/flint"

/** The one empty state: a mark, a short title, one line, an optional action. */
export function EmptyState({
  icon = "info",
  title,
  description,
  action,
  compact = false,
}: {
  icon?: IconName
  title: string
  description?: string
  action?: ReactNode
  compact?: boolean
}) {
  const theme = useTheme()
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        gap: compact ? 6 : 9,
        padding: compact ? 18 : 28,
        borderRadius: 18,
        borderWidth: 1,
        borderStyle: "dashed",
        borderColor: theme.colors.border,
      }}
    >
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          width: compact ? 34 : 42,
          height: compact ? 34 : 42,
          borderRadius: 14,
          backgroundColor: theme.colors.accent,
        }}
      >
        <Icon name={icon} size={compact ? 15 : 18} color={theme.colors.mutedForeground} />
      </div>
      <Text size={compact ? "xs" : "sm"} weight="medium" style={{ textAlign: "center" }}>
        {title}
      </Text>
      {description ? (
        <Text size="xs" tone="muted" style={{ textAlign: "center", maxWidth: 420 }}>
          {description}
        </Text>
      ) : null}
      {action}
    </div>
  )
}

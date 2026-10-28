import type { ReactNode } from "react"
import { Icon, Text, useTheme, type IconName } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import type { Snapshot } from "./api"

export type Page = "home" | "instances" | "settings"

const NAV: { id: Page; label: string; icon: IconName }[] = [
  { id: "home", label: "Home", icon: "sparkle" },
  { id: "instances", label: "Instances", icon: "users" },
  { id: "settings", label: "Settings", icon: "panelLeft" },
]

const TITLES: Record<Page, string> = {
  home: "Ready to play",
  instances: "Instances",
  settings: "Settings",
}

export interface ShellProps {
  page: Page
  onPage: (page: Page) => void
  dark: boolean
  onToggleTheme: () => void
  data: Snapshot
  children?: ReactNode
}

/** The frame: a quiet sidebar, a header that says where you are, and the page. */
export function Shell({ page, onPage, dark, onToggleTheme, data, children }: ShellProps) {
  const theme = useTheme()

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        width: "100%",
        height: "100%",
        backgroundColor: theme.colors.background,
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          width: 216,
          flexShrink: 0,
          gap: 6,
          paddingLeft: 14,
          paddingRight: 14,
          paddingTop: 18,
          paddingBottom: 16,
          borderRightWidth: 1,
          borderStyle: "solid",
          borderColor: theme.colors.border,
          backgroundColor: theme.colors.card,
        }}
      >
        <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8, paddingLeft: 8, paddingBottom: 14 }}>
          <Icon name="sparkle" size={16} color={theme.colors.primary} />
          <Text size="lg" weight="semibold">Scoria</Text>
        </div>

        {NAV.map((item) => (
          <Button
            key={item.id}
            variant={page === item.id ? "secondary" : "ghost"}
            style={{ width: "100%", justifyContent: "flex-start" }}
            onClick={() => onPage(item.id)}
          >
            <Icon name={item.icon} size={15} color={page === item.id ? theme.colors.foreground : theme.colors.mutedForeground} />
            <Text tone={page === item.id ? "default" : "muted"}>{item.label}</Text>
          </Button>
        ))}

        <div style={{ flexGrow: 1 }} />

        <div
          style={{
            display: "flex",
            flexDirection: "column",
            gap: 6,
            padding: 10,
            borderRadius: theme.radius.md,
            borderWidth: 1,
            borderStyle: "solid",
            borderColor: theme.colors.border,
            backgroundColor: theme.colors.muted,
          }}
        >
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 6 }}>
            <Badge variant="success" dot>Offline</Badge>
            <Text size="sm" tone="muted">{data.offlineName}</Text>
          </div>
          <Text size="xs" tone="muted">No Microsoft account needed</Text>
        </div>

        <Text size="xs" tone="muted" style={{ paddingLeft: 8, paddingTop: 4 }}>
          {data.version} · {data.branch}
        </Text>
      </div>

      <div style={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0 }}>
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
            gap: 10,
            height: 56,
            flexShrink: 0,
            paddingLeft: 24,
            paddingRight: 20,
            borderBottomWidth: 1,
            borderColor: theme.colors.border,
          }}
        >
          <Text size="md" weight="semibold">{TITLES[page]}</Text>
          <div style={{ flexGrow: 1 }} />
          <Badge variant={data.live ? "success" : "secondary"} dot>
            {data.live ? "engine" : "preview"}
          </Badge>
          <Badge variant="outline">{data.branch}</Badge>
          <Button variant="ghost" size="icon" onClick={onToggleTheme} aria-label="Toggle theme">
            <Icon name={dark ? "sun" : "moon"} size={16} color={theme.colors.mutedForeground} />
          </Button>
        </div>

        <div
          style={{
            display: "flex",
            flexDirection: "column",
            gap: 14,
            flexGrow: 1,
            minHeight: 0,
            padding: 24,
          }}
        >
          {children}
        </div>
      </div>
    </div>
  )
}

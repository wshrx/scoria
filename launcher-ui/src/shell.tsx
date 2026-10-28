import type { ReactNode } from "react"
import { Icon, Text, useTheme, type IconName } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import type { Snapshot } from "./api"

export type Page = "home" | "instances" | "settings"

const NAV: { id: Page; label: string; icon: IconName }[] = [
  { id: "home", label: "Home", icon: "home" },
  { id: "instances", label: "Instances", icon: "layers" },
  { id: "settings", label: "Settings", icon: "sliders" },
]

const TITLES: Record<Page, string> = {
  home: "Overview",
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

/** A rail, a thin header and a page. The rail carries icons, not labels. */
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
          alignItems: "center",
          width: 64,
          flexShrink: 0,
          gap: 6,
          paddingTop: 14,
          paddingBottom: 14,
          borderRightWidth: 1,
          borderStyle: "solid",
          borderColor: theme.colors.border,
          backgroundColor: theme.colors.card,
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            width: 36,
            height: 36,
            marginBottom: 14,
            backgroundColor: theme.colors.primary,
          }}
        >
          <Text
            style={{
              color: theme.colors.primaryForeground,
              fontFamily: theme.font.mono,
              fontSize: 17,
              fontWeight: "700",
            }}
          >
            S
          </Text>
        </div>

        {NAV.map((item) => {
          const active = page === item.id
          return (
            <Button
              key={item.id}
              variant="ghost"
              size="icon"
              aria-label={item.label}
              style={{
                width: 40,
                height: 40,
                borderRadius: theme.radius.sm,
                backgroundColor: active ? theme.colors.accent : "transparent",
              }}
              onClick={() => onPage(item.id)}
            >
              <Icon
                name={item.icon}
                size={17}
                color={active ? theme.colors.primary : theme.colors.mutedForeground}
              />
            </Button>
          )
        })}

        <div style={{ flexGrow: 1 }} />

        <Button
          variant="ghost"
          size="icon"
          aria-label="Toggle theme"
          style={{ width: 40, height: 40 }}
          onClick={onToggleTheme}
        >
          <Icon name={dark ? "sun" : "moon"} size={16} color={theme.colors.mutedForeground} />
        </Button>
      </div>

      <div style={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0 }}>
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
            gap: 10,
            height: 50,
            flexShrink: 0,
            paddingLeft: 28,
            paddingRight: 22,
            borderBottomWidth: 1,
            borderStyle: "solid",
            borderColor: theme.colors.border,
          }}
        >
          <Text
            mono
            size="xs"
            tone="muted"
            style={{ fontFamily: theme.font.mono }}
          >
            SCORIA / {TITLES[page].toUpperCase()}
          </Text>
          <div style={{ flexGrow: 1 }} />
          <Badge variant={data.live ? "success" : "secondary"} dot>
            {data.live ? "engine" : "preview"}
          </Badge>
          <Text mono size="xs" tone="muted">{data.version}</Text>
        </div>

        <div style={{ flexGrow: 1, minHeight: 0, overflow: "scroll" }}>
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              gap: 22,
              width: "100%",
              maxWidth: 1120,
              paddingLeft: 36,
              paddingRight: 36,
              paddingTop: 30,
              paddingBottom: 36,
            }}
          >
            {children}
          </div>
        </div>
      </div>
    </div>
  )
}

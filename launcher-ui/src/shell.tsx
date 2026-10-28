import type { ReactNode } from "react"
import { Icon, Text, useTheme, type IconName } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import type { Snapshot } from "./api"
import { useT } from "./i18n"
import { gradient } from "./visual"

export type Page = "home" | "instances" | "servers" | "settings"

const NAV: { id: Page; label: string; icon: IconName }[] = [
  { id: "home", label: "Home", icon: "home" },
  { id: "instances", label: "Instances", icon: "layers" },
  { id: "servers", label: "Servers", icon: "server" },
  { id: "settings", label: "Settings", icon: "sliders" },
]

const TITLES: Record<Page, string> = {
  home: "Overview",
  instances: "Instances",
  servers: "Servers",
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

/** A floating rail, a hairline header, and a page on a warm basalt field. */
export function Shell({ page, onPage, dark, onToggleTheme, data, children }: ShellProps) {
  const theme = useTheme()
  const t = useT()

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        width: "100%",
        height: "100%",
        background: gradient("#0C0908", "#060504", 160),
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          width: 76,
          flexShrink: 0,
          gap: 8,
          paddingTop: 18,
          paddingBottom: 16,
          borderRightWidth: 1,
          borderStyle: "solid",
          borderColor: "#FFFFFF0D",
          background: gradient("#100C0B", "#090706", 180),
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            width: 42,
            height: 42,
            marginBottom: 16,
            borderRadius: 14,
            background: gradient("#FF7A45", "#B33A12", 150),
            boxShadow: { offsetX: 0, offsetY: 10, blurRadius: 26, spreadRadius: -6, color: "#FF6B3566" },
          }}
        >
          <Text
            style={{
              color: theme.colors.primaryForeground,
              fontFamily: theme.font.mono,
              fontSize: 19,
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
              aria-label={t(item.label)}
              style={{
                width: 44,
                height: 44,
                borderRadius: 13,
                backgroundColor: active ? theme.colors.primary : "transparent",
                boxShadow: active
                  ? { offsetX: 0, offsetY: 10, blurRadius: 24, spreadRadius: -8, color: "#FF6B3577" }
                  : undefined,
              }}
              onClick={() => onPage(item.id)}
            >
              <Icon
                name={item.icon}
                size={18}
                color={active ? theme.colors.primaryForeground : theme.colors.mutedForeground}
              />
            </Button>
          )
        })}

        <div style={{ flexGrow: 1 }} />

        <Button
          variant="ghost"
          size="icon"
          aria-label={t("Toggle theme")}
          style={{ width: 44, height: 44, borderRadius: 13 }}
          onClick={onToggleTheme}
        >
          <Icon name={dark ? "sun" : "moon"} size={17} color={theme.colors.mutedForeground} />
        </Button>
      </div>

      <div style={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0 }}>
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
            gap: 10,
            height: 52,
            flexShrink: 0,
            paddingLeft: 32,
            paddingRight: 24,
            borderBottomWidth: 1,
            borderStyle: "solid",
            borderColor: "#FFFFFF0A",
          }}
        >
          <Text mono size="xs" tone="muted">
            SCORIA / <Text mono size="xs" style={{ color: theme.colors.foreground }}>{t(TITLES[page]).toUpperCase()}</Text>
          </Text>
          <div style={{ flexGrow: 1 }} />
          <Badge variant={data.live ? "success" : "secondary"} dot>
            {t(data.live ? "engine" : "preview")}
          </Badge>
          <Text mono size="xs" tone="muted">{data.version}</Text>
        </div>

        <div style={{ flexGrow: 1, minHeight: 0, overflow: "scroll" }}>
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              gap: 24,
              width: "100%",
              maxWidth: 1240,
              paddingLeft: 32,
              paddingRight: 32,
              paddingTop: 28,
              paddingBottom: 40,
            }}
          >
            {children}
          </div>
        </div>
      </div>
    </div>
  )
}

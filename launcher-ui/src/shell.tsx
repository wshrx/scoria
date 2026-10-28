import type { ReactNode } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme, type IconName } from "@/lib/flint"
import type { Snapshot } from "./api"
import { useT } from "./i18n"
import { Mark } from "./visual"

export type Page = "home" | "instances" | "files" | "servers" | "settings"

const NAV: { id: Page; label: string; icon: IconName }[] = [
  { id: "home", label: "Play", icon: "home" },
  { id: "instances", label: "Instances", icon: "layers" },
  { id: "files", label: "Game files", icon: "hardDrive" },
  { id: "servers", label: "Servers", icon: "server" },
  { id: "settings", label: "Settings", icon: "sliders" },
]

export interface ShellProps {
  page: Page
  onPage: (page: Page) => void
  dark: boolean
  onToggleTheme: () => void
  data: Snapshot
  children?: ReactNode
}

/** A masthead, a wide sheet of content, and a hairline that keeps the window honest. */
export function Shell({ page, onPage, dark, onToggleTheme, data, children }: ShellProps) {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const compact = width < 1080

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        width: "100%",
        height: "100%",
        backgroundColor: theme.colors.background,
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          alignItems: "stretch",
          height: 66,
          flexShrink: 0,
          paddingLeft: compact ? 20 : 32,
          paddingRight: compact ? 16 : 28,
          gap: compact ? 16 : 30,
          borderBottomWidth: 1,
          borderStyle: "solid",
          borderColor: theme.colors.border,
          backgroundColor: theme.colors.background,
        }}
      >
        <div
          style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 11, cursor: "pointer" }}
          onClick={() => onPage("home")}
        >
          <Mark size={24} />
          <Text
            style={{
              fontSize: 16,
              lineHeight: 20,
              fontWeight: "700",
              color: theme.colors.foreground,
            }}
          >
            Scoria
          </Text>
          {compact ? null : (
            <Text mono size="xs" tone="muted" style={{ marginLeft: 2 }}>
              {data.minecraft.version}
            </Text>
          )}
        </div>

        <div style={{ display: "flex", flexDirection: "row", alignItems: "stretch", gap: 2, flexGrow: 1 }}>
          {NAV.map((item) => {
            const active = page === item.id
            return (
              <div
                key={item.id}
                role="tab"
                tabIndex={0}
                aria-selected={active}
                onClick={() => onPage(item.id)}
                style={{
                  display: "flex",
                  flexDirection: "row",
                  alignItems: "center",
                  gap: 7,
                  height: 66,
                  paddingLeft: compact ? 10 : 13,
                  paddingRight: compact ? 10 : 13,
                  borderBottomWidth: 2,
                  borderStyle: "solid",
                  borderColor: active ? theme.colors.primary : "transparent",
                  cursor: "pointer",
                  hover: { backgroundColor: theme.colors.accent },
                }}
              >
                <Icon
                  name={item.icon}
                  size={14}
                  color={active ? theme.colors.primary : theme.colors.mutedForeground}
                />
                {compact ? null : (
                  <Text
                    size="sm"
                    weight={active ? "semibold" : "regular"}
                    style={{ color: active ? theme.colors.foreground : theme.colors.mutedForeground }}
                  >
                    {t(item.label)}
                  </Text>
                )}
              </div>
            )
          })}
        </div>

        <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 14 }}>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 6 }}>
            <div
              style={{
                width: 6,
                height: 6,
                borderRadius: 3,
                backgroundColor: data.live ? theme.colors.success : theme.colors.mutedForeground,
                boxShadow: data.live
                  ? { offsetX: 0, offsetY: 0, blurRadius: 8, spreadRadius: 1, color: "#8ED9B266" }
                  : undefined,
              }}
            />
            <Text mono size="xs" tone="muted">
              {t(data.live ? "engine" : "preview")}
            </Text>
          </div>

          {compact ? null : (
            <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
              <Text size="sm" style={{ color: theme.colors.foreground }}>
                {data.offlineName}
              </Text>
              <Text mono size="xs" tone="muted">
                {t("offline")}
              </Text>
            </div>
          )}

          <div
            role="button"
            tabIndex={0}
            aria-label={t("Toggle theme")}
            onClick={onToggleTheme}
            style={{
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              width: 30,
              height: 30,
              borderRadius: theme.radius.sm,
              borderWidth: 1,
              borderStyle: "solid",
              borderColor: theme.colors.border,
              cursor: "pointer",
              hover: { backgroundColor: theme.colors.accent },
            }}
          >
            <Icon name={dark ? "sun" : "moon"} size={14} color={theme.colors.mutedForeground} />
          </div>
        </div>
      </div>

      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          flexGrow: 1,
          minHeight: 0,
          overflow: "scroll",
          scrollbarColor: theme.colors.border,
          scrollbarWidth: 8,
        }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            gap: 30,
            width: "100%",
            maxWidth: 1440,
            paddingLeft: compact ? 20 : 40,
            paddingRight: compact ? 20 : 40,
            paddingTop: 30,
            paddingBottom: 48,
          }}
        >
          {children}
        </div>
      </div>
    </div>
  )
}

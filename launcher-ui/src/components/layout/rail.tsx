import { Tooltip, TooltipContent, TooltipTrigger, useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme, type IconName } from "@/lib/flint"
import type { Snapshot } from "../../api"
import { useT } from "../../i18n"
import { SCREENS, type ScreenId } from "../../screens/registry"
import { LAUNCHER_CHANNEL, LAUNCHER_VERSION } from "../../version"
import { Avatar, Mark, gradient } from "../../visual"

/** The labelled sidebar: the only page navigation in the launcher. */
export function Rail({
  page,
  onPage,
  data,
}: {
  page: ScreenId
  onPage: (page: ScreenId) => void
  data: Snapshot
}) {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const compact = width < 1100

  const item = (id: ScreenId, icon: IconName, label: string) => {
    const active = page === id
    const row = (
      <div
        key={id}
        role="button"
        tabIndex={0}
        aria-label={t(label)}
        aria-selected={active}
        onClick={() => onPage(id)}
        onKeyDown={(event) => {
          if (event.key === "enter" || event.key === " " || event.key === "space") {
            event.preventDefault()
            onPage(id)
          }
        }}
        style={{
          display: "flex",
          flexDirection: "row",
          alignItems: "center",
          gap: 10,
          width: "100%",
          height: 38,
          paddingLeft: 11,
          paddingRight: 10,
          flexShrink: 0,
          borderRadius: 10,
          background: active ? gradient("#FF8A4C24", "#FF8A4C0A", 120) : undefined,
          cursor: "pointer",
          hover: active ? {} : { backgroundColor: theme.colors.accent },
          focusVisible: { outlineWidth: 2, outlineColor: theme.colors.ring, outlineOffset: -2 },
        }}
      >
        <Icon
          name={icon}
          size={16}
          color={active ? theme.colors.primary : theme.colors.mutedForeground}
        />
        {compact ? null : (
          <Text
            size="xs"
            weight={active ? "semibold" : "regular"}
            style={{ color: active ? theme.colors.foreground : theme.colors.mutedForeground }}
          >
            {t(label)}
          </Text>
        )}
      </div>
    )
    if (!compact) return row
    return (
      <Tooltip key={id}>
        <TooltipTrigger asChild>{row}</TooltipTrigger>
        <TooltipContent
          side="right"
          sideOffset={8}
          style={{
            paddingLeft: 8,
            paddingRight: 8,
            paddingTop: 5,
            paddingBottom: 5,
            borderRadius: 8,
            borderWidth: 1,
            borderStyle: "solid",
            borderColor: theme.colors.border,
            backgroundColor: theme.colors.popover,
          }}
        >
          <Text size="xs">{t(label)}</Text>
        </TooltipContent>
      </Tooltip>
    )
  }

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: compact ? "center" : "stretch",
        width: compact ? 64 : 220,
        flexShrink: 0,
        gap: 6,
        paddingTop: 16,
        paddingLeft: compact ? 8 : 14,
        paddingRight: compact ? 8 : 14,
        paddingBottom: 14,
        backgroundColor: theme.appearance === "dark" ? "#0B0D12" : theme.colors.card,
        borderRightWidth: 1,
        borderStyle: "solid",
        borderColor: theme.colors.border,
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          alignItems: "center",
          gap: 10,
          paddingLeft: compact ? 0 : 4,
          marginBottom: 16,
          cursor: "pointer",
        }}
        onClick={() => onPage("home")}
      >
        <Mark size={30} />
        {compact ? null : (
          <div style={{ display: "flex", flexDirection: "column", gap: 1 }}>
            <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 6 }}>
              <Text size="sm" weight="semibold">
                Scoria
              </Text>
              <Text
                mono
                size="xs"
                style={{
                  color: theme.colors.primary,
                  borderWidth: 1,
                  borderStyle: "solid",
                  borderColor: theme.colors.primary + "55",
                  borderRadius: 999,
                  paddingLeft: 6,
                  paddingRight: 6,
                  paddingTop: 1,
                  paddingBottom: 1,
                }}
              >
                {LAUNCHER_CHANNEL}
              </Text>
            </div>
            <Text mono size="xs" tone="muted">
              {LAUNCHER_VERSION}
            </Text>
          </div>
        )}
      </div>

      <div style={{ display: "flex", flexDirection: "column", gap: 3 }}>
        {SCREENS.filter((screen) => screen.rail === "main").map((screen) =>
          item(screen.id, screen.icon, screen.label),
        )}
      </div>

      <div style={{ flexGrow: 1 }} />

      <div style={{ display: "flex", flexDirection: "column", gap: 3 }}>
        {SCREENS.filter((screen) => screen.rail === "footer").map((screen) =>
          item(screen.id, screen.icon, screen.label),
        )}
      </div>

      <div
        style={{
          display: "flex",
          flexDirection: "row",
          alignItems: "center",
          gap: 9,
          marginTop: 10,
          paddingLeft: compact ? 0 : 4,
          paddingTop: 10,
          borderTopWidth: 1,
          borderStyle: "solid",
          borderColor: theme.colors.border,
        }}
      >
        <Avatar name={data.offlineName} size={28} />
        {compact ? null : (
          <div style={{ display: "flex", flexDirection: "column", gap: 1, minWidth: 0 }}>
            <Text size="xs" weight="medium" truncate>
              {data.offlineName}
            </Text>
            <Text mono size="xs" tone="muted">
              {t("offline")}
            </Text>
          </div>
        )}
      </div>
    </div>
  )
}

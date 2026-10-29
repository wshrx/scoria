import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme, type IconName } from "@/lib/flint"
import type { Snapshot } from "../../api"
import { useT } from "../../i18n"
import { SCREENS, type ScreenId } from "../../screens/registry"
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
    return (
      <div
        key={id}
        role="button"
        tabIndex={0}
        aria-label={t(label)}
        aria-selected={active}
        onClick={() => onPage(id)}
        style={{
          display: "flex",
          flexDirection: "row",
          alignItems: "center",
          gap: 10,
          height: 38,
          paddingLeft: 11,
          paddingRight: 10,
          flexShrink: 0,
          borderRadius: 10,
          background: active ? gradient("#8B5CF624", "#8B5CF60A", 120) : undefined,
          cursor: "pointer",
          hover: active ? {} : { backgroundColor: theme.colors.accent },
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
          <Text size="sm" weight="semibold">
            Scoria
          </Text>
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

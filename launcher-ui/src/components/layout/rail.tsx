import { Icon, useTheme, type IconName } from "@/lib/flint"
import { useT } from "../../i18n"
import { SCREENS, type ScreenId } from "../../screens/registry"
import { Mark, gradient } from "../../visual"

/** The icon rail: the only page navigation in the launcher. */
export function Rail({ page, onPage }: { page: ScreenId; onPage: (page: ScreenId) => void }) {
  const theme = useTheme()
  const t = useT()

  const item = (id: ScreenId, icon: IconName, label: string) => {
    const active = page === id
    return (
      <div
        key={id}
        role="button"
        tabIndex={0}
        aria-label={t(label)}
        onClick={() => onPage(id)}
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          width: 44,
          height: 44,
          flexShrink: 0,
          borderRadius: 14,
          background: active ? gradient("#A78BFA", "#6D28D9", 150) : undefined,
          backgroundColor: active ? theme.colors.primary : "transparent",
          cursor: "pointer",
          hover: active ? {} : { backgroundColor: theme.colors.accent },
        }}
      >
        <Icon name={icon} size={18} color={active ? "#FFFFFF" : theme.colors.mutedForeground} />
      </div>
    )
  }

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        width: 74,
        flexShrink: 0,
        gap: 8,
        paddingTop: 16,
        paddingBottom: 14,
        backgroundColor: theme.appearance === "dark" ? "#0B0713" : theme.colors.card,
        borderRightWidth: 1,
        borderStyle: "solid",
        borderColor: theme.colors.border,
      }}
    >
      <div
        role="button"
        tabIndex={0}
        aria-label={t("Play")}
        onClick={() => onPage("home")}
        style={{ cursor: "pointer", marginBottom: 14 }}
      >
        <Mark size={38} />
      </div>

      {SCREENS.filter((screen) => screen.rail === "main").map((screen) =>
        item(screen.id, screen.icon, screen.label),
      )}

      <div style={{ flexGrow: 1 }} />

      {SCREENS.filter((screen) => screen.rail === "footer").map((screen) =>
        item(screen.id, screen.icon, screen.label),
      )}
    </div>
  )
}

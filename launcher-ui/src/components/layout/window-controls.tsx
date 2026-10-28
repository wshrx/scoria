import { useGpuixRequired } from "@gpuix/react"
import { Icon, useTheme, type IconName } from "@/lib/flint"
import { useT } from "../../i18n"

function WindowButton({
  icon,
  label,
  danger = false,
  onClick,
}: {
  icon: IconName
  label: string
  danger?: boolean
  onClick: () => void
}) {
  const theme = useTheme()
  return (
    <div
      role="button"
      tabIndex={0}
      aria-label={label}
      onClick={onClick}
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        width: 30,
        height: 30,
        borderRadius: 10,
        cursor: "pointer",
        color: theme.colors.mutedForeground,
        hover: {
          backgroundColor: danger ? theme.colors.destructive : theme.colors.accent,
          color: danger ? theme.colors.destructiveForeground : theme.colors.foreground,
        },
      }}
    >
      <Icon name={icon} size={14} color={theme.colors.mutedForeground} />
    </div>
  )
}

/** Minimize, maximize and close, for the frameless window. */
export function WindowControls() {
  const renderer = useGpuixRequired()
  const t = useT()
  return (
    <div style={{ display: "flex", flexDirection: "row", gap: 2, marginLeft: 2 }}>
      <WindowButton icon="minus" label={t("Minimize")} onClick={() => renderer.minimizeWindow?.()} />
      <WindowButton icon="square" label={t("Maximize")} onClick={() => renderer.zoomWindow?.()} />
      <WindowButton icon="x" label={t("Close")} danger onClick={() => process.exit(0)} />
    </div>
  )
}

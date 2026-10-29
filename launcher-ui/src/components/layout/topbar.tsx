import { useWindowSize } from "@gpuix/react"
import { Text, useTheme } from "@/lib/flint"
import type { Snapshot } from "../../api"
import { useT } from "../../i18n"
import { startWindowMove } from "../../chrome"
import { WindowControls } from "./window-controls"

/** Engine status, a drag surface and the window controls. */
export function Topbar({ data, frameless }: { data: Snapshot; frameless: boolean }) {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const compact = width < 1100
  const files = data.minecraft
  const ready = files.installed && data.prepared
  const status = ready ? t("ready") : !files.installed ? t("needs game files") : t("needs preparing")

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        height: 52,
        flexShrink: 0,
        gap: 12,
        paddingLeft: compact ? 16 : 24,
        paddingRight: 12,
        borderBottomWidth: 1,
        borderStyle: "solid",
        borderColor: theme.colors.border,
      }}
    >
      <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 7,
            height: 28,
            paddingLeft: 10,
            paddingRight: 10,
            borderRadius: 999,
            borderWidth: 1,
            borderStyle: "solid",
            borderColor: theme.colors.border,
            backgroundColor: theme.colors.card,
          }}
        >
          <div
            style={{
              width: 6,
              height: 6,
              borderRadius: 3,
              backgroundColor: ready ? theme.colors.success : "#D8B26A",
            }}
          />
          <Text size="xs" style={{ color: theme.colors.foreground }}>
            {status}
          </Text>
          <Text mono size="xs" tone="muted">
            {files.version}
          </Text>
        </div>
      </div>

      <div
        onMouseDown={frameless ? () => startWindowMove() : undefined}
        style={{
          flexGrow: 1,
          alignSelf: "stretch",
          pointerEvents: frameless ? "auto" : "none",
          cursor: "default",
        }}
      />

      {frameless ? <WindowControls /> : null}
    </div>
  )
}

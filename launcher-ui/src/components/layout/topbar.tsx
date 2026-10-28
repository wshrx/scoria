import { useWindowSize } from "@gpuix/react"
import { Text, useTheme } from "@/lib/flint"
import type { Snapshot } from "../../api"
import { useT } from "../../i18n"
import { startWindowMove } from "../../chrome"
import { Avatar } from "../../visual"
import { WindowControls } from "./window-controls"

/** Status, identity and the window controls over the scene. */
export function Topbar({ data, frameless }: { data: Snapshot; frameless: boolean }) {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const compact = width < 1000
  const files = data.minecraft
  const ready = files.installed && data.prepared
  const status = ready ? t("ready") : !files.installed ? t("needs game files") : t("needs preparing")

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        height: 58,
        flexShrink: 0,
        gap: 12,
        paddingLeft: compact ? 18 : 26,
        paddingRight: 12,
        position: "relative",
      }}
    >
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: 7,
          height: 30,
          paddingLeft: 12,
          paddingRight: 12,
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

      <div
        onMouseDown={frameless ? () => startWindowMove() : undefined}
        style={{
          flexGrow: 1,
          alignSelf: "stretch",
          pointerEvents: frameless ? "auto" : "none",
          cursor: "default",
        }}
      />

      <div
        aria-label={`${data.offlineName} · ${t("offline")}`}
        style={{
          display: "flex",
          flexDirection: "row",
          alignItems: "center",
          gap: 8,
          height: 34,
          paddingLeft: 5,
          paddingRight: 10,
        }}
      >
        <Avatar name={data.offlineName} size={26} />
        {compact ? null : (
          <div style={{ display: "flex", flexDirection: "column" }}>
            <Text size="xs" weight="medium">
              {data.offlineName}
            </Text>
            <Text mono size="xs" tone="muted">
              {t("offline")}
            </Text>
          </div>
        )}
      </div>

      {frameless ? <WindowControls /> : null}
    </div>
  )
}

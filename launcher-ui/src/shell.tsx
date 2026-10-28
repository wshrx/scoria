import { useEffect, type ReactNode } from "react"
import { motion, useGpuixRequired, useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme, type IconName } from "@/lib/flint"
import type { Snapshot } from "./api"
import { useT } from "./i18n"
import { Avatar, Backdrop, BONE, CHROME_MUTED, Mark, gradient } from "./visual"
import { shapeWindow, startWindowMove, startWindowResize } from "./chrome"

export type Page = "home" | "instances" | "servers" | "settings"

const NAV: { id: Page; label: string; icon: IconName }[] = [
  { id: "home", label: "Play", icon: "play" },
  { id: "instances", label: "Instances", icon: "layers" },
  { id: "servers", label: "Servers", icon: "server" },
]

export interface ShellProps {
  page: Page
  onPage: (page: Page) => void
  data: Snapshot
  frameless: boolean
  children?: ReactNode
}

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
        color: CHROME_MUTED,
        hover: {
          backgroundColor: danger ? theme.colors.destructive : "#FFFFFF14",
          color: danger ? theme.colors.destructiveForeground : BONE,
        },
      }}
    >
      <Icon name={icon} size={14} color={CHROME_MUTED} />
    </div>
  )
}

function Rail({ page, onPage }: { page: Page; onPage: (page: Page) => void }) {
  const theme = useTheme()
  const t = useT()

  const item = (id: Page, icon: IconName, label: string) => {
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
          hover: active ? {} : { backgroundColor: "#1A1428" },
        }}
      >
        <Icon
          name={icon}
          size={18}
          color={active ? "#FFFFFF" : CHROME_MUTED}
        />
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
        backgroundColor: "#0B0713",
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

      {NAV.map((entry) => item(entry.id, entry.icon, entry.label))}

      <div style={{ flexGrow: 1 }} />

      {item("settings", "sliders", "Settings")}
    </div>
  )
}

function ResizeHandles() {
  const zones: { style: Record<string, unknown>; direction: number; cursor: string }[] = [
    { style: { top: 0, left: 18, right: 18, height: 5 }, direction: 1, cursor: "ns-resize" },
    { style: { bottom: 0, left: 18, right: 18, height: 5 }, direction: 5, cursor: "ns-resize" },
    { style: { left: 0, top: 18, bottom: 18, width: 5 }, direction: 7, cursor: "ew-resize" },
    { style: { right: 0, top: 18, bottom: 18, width: 5 }, direction: 3, cursor: "ew-resize" },
    { style: { top: 0, left: 0, width: 16, height: 16 }, direction: 0, cursor: "nwse-resize" },
    { style: { top: 0, right: 0, width: 16, height: 16 }, direction: 2, cursor: "nesw-resize" },
    { style: { bottom: 0, right: 0, width: 16, height: 16 }, direction: 4, cursor: "nwse-resize" },
    { style: { bottom: 0, left: 0, width: 16, height: 16 }, direction: 6, cursor: "nesw-resize" },
  ]
  return (
    <>
      {zones.map((zone) => (
        <div
          key={zone.direction}
          onMouseDown={() => startWindowResize(zone.direction)}
          style={{
            position: "absolute",
            pointerEvents: "auto",
            cursor: zone.cursor as never,
            ...zone.style,
          }}
        />
      ))}
    </>
  )
}

/** The rail, a floating top strip, and a wide stage for one page. */
export function Shell({ page, onPage, data, frameless, children }: ShellProps) {
  const theme = useTheme()
  const t = useT()
  const { width, height } = useWindowSize()
  const renderer = useGpuixRequired()
  const compact = width < 1000

  useEffect(() => {
    if (frameless) void shapeWindow(width, height, 10, 24)
  }, [frameless, width, height])
  const files = data.minecraft
  const ready = files.installed && data.prepared

  const status = ready
    ? t("ready")
    : !files.installed
      ? t("needs game files")
      : t("needs preparing")

  const close = () => process.exit(0)

  return (
    <div
      style={{
        display: "flex",
        width: "100%",
        height: "100%",
        padding: frameless ? 10 : 0,
        backgroundColor: frameless ? "transparent" : theme.colors.background,
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          width: "100%",
          height: "100%",
          overflow: "hidden",
          borderRadius: frameless ? 24 : 0,
          borderWidth: frameless ? 1 : 0,
          borderStyle: "solid",
          borderColor: theme.colors.border,
          backgroundColor: theme.colors.background,
          position: "relative",
        }}
      >
        <Rail page={page} onPage={onPage} />

        <div
          style={{
            display: "flex",
            flexDirection: "column",
            flexGrow: 1,
            minWidth: 0,
            position: "relative",
          }}
        >
          <Backdrop strong={page === "home"} />

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
            <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
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
                  borderColor: "#FFFFFF14",
                  backgroundColor: "#FFFFFF0D",
                }}
              >
                <div
                  style={{
                    width: 6,
                    height: 6,
                    borderRadius: 3,
                    backgroundColor: ready ? "#7FD8A4" : "#D8B26A",
                  }}
                />
                <Text size="xs" style={{ color: BONE }}>
                  {status}
                </Text>
                <Text mono size="xs" style={{ color: CHROME_MUTED }}>
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
                  <Text size="xs" weight="medium" style={{ color: BONE }}>
                    {data.offlineName}
                  </Text>
                  <Text mono size="xs" style={{ color: CHROME_MUTED }}>
                    {t("offline")}
                  </Text>
                </div>
              )}
            </div>

            {frameless ? (
              <div style={{ display: "flex", flexDirection: "row", gap: 2, marginLeft: 2 }}>
                <WindowButton icon="minus" label={t("Minimize")} onClick={() => renderer.minimizeWindow?.()} />
                <WindowButton icon="square" label={t("Maximize")} onClick={() => renderer.zoomWindow?.()} />
                <WindowButton icon="x" label={t("Close")} danger onClick={close} />
              </div>
            ) : null}
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
              position: "relative",
            }}
          >
            <motion.div
              key={page}
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.18, ease: "easeOut" }}
              style={{
                display: "flex",
                flexDirection: "column",
                gap: 24,
                width: "100%",
                maxWidth: 1360,
                paddingLeft: compact ? 18 : 30,
                paddingRight: compact ? 18 : 30,
                paddingTop: 4,
                paddingBottom: 32,
                flexGrow: 1,
              }}
            >
              {children}
            </motion.div>
          </div>
        </div>

        {frameless ? <ResizeHandles /> : null}
      </div>
    </div>
  )
}

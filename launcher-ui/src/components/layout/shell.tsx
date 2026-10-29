import { useEffect, type ReactNode } from "react"
import { motion, useWindowSize } from "@gpuix/react"
import { useTheme } from "@/lib/flint"
import type { Snapshot } from "../../api"
import { shapeWindow } from "../../chrome"
import type { ScreenId } from "../../screens/registry"
import { Backdrop, ParticleField } from "../../visual"
import { Rail } from "./rail"
import { ResizeHandles } from "./resize-handles"
import { Topbar } from "./topbar"

export interface ShellProps {
  page: ScreenId
  onPage: (page: ScreenId) => void
  data: Snapshot
  frameless: boolean
  children?: ReactNode
}

/** The rail, a floating top strip, and a wide stage for one screen. */
export function Shell({ page, onPage, data, frameless, children }: ShellProps) {
  const theme = useTheme()
  const { width, height } = useWindowSize()
  const compact = width < 1000
  const dark = theme.appearance === "dark"

  useEffect(() => {
    if (frameless) void shapeWindow(width, height, 10, 24)
  }, [frameless, width, height])

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
        <Rail page={page} onPage={onPage} data={data} />

        <div
          style={{
            display: "flex",
            flexDirection: "column",
            flexGrow: 1,
            minWidth: 0,
            position: "relative",
          }}
        >
          <Backdrop strong={page === "home"} dark={dark} />
          <ParticleField dark={dark} />

          <Topbar data={data} frameless={frameless} />

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

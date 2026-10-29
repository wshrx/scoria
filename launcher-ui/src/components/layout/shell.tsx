import { type ReactNode } from "react"
import { motion, useWindowSize } from "@gpuix/react"
import { useTheme } from "@/lib/flint"
import type { Snapshot } from "../../api"
import type { ScreenId } from "../../screens/registry"
import { Backdrop, ParticleField } from "../../visual"
import { Rail } from "./rail"

export interface ShellProps {
  page: ScreenId
  onPage: (page: ScreenId) => void
  data: Snapshot
  children?: ReactNode
}

/**
 * The rail and a wide stage for one screen. The window itself is decorated by
 * the Wayland compositor: no in-app titlebar, no X11 shape, no client-side move.
 */
export function Shell({ page, onPage, data, children }: ShellProps) {
  const theme = useTheme()
  const { width, height } = useWindowSize()
  const compact = width < 1000
  const dark = theme.appearance === "dark"

  return (
    <div
      style={{
        display: "flex",
        width: "100%",
        height: "100%",
        backgroundColor: theme.colors.background,
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          width: "100%",
          height: "100%",
          overflow: "hidden",
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
                paddingTop: 18,
                paddingBottom: 32,
                flexGrow: 1,
              }}
            >
              {children}
            </motion.div>
          </div>
        </div>

      </div>
    </div>
  )
}

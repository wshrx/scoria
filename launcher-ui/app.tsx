import { render } from "@gpuix/react"
import { App } from "./src/app"

// SCORIA_UI_HEADLESS lets CI (and an agent) check that the window and the fonts
// come up without showing anything on the user's desktop.
const headless = process.env.SCORIA_UI_HEADLESS === "1"

// The window is 1280x720 and the compositor draws its frame. The launcher is
// Wayland-native on Linux: no X11 backend and no client-side titlebar.
const width = Number(process.env.SCORIA_UI_WIDTH ?? 1280)
const height = Number(process.env.SCORIA_UI_HEIGHT ?? 720)

render(<App />, {
  title: "Scoria",
  appId: "scoria-launcher",
  width,
  height,
  minWidth: 1040,
  minHeight: 660,
  focus: !headless,
  show: !headless,
})

if (headless) setTimeout(() => process.exit(0), Number(process.env.SCORIA_UI_HEADLESS_MS ?? 2000))

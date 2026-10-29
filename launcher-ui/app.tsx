import { createRenderer, render, startFrameLoop } from "@gpuix/react"
import { existsSync, readFileSync } from "node:fs"
import { dirname, resolve } from "node:path"
import { App } from "./src/app"
import { FLINT_FONT_FILES } from "./src/lib/flint/fonts"

// SCORIA_UI_HEADLESS lets CI (and an agent) check that the window and the fonts
// come up without showing anything on the user's desktop.
const headless = process.env.SCORIA_UI_HEADLESS === "1"

// The window is 1280x720 and the compositor draws its frame. The launcher is
// Wayland-native on Linux: no X11 backend and no client-side titlebar.
const width = Number(process.env.SCORIA_UI_WIDTH ?? 1280)
const height = Number(process.env.SCORIA_UI_HEIGHT ?? 720)

const options = {
  title: "Scoria",
  appId: "scoria-launcher",
  width,
  height,
  minWidth: 1040,
  minHeight: 660,
  focus: !headless,
  show: !headless,
}

// Fonts must be registered before the first frame: registering them from a React effect leaves
// the first paint in the system fallback. In a checkout they are in the tree; in a packaged build
// they sit beside the binary.
const fontDir = existsSync("./src/lib/fonts")
  ? "./src/lib/fonts"
  : resolve(dirname(process.execPath), "fonts")
const host = globalThis as typeof globalThis & {
  __scoriaRenderer?: ReturnType<typeof createRenderer>
}
let renderer = host.__scoriaRenderer
if (!renderer) {
  renderer = createRenderer()
  renderer.init(options)
  console.log("[gpuix] created native window")
  // Fonts before the first frame; a React effect would leave the first paint in the fallback.
  for (const file of FLINT_FONT_FILES) {
    try {
      renderer.registerFont(readFileSync(resolve(fontDir, file)))
    } catch (error) {
      console.error(`[scoria] font ${file}: ${String(error)}`)
    }
  }
  host.__scoriaRenderer = renderer
  startFrameLoop(renderer, {
    onTerminated: () => process.exit(0),
  })
}

// `bun --hot` re-evaluates this module; the guard keeps one window and one frame loop.
render(<App />, { renderer })

if (headless) setTimeout(() => process.exit(0), Number(process.env.SCORIA_UI_HEADLESS_MS ?? 2000))

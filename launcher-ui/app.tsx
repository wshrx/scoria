import { render } from "@gpuix/react"
import { App } from "./src/app"

// SCORIA_UI_HEADLESS lets CI (and an agent) check that the window and the fonts
// come up without showing anything on the user's desktop.
const headless = process.env.SCORIA_UI_HEADLESS === "1"

render(<App />, {
  title: "Scoria",
  width: 1160,
  height: 700,
  focus: !headless,
  show: !headless,
})

if (headless) setTimeout(() => process.exit(0), 2000)

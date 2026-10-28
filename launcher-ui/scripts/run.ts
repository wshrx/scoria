/**
 * Start the launcher with the window backend GPUIX needs for a frameless
 * window. The native binding decides Wayland or X11 when it loads, so the
 * variables have to be set before this process imports the app.
 */
const hot = process.argv.includes("--hot")
const env = { ...process.env }
if (process.platform === "linux" && env.DISPLAY && env.SCORIA_UI_WAYLAND !== "1") {
  env.WINIT_UNIX_BACKEND = "x11"
  env.WAYLAND_DISPLAY = ""
}
const child = Bun.spawn(["bun", ...(hot ? ["--hot"] : []), "app.tsx"], {
  env,
  stdio: ["inherit", "inherit", "inherit"],
})
process.exit(await child.exited)

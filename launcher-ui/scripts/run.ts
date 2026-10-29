/** Start the launcher on the session's native backend: Wayland when it is available. */
const hot = process.argv.includes("--hot")
const env = { ...process.env }
const child = Bun.spawn(["bun", ...(hot ? ["--hot"] : []), "app.tsx"], {
  env,
  stdio: ["inherit", "inherit", "inherit"],
})
process.exit(await child.exited)

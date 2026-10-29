import { existsSync, readFileSync, readdirSync, statSync } from "node:fs"
import { join } from "node:path"
import { fileURLToPath } from "node:url"

const root = fileURLToPath(new URL("..", import.meta.url))
const src = join(root, "src")
const issues: string[] = []

function files(dir: string): string[] {
  return readdirSync(dir).flatMap((name) => {
    const path = join(dir, name)
    return statSync(path).isDirectory() ? files(path) : [path]
  })
}

for (const path of files(src)) {
  if (!/\.(ts|tsx)$/.test(path)) continue
  const text = readFileSync(path, "utf8")
  const relative = path.slice(root.length)
  if (/src\/(screens|components\/layout|visual\.tsx)/.test(relative) && text.includes("boxShadow")) {
    issues.push(`${relative}: boxShadow is banned in UI surfaces (black corners on X11)`)
  }
  if (relative.startsWith("src/screens") && text.includes('onPage("settings")')) {
    issues.push(`${relative}: Settings belongs to the rail registry, not another button`)
  }
  if (text.includes("from \"x11\"") || text.includes("WINIT_UNIX_BACKEND")) {
    issues.push(`${relative}: X11 is not supported; the launcher runs on Wayland`)
  }
}

if (existsSync(join(src, "pages"))) issues.push("src/pages exists; screens live in src/screens")
const app = readFileSync(join(root, "app.tsx"), "utf8")
if (!app.includes("?? 1280") || !app.includes("?? 720")) issues.push("app.tsx: default window must stay 1280x720")
if (existsSync(join(src, "chrome.ts"))) issues.push("src/chrome.ts exists; Wayland uses the compositor's decorations")

if (issues.length) {
  console.error(`UI check failed:\n${issues.map((issue) => `  - ${issue}`).join("\n")}`)
  process.exit(1)
}
console.log("UI check passed")

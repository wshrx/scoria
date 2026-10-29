// Every `t("...")` in the UI must have a Russian line in src/i18n.tsx. Missing keys silently
// fall back to English, which is how untranslated screens ship unnoticed. Run from launcher-ui:
//   bun run check:i18n
import { readdirSync, readFileSync, statSync } from "node:fs"
import { join } from "node:path"
import { RU } from "../src/i18n"

const root = join(import.meta.dir, "..", "src")
const files: string[] = []
const walk = (dir: string) => {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name)
    if (statSync(path).isDirectory()) walk(path)
    else if (/\.tsx?$/.test(name)) files.push(path)
  }
}
walk(root)

const used = new Set<string>()
for (const file of files) {
  const text = readFileSync(file, "utf8")
  for (const match of text.matchAll(/\bt\(\s*"((?:[^"\\]|\\.)*)"/g)) used.add(match[1])
}

const missing = [...used].filter((key) => !(key in RU)).sort()
const unused = Object.keys(RU).filter((key) => !used.has(key)).sort()

if (unused.length) console.warn(`unused translations: ${unused.length} (dynamic labels are expected)`)
if (missing.length) {
  console.error(`missing translations (${missing.length}):\n  ${missing.join("\n  ")}`)
  process.exit(1)
}
console.log(`i18n check passed: ${used.size} strings, all translated.`)

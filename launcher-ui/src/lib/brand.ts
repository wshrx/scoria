import { existsSync } from "node:fs"
import { dirname, resolve } from "node:path"

/** The in-app Scoria mark: a checkout asset, else the packaged `brand/` beside the binary. */
export function brandAsset(name: string): string {
  const dev = resolve("./src/assets", name)
  return existsSync(dev) ? dev : resolve(dirname(process.execPath), "brand", name)
}

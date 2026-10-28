/** A short player-facing line for a stopped download; the detail stays in the console. */
export function downloadStopped(error: unknown, t: (text: string) => string): string {
  const text = String(error)
  console.warn(`[scoria] ${text}`)
  if (/network is unreachable|no connection|connection|timed out|dns|resolve/i.test(text)) {
    return t("The connection dropped. Press the button again to continue from where it stopped.")
  }
  return t("The download stopped. Press the button again to continue.")
}

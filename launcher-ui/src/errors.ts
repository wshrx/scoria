/** A short player-facing line for a stopped download; the detail stays in the console. */
export function downloadStopped(error: unknown, t: (text: string) => string): string {
  const text = String(error)
  console.warn(`[scoria] ${text}`)
  if (/network is unreachable|no connection|connection|timed out|dns|resolve/i.test(text)) {
    return t("The connection dropped. Check your network and try again.")
  }
  return t("The download stopped. Press the button again to continue.")
}

/**
 * Engine errors are English and often carry a file path or a URL. Show a short
 * localized line and keep the original in the console for a bug report.
 */
export function friendlyError(error: unknown, t: (text: string) => string): string {
  const text = String(error instanceof Error ? error.message : error)
  console.warn(`[scoria] ${text}`)
  if (/network is unreachable|no connection|connection refused|connection reset|timed out|dns|resolve|offline/i.test(text)) {
    return t("The connection dropped. Check your network and try again.")
  }
  if (/rate limit|too many requests|403/i.test(text)) {
    return t("The service is rate-limiting requests. Try again in a minute.")
  }
  if (/404|not found|no such file/i.test(text)) {
    return t("The file was not found on the server.")
  }
  if (/sha1|checksum|corrupt|damaged/i.test(text)) {
    return t("The download is corrupted. Try again.")
  }
  if (/permission|denied|access is denied|read-only/i.test(text)) {
    return t("No access to the folder. Choose another one.")
  }
  if (/already running|already open/i.test(text)) {
    return t("The game is already running. Close its window to play again.")
  }
  if (/no space|enospc|disk full/i.test(text)) {
    return t("Not enough disk space.")
  }
  return t("Something went wrong. Try again.")
}

/** "6 minutes ago" in the UI's language; `Intl.RelativeTimeFormat` keeps it grammatical. */
export function ago(at: number, lang: "en" | "ru"): string {
  if (!at) return ""
  const seconds = Math.round((at - Date.now()) / 1000)
  const abs = Math.abs(seconds)
  const format = new Intl.RelativeTimeFormat(lang, { numeric: "auto" })
  if (abs < 60) return format.format(seconds, "second")
  if (abs < 3600) return format.format(Math.round(seconds / 60), "minute")
  if (abs < 86400) return format.format(Math.round(seconds / 3600), "hour")
  return format.format(Math.round(seconds / 86400), "day")
}

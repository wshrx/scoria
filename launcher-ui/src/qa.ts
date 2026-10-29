/**
 * Screenshot fixtures for the scoria-qa skill. Nothing here runs in a normal
 * launch: every branch checks `SCORIA_UI_QA` and the value is never set by the
 * app itself. GPUIX ignores synthetic clicks, so state screenshots need a door.
 */
export const QA = process.env.SCORIA_UI_QA ?? ""

export function qaOn(name: string): boolean {
  return QA === name
}

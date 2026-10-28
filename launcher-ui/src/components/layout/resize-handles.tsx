import { startWindowResize } from "../../chrome"

/** Eight invisible edges that hand a resize to the window manager. */
export function ResizeHandles() {
  const zones: { style: Record<string, unknown>; direction: number; cursor: string }[] = [
    { style: { top: 0, left: 18, right: 18, height: 5 }, direction: 1, cursor: "ns-resize" },
    { style: { bottom: 0, left: 18, right: 18, height: 5 }, direction: 5, cursor: "ns-resize" },
    { style: { left: 0, top: 18, bottom: 18, width: 5 }, direction: 7, cursor: "ew-resize" },
    { style: { right: 0, top: 18, bottom: 18, width: 5 }, direction: 3, cursor: "ew-resize" },
    { style: { top: 0, left: 0, width: 16, height: 16 }, direction: 0, cursor: "nwse-resize" },
    { style: { top: 0, right: 0, width: 16, height: 16 }, direction: 2, cursor: "nesw-resize" },
    { style: { bottom: 0, right: 0, width: 16, height: 16 }, direction: 4, cursor: "nwse-resize" },
    { style: { bottom: 0, left: 0, width: 16, height: 16 }, direction: 6, cursor: "nesw-resize" },
  ]
  return (
    <>
      {zones.map((zone) => (
        <div
          key={zone.direction}
          onMouseDown={() => startWindowResize(zone.direction)}
          style={{
            position: "absolute",
            pointerEvents: "auto",
            cursor: zone.cursor as never,
            ...zone.style,
          }}
        />
      ))}
    </>
  )
}

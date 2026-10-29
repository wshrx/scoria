import { startWindowMove } from "../../chrome"
import { WindowControls } from "./window-controls"

/** Window chrome: a drag surface and the minimize/maximize/close controls. */
export function Topbar({ frameless }: { frameless: boolean }) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        height: 46,
        flexShrink: 0,
        paddingRight: 12,
        paddingLeft: 18,
        borderBottomWidth: 1,
        borderStyle: "solid",
        borderColor: "transparent",
      }}
    >
      <div
        onMouseDown={frameless ? () => startWindowMove() : undefined}
        style={{
          flexGrow: 1,
          alignSelf: "stretch",
          pointerEvents: frameless ? "auto" : "none",
          cursor: "default",
        }}
      />
      {frameless ? <WindowControls /> : null}
    </div>
  )
}

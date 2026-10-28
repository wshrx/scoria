/**
 * The window's own chrome.
 *
 * GPUIX opens a normal decorated window on Linux, and its only chrome option
 * (`titlebarTransparent`) is a macOS traffic-light setting. A launcher that
 * draws its own titlebar therefore has to ask X11 for a frameless window and
 * hand moves and resizes back to the window manager with `_NET_WM_MOVERESIZE`.
 *
 * The app runs through XWayland for this (see `app.tsx`); on a session with no
 * X server `enableFrameless` returns false and the native titlebar stays.
 */
import x11 from "x11"

type Client = any
type Property = { type: number; format: number; data: Buffer }

let client: Client | null = null
let root = 0
let window = 0
let moveAtom = 0
let shapeExt: any = null
let ready = false

function connect(): Promise<any> {
  return new Promise((resolve, reject) => {
    x11.createClient((error: Error | null, display: any) => {
      if (error) reject(error)
      else resolve(display)
    })
  })
}

function property(wid: number, atom: number, type = 0, length = 100_000): Promise<Property> {
  return new Promise((resolve, reject) => {
    client.GetProperty(0, wid, atom, type, 0, length, (error: Error | null, data: Property) => {
      if (error) reject(error)
      else resolve(data)
    })
  })
}

function atom(name: string): Promise<number> {
  return new Promise((resolve, reject) => {
    client.InternAtom(false, name, (error: Error | null, id: number) => {
      if (error) reject(error)
      else resolve(id)
    })
  })
}

function u32(data: Buffer): number[] {
  const values: number[] = []
  for (let i = 0; i + 4 <= data.length; i += 4) values.push(data.readUInt32LE(i))
  return values
}

async function findWindow(pid: number, listAtom: number, pidAtom: number): Promise<number> {
  const list = await property(root, listAtom)
  for (const wid of u32(list.data)) {
    const found = await property(wid, pidAtom, 0, 1)
    if (found.data.length >= 4 && found.data.readUInt32LE(0) === pid) return wid
  }
  return 0
}

/** Make the window frameless, once it exists. True when the chrome can be drawn. */
export async function enableFrameless(): Promise<boolean> {
  if (process.platform !== "linux" || !process.env.DISPLAY) return false
  try {
    const display = await connect()
    client = display.client
    root = display.screen[0].root
    const listAtom = await atom("_NET_CLIENT_LIST")
    const pidAtom = await atom("_NET_WM_PID")
    moveAtom = await atom("_NET_WM_MOVERESIZE")
    for (let attempt = 0; attempt < 40 && !window; attempt += 1) {
      window = await findWindow(process.pid, listAtom, pidAtom)
      if (!window) await new Promise((resolve) => setTimeout(resolve, 100))
    }
    if (!window) return false
    const motif = await atom("_MOTIF_WM_HINTS")
    // MWM_HINTS_DECORATIONS, decorations = 0: the window manager draws no frame.
    client.ChangeProperty(0, window, motif, motif, 32, [2, 0, 0, 0, 0])
    ready = true
    const shaped = await shapeWindow(undefined, undefined, 10, 24)
    if (!shaped) {
      // No SHAPE: give the decorations back rather than leave a black inset.
      client.ChangeProperty(0, window, motif, motif, 32, [2, 0, 1, 0, 0])
      ready = false
    }
    await new Promise<void>((resolve) => client.flush(resolve))
    return shaped
  } catch {
    return false
  }
}

function pointer(): Promise<{ x: number; y: number }> {
  return new Promise((resolve) => {
    client.QueryPointer(root, (error: Error | null, ptr: any) => {
      resolve(error ? { x: 0, y: 0 } : { x: ptr.rootX, y: ptr.rootY })
    })
  })
}

async function moveResize(direction: number) {
  if (!ready || !client || !window) return
  const at = await pointer()
  // x_root, y_root, direction, button, source indication (normal application).
  client.SendClientMessage(root, window, moveAtom, 32, [at.x, at.y, direction, 1, 1])
}

/** Start the window-manager move for a press on the titlebar. */
export function startWindowMove() {
  void moveResize(8)
}

/**
 * Start a window-manager resize from one of the eight edges. Directions follow
 * `_NET_WM_MOVERESIZE`: 0 top-left, 1 top, 2 top-right, 3 right, 4 bottom-right,
 * 5 bottom, 6 bottom-left, 7 left.
 */
export function startWindowResize(direction: number) {
  void moveResize(direction)
}

/**
 * Cut the window to a rounded rectangle inset from its edges. Without a
 * compositor alpha channel the padding and the four corners of a transparent
 * window paint black; a SHAPE bounding and input mask removes them instead, so
 * the desktop shows through and clicks fall through outside the shell.
 */
export async function shapeWindow(
  width?: number,
  height?: number,
  inset = 10,
  radius = 24,
): Promise<boolean> {
  if (!ready || !client || !window) return false
  try {
    if (!shapeExt) {
      shapeExt = await new Promise((resolve, reject) => {
        client.require("shape", (error: Error | null, ext: any) => {
          if (error) reject(error)
          else resolve(ext)
        })
      })
    }
    const geometry: any = await new Promise((resolve) => {
      client.GetGeometry(window, (error: Error | null, value: any) => resolve(value))
    })
    const physicalWidth = geometry?.width ?? 0
    const physicalHeight = geometry?.height ?? 0
    // GPUI sizes are logical; X11 shapes are physical. Scale the inset and radius
    // by the window's device pixel ratio so the cut matches the drawn shell.
    const scale = width && height ? Math.max(1, physicalWidth / width) : 1
    const cut = Math.round(inset * scale)
    const corner = Math.round(radius * scale)
    if (!physicalWidth || !physicalHeight || physicalWidth <= cut * 2 || physicalHeight <= cut * 2) {
      return false
    }
    const rects = roundedRects(cut, cut, physicalWidth - cut * 2, physicalHeight - cut * 2, corner)
    shapeExt.Rectangles(shapeExt.Op.Set, shapeExt.Kind.Bounding, window, 0, 0, rects)
    shapeExt.Rectangles(shapeExt.Op.Set, shapeExt.Kind.Input, window, 0, 0, rects)
    await new Promise<void>((resolve) => client.flush(resolve))
    return true
  } catch {
    return false
  }
}

/** A rounded rectangle as one-pixel rows, the shape protocol's rectangle list. */
function roundedRects(x: number, y: number, width: number, height: number, radius: number): number[][] {
  const r = Math.max(0, Math.min(Math.floor(radius), Math.floor(width / 2), Math.floor(height / 2)))
  const rects: number[][] = []
  for (let dy = 0; dy < r; dy += 1) {
    const dx = Math.round(r - Math.sqrt(Math.max(0, r * r - (r - dy - 1) * (r - dy - 1))))
    rects.push([x + dx, y + dy, width - dx * 2, 1])
  }
  rects.push([x, y + r, width, height - r * 2])
  for (let dy = 0; dy < r; dy += 1) {
    const dx = Math.round(r - Math.sqrt(Math.max(0, r * r - (dy + 1) * (dy + 1))))
    rects.push([x + dx, y + height - r + dy, width - dx * 2, 1])
  }
  return rects
}

export function chromeEnabled(): boolean {
  return ready
}

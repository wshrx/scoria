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
    await new Promise<void>((resolve) => client.flush(resolve))
    ready = true
    return true
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

export function chromeEnabled(): boolean {
  return ready
}

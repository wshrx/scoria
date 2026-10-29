/**
 * The bridge to the Rust engine: `horizon-launcher api` speaks JSON lines on stdin/stdout.
 *
 * The UI never touches the file system or starts the game itself. If the engine is not found
 * (a checkout without a built launcher, or the compiled UI before packaging), the screens fall
 * back to the preview snapshot and say so.
 */
import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process"
import { existsSync } from "node:fs"
import { dirname, resolve } from "node:path"
import { fileURLToPath } from "node:url"

const here = dirname(fileURLToPath(import.meta.url))

function engineBinary(): string {
  if (process.env.SCORIA_LAUNCHER_BIN) return process.env.SCORIA_LAUNCHER_BIN
  const suffix = process.platform === "win32" ? ".exe" : ""
  // A packaged build: the Rust engine sits beside the compiled UI.
  const beside = resolve(dirname(process.execPath), `horizon-launcher${suffix}`)
  if (existsSync(beside)) return beside
  const dev = resolve(here, `../../../launcher/target/debug/horizon-launcher${suffix}`)
  return existsSync(dev) ? dev : beside
}

type Pending = {
  resolve: (value: unknown) => void
  reject: (error: Error) => void
  timer: ReturnType<typeof setTimeout>
  onEvent?: (event: Record<string, unknown>) => void
}

let engine: ChildProcessWithoutNullStreams | null = null
let buffer = ""
let nextId = 1
let unavailable: string | null = null
const pending = new Map<number, Pending>()

function failAll(error: Error) {
  for (const [, entry] of pending) {
    clearTimeout(entry.timer)
    entry.reject(error)
  }
  pending.clear()
}

function onLine(line: string) {
  let message: { id?: number; ok?: boolean; result?: unknown; error?: string; event?: string }
  try {
    message = JSON.parse(line)
  } catch {
    return
  }
  if (typeof message.id !== "number") return
  const entry = pending.get(message.id)
  if (!entry) return
  if (message.event) {
    entry.onEvent?.(message as Record<string, unknown>)
    return
  }
  pending.delete(message.id)
  clearTimeout(entry.timer)
  if (message.ok) entry.resolve(message.result)
  else entry.reject(new Error(message.error ?? "the engine refused"))
}

function start(): ChildProcessWithoutNullStreams | null {
  if (engine) return engine
  if (unavailable) return null
  const bin = engineBinary()
  if (!existsSync(bin)) {
    unavailable = `engine not found at ${bin}`
    return null
  }
  try {
    const child = spawn(bin, ["api"], { stdio: ["pipe", "pipe", "pipe"], windowsHide: true })
    child.stdout.on("data", (chunk: Buffer) => {
      buffer += chunk.toString("utf8")
      let newline = buffer.indexOf("\n")
      while (newline >= 0) {
        const line = buffer.slice(0, newline)
        buffer = buffer.slice(newline + 1)
        onLine(line)
        newline = buffer.indexOf("\n")
      }
    })
    child.stderr.on("data", (chunk: Buffer) => {
      const text = chunk.toString("utf8").trim()
      if (text) console.error(`[scoria] ${text}`)
    })
    child.on("error", (error) => {
      unavailable = error.message
      failAll(error)
      engine = null
    })
    child.on("exit", (code) => {
      if (pending.size > 0) failAll(new Error(`the engine exited (${code ?? "signal"})`))
      engine = null
    })
    engine = child
    return child
  } catch (error) {
    unavailable = String(error)
    return null
  }
}

/** One request on the engine, with optional events until the final response. */
export function call<T>(
  method: string,
  params: unknown = {},
  timeoutMs = 10_000,
  onEvent?: (event: Record<string, unknown>) => void,
): Promise<T> {
  const child = start()
  if (!child) return Promise.reject(new Error(unavailable ?? "the engine is not available"))
  const id = nextId++
  return new Promise<T>((resolvePromise, rejectPromise) => {
    const timer = setTimeout(() => {
      pending.delete(id)
      rejectPromise(new Error(`the engine did not answer ${method} in time`))
    }, timeoutMs)
    pending.set(id, {
      resolve: resolvePromise as (value: unknown) => void,
      reject: rejectPromise,
      timer,
      onEvent,
    })
    child.stdin.write(`${JSON.stringify({ id, method, params })}\n`)
  })
}

process.on("exit", () => {
  engine?.kill()
})

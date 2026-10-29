import { createContext, useCallback, useContext, useEffect, useRef, useState } from "react"
import type { ReactNode } from "react"
import { motion } from "@gpuix/react"
import { Icon, Text, useTheme, type IconName } from "@/lib/flint"
import { useT } from "../../i18n"

export type ToastTone = "success" | "error" | "info"

export interface ToastOptions {
  title: string
  description?: string
  tone?: ToastTone
  /** Milliseconds before it disappears; errors stay twice as long by default. */
  duration?: number
}

interface ToastItem extends ToastOptions {
  id: number
  tone: ToastTone
}

type ToastApi = (toast: ToastOptions) => void

interface ToastState {
  items: ToastItem[]
  dismiss: (id: number) => void
}

const ToastContext = createContext<ToastApi | null>(null)
const ViewportContext = createContext<ToastState | null>(null)

/** Global feedback: one bottom-right stack, used by every screen. */
export function useToast(): ToastApi {
  const api = useContext(ToastContext)
  if (!api) throw new Error("flint: useToast must be used inside <ToastProvider>")
  return api
}

export function ToastProvider({ children }: { children?: ReactNode }) {
  const [items, setItems] = useState<ToastItem[]>([])
  const timers = useRef(new Map<number, ReturnType<typeof setTimeout>>())
  const next = useRef(1)

  const dismiss = useCallback((id: number) => {
    const timer = timers.current.get(id)
    if (timer) clearTimeout(timer)
    timers.current.delete(id)
    setItems((list) => list.filter((item) => item.id !== id))
  }, [])

  const show = useCallback(
    ({ title, description, tone = "info", duration }: ToastOptions) => {
      const id = next.current
      next.current += 1
      // Three at a time: more covers the page and repeats what the panel says.
      setItems((list) => [...list.slice(-2), { id, title, description, tone }])
      const ms = duration ?? (tone === "error" ? 7000 : 4200)
      timers.current.set(id, setTimeout(() => dismiss(id), ms))
    },
    [dismiss]
  )

  useEffect(() => {
    const pending = timers.current
    return () => {
      for (const timer of pending.values()) clearTimeout(timer)
    }
  }, [])

  return (
    <ToastContext.Provider value={show}>
      <ViewportContext.Provider value={{ items, dismiss }}>{children}</ViewportContext.Provider>
    </ToastContext.Provider>
  )
}

/** Mount inside the window's relative root; the stack paints above the page. */
export function ToastViewport() {
  const state = useContext(ViewportContext)
  if (!state) return null
  return (
    <div
      role="status"
      style={{
        position: "absolute",
        right: 18,
        bottom: 18,
        width: 360,
        display: "flex",
        flexDirection: "column",
        gap: 8,
        pointerEvents: "none",
      }}
    >
      {state.items.map((item) => (
        <ToastCard key={item.id} item={item} onClose={() => state.dismiss(item.id)} />
      ))}
    </div>
  )
}

const TONES: Record<ToastTone, { icon: IconName; color: (theme: ReturnType<typeof useTheme>) => string }> = {
  success: { icon: "check", color: (theme) => theme.colors.success },
  error: { icon: "alert", color: (theme) => theme.colors.destructive },
  info: { icon: "info", color: (theme) => theme.colors.primary },
}

function ToastCard({ item, onClose }: { item: ToastItem; onClose: () => void }) {
  const theme = useTheme()
  const t = useT()
  const tone = TONES[item.tone]
  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.18, ease: "easeOut" }}
      style={{
        pointerEvents: "auto",
        display: "flex",
        flexDirection: "row",
        alignItems: "flex-start",
        gap: 10,
        padding: 12,
        borderRadius: 14,
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: theme.colors.border,
        backgroundColor: theme.colors.popover,
      }}
    >
      <Icon name={tone.icon} size={15} color={tone.color(theme)} />
      <div style={{ display: "flex", flexDirection: "column", gap: 2, flexGrow: 1, minWidth: 0 }}>
        <Text size="sm" weight="medium">
          {item.title}
        </Text>
        {item.description ? (
          <Text size="xs" tone="muted" style={{ lineClamp: 2 }}>
            {item.description}
          </Text>
        ) : null}
      </div>
      <div
        role="button"
        tabIndex={0}
        aria-label={t("Dismiss")}
        onClick={onClose}
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          width: 22,
          height: 22,
          borderRadius: 7,
          cursor: "pointer",
          hover: { backgroundColor: theme.colors.accent },
          focusVisible: { outlineWidth: 2, outlineColor: theme.colors.ring, outlineOffset: 1 },
        }}
      >
        <Icon name="x" size={12} color={theme.colors.mutedForeground} />
      </div>
    </motion.div>
  )
}

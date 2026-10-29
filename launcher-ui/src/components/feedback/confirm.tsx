import { createContext, useCallback, useContext, useState } from "react"
import type { ReactNode } from "react"
import {
  Dialog,
  DialogBackdrop,
  DialogDescription,
  DialogPopup,
  DialogPortal,
  DialogTitle,
} from "@gpuix/react"
import { Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import { useT } from "../../i18n"

export interface ConfirmOptions {
  title: string
  description?: string
  confirmLabel?: string
  cancelLabel?: string
  /** Red confirm button and a warning icon for destructive actions. */
  tone?: "default" | "destructive"
}

type ConfirmFn = (options: ConfirmOptions) => Promise<boolean>

const ConfirmContext = createContext<ConfirmFn | null>(null)

/** Ask before a destructive action; resolves true only on Confirm. */
export function useConfirm(): ConfirmFn {
  const confirm = useContext(ConfirmContext)
  if (!confirm) throw new Error("flint: useConfirm must be used inside <ConfirmProvider>")
  return confirm
}

interface Pending {
  options: ConfirmOptions
  resolve: (confirmed: boolean) => void
}

export function ConfirmProvider({ children }: { children?: ReactNode }) {
  const theme = useTheme()
  const t = useT()
  const [pending, setPending] = useState<Pending | null>(null)

  const confirm = useCallback<ConfirmFn>((options) => {
    return new Promise<boolean>((resolve) => setPending({ options, resolve }))
  }, [])

  const finish = (confirmed: boolean) => {
    const current = pending
    if (!current) return
    setPending(null)
    current.resolve(confirmed)
  }

  const options = pending?.options
  return (
    <ConfirmContext.Provider value={confirm}>
      {children}
      <Dialog open={pending !== null} onOpenChange={(open) => (open ? undefined : finish(false))}>
        <DialogPortal>
          <DialogBackdrop style={{ backgroundColor: "#00000080" }} />
          {options ? (
            <DialogPopup
              onKeyDown={(event) => {
                if (event.key === "Enter" && !event.defaultPrevented) {
                  event.preventDefault()
                  finish(true)
                }
              }}
              style={{
                display: "flex",
                flexDirection: "column",
                gap: 14,
                width: 440,
                maxWidth: "92%",
                padding: 22,
                borderRadius: 18,
                borderWidth: 1,
                borderStyle: "solid",
                borderColor: theme.colors.border,
                backgroundColor: theme.colors.popover,
              }}
            >
              <DialogTitle>
                <Text size="lg" weight="semibold">
                  {options.title}
                </Text>
              </DialogTitle>
              {options.description ? (
                <DialogDescription>
                  <Text size="sm" tone="muted">
                    {options.description}
                  </Text>
                </DialogDescription>
              ) : null}
              <div style={{ display: "flex", flexDirection: "row", justifyContent: "flex-end", gap: 8 }}>
                <Button
                  variant="ghost"
                  autoFocus
                  onClick={() => finish(false)}
                  style={{ borderRadius: 999, height: 36, paddingLeft: 16, paddingRight: 16 }}
                >
                  {options.cancelLabel ?? t("Cancel")}
                </Button>
                <Button
                  variant={options.tone === "destructive" ? "destructive" : "primary"}
                  onClick={() => finish(true)}
                  style={{ borderRadius: 999, height: 36, paddingLeft: 18, paddingRight: 18 }}
                >
                  {options.confirmLabel ?? t("Confirm")}
                </Button>
              </div>
            </DialogPopup>
          ) : null}
        </DialogPortal>
      </Dialog>
    </ConfirmContext.Provider>
  )
}

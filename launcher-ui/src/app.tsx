import { useEffect, useRef, useState } from "react"
import { TooltipProvider } from "@gpuix/react"
import { Text, ThemeProvider } from "@/lib/flint"
import { setAppearance, setLanguage, snapshot, type Snapshot } from "./api"
import { Shell } from "./components/layout/shell"
import { SCREENS, screenFor, type ScreenId } from "./screens/registry"
import { scoriaDark, scoriaLight } from "./theme"
import { LangProvider } from "./i18n"
import { ConfirmProvider } from "./components/feedback/confirm"
import { ErrorBoundary } from "./components/feedback/error-boundary"
import { ToastProvider, ToastViewport, useToast } from "./components/feedback/toast"
import { useConfirm } from "./components/feedback/confirm"
import { qaOn } from "./qa"

/** Test-only states the QA skill screenshots; inert unless SCORIA_UI_QA is set. */
function QaFixtures() {
  const toast = useToast()
  const confirm = useConfirm()
  useEffect(() => {
    if (qaOn("toast")) {
      toast({ tone: "success", title: "Установлено.", description: "Sodium", duration: 60_000 })
      toast({ tone: "info", title: "Обновление доступно.", description: "Scoria 0.1.1 beta", duration: 60_000 })
      toast({ tone: "error", title: "Нет соединения. Проверьте сеть и попробуйте снова.", duration: 60_000 })
    }
    if (qaOn("dialog")) {
      void confirm({
        title: "Удалить сборку?",
        description: "Файлы сборки будут удалены с этого компьютера. Миры сохранятся.",
        confirmLabel: "Удалить",
        tone: "destructive",
      })
    }
  }, [confirm, toast])
  return null
}

export function App() {
  const [dark, setDark] = useState(true)
  const [page, setPage] = useState<ScreenId>(() => {
    const requested = process.env.SCORIA_UI_PAGE
    return SCREENS.some((screen) => screen.id === requested) ? (requested as ScreenId) : "home"
  })
  const [data, setData] = useState<Snapshot | null>(null)
  const live = useRef(false)

  const apply = (state: Snapshot) => {
    live.current = state.live
    setData(state)
    setDark(state.appearance !== "light")
  }

  useEffect(() => {
    void snapshot().then(apply)
  }, [])

  // A game writes a new world to disk while the launcher is open. Poll the
  // engine so the list updates when the player comes back, without a restart.
  useEffect(() => {
    let cancelled = false
    let running = false
    const timer = setInterval(() => {
      if (cancelled || !live.current || running) return
      running = true
      void snapshot()
        .then((state) => {
          if (!cancelled && state.live) apply(state)
        })
        .finally(() => {
          running = false
        })
    }, 4000)
    return () => {
      cancelled = true
      clearInterval(timer)
    }
  }, [])

  const theme = dark ? scoriaDark : scoriaLight
  const refresh = () => {
    void snapshot().then(apply)
  }
  const toggleTheme = () => {
    void setAppearance(dark ? "light" : "dark")
      .then(apply)
      .catch(() => setDark((value) => !value))
  }

  if (!data) {
    return (
      <ThemeProvider theme={theme}>
        <div
          style={{
            display: "flex",
            width: "100%",
            height: "100%",
            alignItems: "center",
            justifyContent: "center",
            backgroundColor: theme.colors.background,
          }}
        >
          <Text tone="muted">Loading…</Text>
        </div>
      </ThemeProvider>
    )
  }

  const Active = screenFor(page).component

  return (
    <ThemeProvider theme={theme}>
      <LangProvider lang={data.language === "ru" ? "ru" : "en"}>
        <TooltipProvider delayDuration={350}>
          <ToastProvider>
            <ConfirmProvider>
              <div style={{ display: "flex", width: "100%", height: "100%", position: "relative" }}>
                <ErrorBoundary>
                  <Shell page={page} onPage={setPage} data={data}>
                    <Active
                      data={data}
                      onRefresh={refresh}
                      onPage={setPage}
                      dark={dark}
                      onToggleTheme={toggleTheme}
                      onLanguage={(value) => {
                        void setLanguage(value).then(apply)
                      }}
                    />
                  </Shell>
                </ErrorBoundary>
                <ToastViewport />
                <QaFixtures />
              </div>
            </ConfirmProvider>
          </ToastProvider>
        </TooltipProvider>
      </LangProvider>
    </ThemeProvider>
  )
}

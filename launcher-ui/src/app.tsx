import { useEffect, useRef, useState } from "react"
import { existsSync } from "node:fs"
import { dirname, resolve } from "node:path"
import { useGpuixRequired } from "@gpuix/react"
import { Text, ThemeProvider, registerFlintFonts } from "@/lib/flint"
import { setAppearance, setLanguage, snapshot, type Snapshot } from "./api"
import { Shell } from "./components/layout/shell"
import { SCREENS, screenFor, type ScreenId } from "./screens/registry"
import { scoriaDark, scoriaLight } from "./theme"
import { LangProvider } from "./i18n"
import { enableFrameless } from "./chrome"

/** Fonts need a live renderer, so they are registered after the window exists. */
function Fonts() {
  const renderer = useGpuixRequired()
  useEffect(() => {
    // In a checkout the fonts are in the tree; in a packaged build they sit beside the binary.
    const dir = existsSync("./src/lib/fonts")
      ? "./src/lib/fonts"
      : resolve(dirname(process.execPath), "fonts")
    void registerFlintFonts(renderer, { dir })
  }, [renderer])
  return null
}

export function App() {
  const [dark, setDark] = useState(true)
  const [frameless, setFrameless] = useState(false)
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

  useEffect(() => {
    let alive = true
    void enableFrameless().then((ok) => {
      if (alive) setFrameless(ok)
    })
    return () => {
      alive = false
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
            backgroundColor: frameless ? "transparent" : theme.colors.background,
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
        <Fonts />
        <Shell page={page} onPage={setPage} data={data} frameless={frameless}>
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
      </LangProvider>
    </ThemeProvider>
  )
}

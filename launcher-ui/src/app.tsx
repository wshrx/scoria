import { useEffect, useState } from "react"
import { existsSync } from "node:fs"
import { dirname, resolve } from "node:path"
import { useGpuixRequired } from "@gpuix/react"
import { Text, ThemeProvider, registerFlintFonts } from "@/lib/flint"
import { setAppearance, setLanguage, snapshot, type Snapshot } from "./api"
import { Shell, type Page } from "./shell"
import { HomePage } from "./pages/home"
import { InstancesPage } from "./pages/instances"
import { FilesPage } from "./pages/files"
import { ServersPage } from "./pages/servers"
import { SettingsPage } from "./pages/settings"
import { scoriaDark, scoriaLight } from "./theme"
import { LangProvider } from "./i18n"

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
  const [page, setPage] = useState<Page>(() => {
    const requested = process.env.SCORIA_UI_PAGE
    return requested === "instances" || requested === "files" || requested === "servers" || requested === "settings"
      ? requested
      : "home"
  })
  const [data, setData] = useState<Snapshot | null>(null)

  const apply = (state: Snapshot) => {
    setData(state)
    setDark(state.appearance !== "light")
  }

  useEffect(() => {
    void snapshot().then(apply)
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
        <div style={{ display: "flex", width: "100%", height: "100%", alignItems: "center", justifyContent: "center" }}>
          <Text tone="muted">Loading…</Text>
        </div>
      </ThemeProvider>
    )
  }

  return (
    <ThemeProvider theme={theme}>
      <LangProvider lang={data.language === "ru" ? "ru" : "en"}>
      <Fonts />
      <Shell
        page={page}
        onPage={setPage}
        dark={dark}
        onToggleTheme={toggleTheme}
        data={data}
      >
        {page === "home" ? <HomePage data={data} onRefresh={refresh} onPage={setPage} /> : null}
        {page === "instances" ? <InstancesPage data={data} onRefresh={refresh} /> : null}
        {page === "files" ? <FilesPage data={data} onRefresh={refresh} /> : null}
        {page === "servers" ? <ServersPage /> : null}
        {page === "settings" ? (
          <SettingsPage
            data={data}
            dark={dark}
            onToggleTheme={toggleTheme}
            onRefresh={refresh}
            onLanguage={(value) => {
              void setLanguage(value).then(apply)
            }}
          />
        ) : null}
      </Shell>
      </LangProvider>
    </ThemeProvider>
  )
}

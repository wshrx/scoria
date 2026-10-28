import { useEffect, useState } from "react"
import { useGpuixRequired } from "@gpuix/react"
import { Text, ThemeProvider, registerFlintFonts } from "@/lib/flint"
import { snapshot, type Snapshot } from "./api"
import { Shell, type Page } from "./shell"
import { HomePage } from "./pages/home"
import { InstancesPage } from "./pages/instances"
import { SettingsPage } from "./pages/settings"
import { scoriaDark, scoriaLight } from "./theme"

/** Fonts need a live renderer, so they are registered after the window exists. */
function Fonts() {
  const renderer = useGpuixRequired()
  useEffect(() => {
    void registerFlintFonts(renderer, { dir: "./src/lib/fonts" })
  }, [renderer])
  return null
}

export function App() {
  const [dark, setDark] = useState(true)
  const [page, setPage] = useState<Page>("home")
  const [data, setData] = useState<Snapshot | null>(null)

  useEffect(() => {
    void snapshot().then(setData)
  }, [])

  const theme = dark ? scoriaDark : scoriaLight
  const refresh = () => {
    void snapshot().then(setData)
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
      <Fonts />
      <Shell
        page={page}
        onPage={setPage}
        dark={dark}
        onToggleTheme={() => setDark((value) => !value)}
        data={data}
      >
        {page === "home" ? <HomePage data={data} onRefresh={refresh} /> : null}
        {page === "instances" ? <InstancesPage data={data} /> : null}
        {page === "settings" ? (
          <SettingsPage
            data={data}
            dark={dark}
            onToggleTheme={() => setDark((value) => !value)}
            onRefresh={refresh}
          />
        ) : null}
      </Shell>
    </ThemeProvider>
  )
}

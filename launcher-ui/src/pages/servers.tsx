import { useEffect, useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { addServer, play, removeServer, savedServers, type SavedServer } from "../api"
import { useT } from "../i18n"
import { Sheet } from "../visual"

export function ServersPage() {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const narrow = width < 1000
  const [servers, setServers] = useState<SavedServer[]>([])
  const [adding, setAdding] = useState(false)
  const [name, setName] = useState("")
  const [ip, setIp] = useState("")
  const [note, setNote] = useState<string | null>(null)
  const [confirming, setConfirming] = useState<number | null>(null)
  const [busy, setBusy] = useState<string | null>(null)

  const refresh = () => {
    void savedServers()
      .then(setServers)
      .catch((error: Error) => setNote(error.message))
  }

  useEffect(refresh, [])

  return (
    <Sheet style={{ padding: narrow ? 20 : 30, gap: 20, }}>
      <div style={{ display: "flex", flexDirection: "row", alignItems: "flex-end", gap: 16 }}>
        <div style={{ display: "flex", flexDirection: "column", gap: 4, flexGrow: 1 }}>
          <Text style={{ fontSize: narrow ? 30 : 38, lineHeight: 42, fontWeight: "700" }}>
            {t("Servers")}
          </Text>
          <Text size="sm" tone="muted">
            {t("Offline-mode servers, saved in this instance.")}
          </Text>
        </div>
        <Button
          variant={adding ? "ghost" : "secondary"}
          onClick={() => {
            setAdding((value) => !value)
            setNote(null)
          }}
          style={{ borderRadius: 999, height: 36, paddingLeft: 16, paddingRight: 16 }}
        >
          <Icon name={adding ? "x" : "plus"} size={13} color={theme.colors.foreground} />
          {adding ? t("Cancel") : t("Add server")}
        </Button>
      </div>

      {adding ? (
        <div
          style={{
            display: "flex",
            flexDirection: narrow ? "column" : "row",
            alignItems: narrow ? "stretch" : "center",
            gap: 8,
            padding: 14,
            borderRadius: 18,
            borderWidth: 1,
            borderStyle: "dashed",
            borderColor: theme.colors.border,
            backgroundColor: theme.colors.muted,
          }}
        >
          <div style={{ width: narrow ? "100%" : 200 }}>
            <Input value={name} placeholder={t("Name")} onChange={(event) => setName(event.value ?? "")} />
          </div>
          <div style={{ width: narrow ? "100%" : 260 }}>
            <Input value={ip} placeholder="host:port" onChange={(event) => setIp(event.value ?? "")} />
          </div>
          <Button
            variant="primary"
            loading={busy === "add"}
            onClick={() => {
              setBusy("add")
              void addServer(name.trim(), ip.trim())
                .then((list) => {
                  setServers(list)
                  setAdding(false)
                  setName("")
                  setIp("")
                  setNote(null)
                })
                .catch((error: Error) => setNote(error.message))
                .finally(() => setBusy(null))
            }}
            style={{ borderRadius: 999, height: 36, paddingLeft: 18, paddingRight: 18 }}
          >
            {t("Add")}
          </Button>
        </div>
      ) : null}

      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        {servers.length === 0 ? (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              gap: 6,
              padding: 22,
              borderRadius: 18,
              borderWidth: 1,
              borderStyle: "dashed",
              borderColor: theme.colors.border,
            }}
          >
            <Text size="sm">{t("No servers yet.")}</Text>
            <Text size="xs" tone="muted">
              {t("Add an offline-mode server, or open a world to LAN.")}
            </Text>
          </div>
        ) : (
          servers.map((server, index) => (
            <div
              key={`${server.ip}-${index}`}
              style={{
                display: "flex",
                flexDirection: narrow ? "column" : "row",
                alignItems: narrow ? "stretch" : "center",
                gap: 12,
                padding: 12,
                borderRadius: 16,
                backgroundColor: theme.colors.muted,
                hover: { backgroundColor: theme.colors.accent },
              }}
            >
              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  width: 36,
                  height: 36,
                  borderRadius: 12,
                  backgroundColor: theme.colors.accent,
                }}
              >
                <Icon name="server" size={15} color={theme.colors.mutedForeground} />
              </div>
              <div style={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0 }}>
                <Text size="base" weight="medium" truncate>
                  {server.name || server.ip}
                </Text>
                <Text mono size="xs" tone="muted">
                  {server.ip}
                </Text>
              </div>
              <div style={{ display: "flex", flexDirection: "row", gap: 6 }}>
                <Button
                  size="sm"
                  variant="secondary"
                  loading={busy === `join-${index}`}
                  onClick={() => {
                    setBusy(`join-${index}`)
                    setNote(null)
                    void play({ server: server.ip }).then((said) => {
                      setNote(said)
                      setBusy(null)
                    })
                  }}
                  style={{ borderRadius: 999, height: 28, paddingLeft: 14, paddingRight: 14 }}
                >
                  {t("Join")}
                </Button>
                <Button
                  size="sm"
                  variant={confirming === index ? "destructive" : "ghost"}
                  onClick={() => {
                    if (confirming !== index) {
                      setConfirming(index)
                      return
                    }
                    setConfirming(null)
                    void removeServer(index)
                      .then(setServers)
                      .catch((error: Error) => setNote(error.message))
                  }}
                  style={{ borderRadius: 999, height: 28, paddingLeft: 12, paddingRight: 12 }}
                >
                  {confirming === index ? t("Confirm") : t("Remove")}
                </Button>
              </div>
            </div>
          ))
        )}
      </div>

      {note ? (
        <Text size="xs" tone="muted">
          {note}
        </Text>
      ) : null}

      <div style={{ flexGrow: 1 }} />
      <Text size="xs" tone="muted">
        {t("Online-mode servers need a Microsoft session. Scoria has none, so use offline-mode servers or LAN.")}
      </Text>
    </Sheet>
  )
}

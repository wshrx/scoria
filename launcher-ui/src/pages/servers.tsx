import { useEffect, useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { addServer, play, removeServer, savedServers, type SavedServer } from "../api"
import { useT } from "../i18n"
import { Panel } from "../visual"

export function ServersPage() {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const narrow = width < 900
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
    <div style={{ display: "flex", flexDirection: "column", gap: 22 }}>
      <div
        style={{
          display: "flex",
          flexDirection: narrow ? "column" : "row",
          alignItems: narrow ? "flex-start" : "flex-end",
          gap: 16,
        }}
      >
        <div style={{ display: "flex", flexDirection: "column", gap: 6, flexGrow: 1 }}>
          <Text style={{ fontSize: 38, lineHeight: 42, fontWeight: "700" }}>{t("Servers")}</Text>
          <Text size="sm" tone="muted" style={{ maxWidth: 620 }}>
            {t("Saved in this instance's servers.dat; offline-mode servers only.")}
          </Text>
        </div>
        <Button
          variant={adding ? "ghost" : "secondary"}
          size="sm"
          onClick={() => {
            setAdding((value) => !value)
            setNote(null)
          }}
        >
          <Icon name={adding ? "x" : "plus"} size={13} color={theme.colors.mutedForeground} />
          {adding ? t("Cancel") : t("Add server")}
        </Button>
      </div>

      {adding ? (
        <Panel style={{ padding: 16, gap: 10, borderStyle: "dashed" }}>
          <Text size="xs" tone="muted">
            {t("A name is only a label. The address is what the game connects to.")}
          </Text>
          <div
            style={{
              display: "flex",
              flexDirection: narrow ? "column" : "row",
              gap: 8,
              alignItems: narrow ? "stretch" : "center",
            }}
          >
            <div style={{ width: narrow ? "100%" : 220 }}>
              <Input value={name} placeholder={t("Name")} onChange={(event) => setName(event.value ?? "")} />
            </div>
            <div style={{ width: narrow ? "100%" : 280 }}>
              <Input value={ip} placeholder="host:port" onChange={(event) => setIp(event.value ?? "")} />
            </div>
            <Button
              variant="secondary"
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
            >
              {t("Add")}
            </Button>
          </div>
          {note ? (
            <Text size="xs" tone="destructive">
              {note}
            </Text>
          ) : null}
        </Panel>
      ) : null}

      <div style={{ display: "flex", flexDirection: "column", borderTopWidth: 1, borderStyle: "solid", borderColor: theme.colors.border }}>
        {servers.length === 0 ? (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              gap: 8,
              paddingTop: 22,
              paddingBottom: 22,
              borderBottomWidth: 1,
              borderStyle: "solid",
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
                paddingTop: 13,
                paddingBottom: 13,
                paddingLeft: 4,
                borderBottomWidth: 1,
                borderStyle: "solid",
                borderColor: theme.colors.border,
                hover: { backgroundColor: theme.colors.accent },
              }}
            >
              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  width: 34,
                  height: 34,
                  flexShrink: 0,
                  borderRadius: theme.radius.sm,
                  borderWidth: 1,
                  borderStyle: "solid",
                  borderColor: theme.colors.border,
                }}
              >
                <Icon name="server" size={14} color={theme.colors.mutedForeground} />
              </div>
              <div style={{ display: "flex", flexDirection: "column", gap: 2, flexGrow: 1, minWidth: 0 }}>
                <Text size="base" weight="medium" truncate>
                  {server.name || server.ip}
                </Text>
                <Text mono size="xs" tone="muted">
                  {server.ip}
                </Text>
              </div>
              <Text mono size="xs" tone="muted">
                {t("offline-mode")}
              </Text>
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

      <Panel style={{ padding: 18, gap: 6 }}>
        <Text size="sm" weight="medium">
          {t("Online-mode servers need a Microsoft session")}
        </Text>
        <Text size="xs" tone="muted">
          {t(
            "Scoria has no Microsoft session, so online-mode servers refuse it. Use an offline-mode server, Open to LAN, or Play with friends.",
          )}
        </Text>
      </Panel>
    </div>
  )
}

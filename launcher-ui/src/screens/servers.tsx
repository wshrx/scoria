import { useEffect, useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { addServer, play, removeServer, savedServers, type SavedServer } from "../api"
import { useT } from "../i18n"
import { friendlyError } from "../errors"
import { PageHeader } from "../components/layout/page-header"
import { EmptyState } from "../components/feedback/empty-state"
import { SkeletonList } from "../components/feedback/skeleton"
import { useToast } from "../components/feedback/toast"
import { useConfirm } from "../components/feedback/confirm"

export function ServersPage() {
  const theme = useTheme()
  const t = useT()
  const toast = useToast()
  const confirm = useConfirm()
  const { width } = useWindowSize()
  const narrow = width < 1000
  const [servers, setServers] = useState<SavedServer[]>([])
  const [adding, setAdding] = useState(false)
  const [name, setName] = useState("")
  const [ip, setIp] = useState("")
  const [busy, setBusy] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)

  const refresh = () => {
    setLoading(true)
    void savedServers()
      .then(setServers)
      .catch((error: Error) => toast({ tone: "error", title: friendlyError(error, t) }))
      .finally(() => setLoading(false))
  }

  useEffect(refresh, [])

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 16 }}>
      <PageHeader
        title={t("Servers")}
        subtitle={t("Offline-mode servers and LAN.")}
        actions={
          <Button
            variant={adding ? "ghost" : "secondary"}
            onClick={() => setAdding((value) => !value)}
            style={{ borderRadius: 999, height: 36, paddingLeft: 16, paddingRight: 16 }}
          >
            <Icon name={adding ? "x" : "plus"} size={13} color={theme.colors.foreground} />
            {adding ? t("Cancel") : t("Add server")}
          </Button>
        }
      />
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
                  toast({ tone: "success", title: t("Server added.") })
                })
                .catch((error: Error) => toast({ tone: "error", title: friendlyError(error, t) }))
                .finally(() => setBusy(null))
            }}
            style={{ borderRadius: 999, height: 36, paddingLeft: 18, paddingRight: 18 }}
          >
            {t("Add")}
          </Button>
        </div>
      ) : null}

      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        {loading && servers.length === 0 ? (
          <SkeletonList rows={2} height={64} />
        ) : servers.length === 0 ? (
          <EmptyState
            icon="server"
            title={t("No servers yet.")}
            description={t("Add an offline-mode server, or open a world to LAN.")}
          />
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
                    void play({ server: server.ip })
                      .then((said) => {
                        if (!said.startsWith("Started")) {
                          toast({ tone: "error", title: friendlyError(said, t) })
                        }
                      })
                      .catch((error: Error) => toast({ tone: "error", title: friendlyError(error, t) }))
                      .finally(() => setBusy(null))
                  }}
                  style={{ borderRadius: 999, height: 28, paddingLeft: 14, paddingRight: 14 }}
                >
                  {t("Join")}
                </Button>
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={async () => {
                    const ok = await confirm({
                      title: t("Remove server?"),
                      description: t("The server is removed from this list. Its address and name are forgotten."),
                      confirmLabel: t("Remove"),
                      tone: "destructive",
                    })
                    if (!ok) return
                    try {
                      const list = await removeServer(index)
                      setServers(list)
                      toast({ tone: "success", title: t("Server removed.") })
                    } catch (error) {
                      toast({ tone: "error", title: friendlyError(error, t) })
                    }
                  }}
                  style={{ borderRadius: 999, height: 28, paddingLeft: 12, paddingRight: 12 }}
                >
                  {t("Remove")}
                </Button>
              </div>
            </div>
          ))
        )}
      </div>

    </div>
  )
}

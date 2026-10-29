import { useEffect, useState, type ReactNode } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import { Field } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import type { DownloadProgress, Snapshot } from "../api"
import {
  downloadMinecraft,
  installLatest,
  prepare,
  removeVersion,
  selectVersion,
  setOfflineName,
  setWhilePlaying,
  versions,
  type VersionInfo,
} from "../api"
import { useT } from "../i18n"
import { LAUNCHER_CHANNEL, LAUNCHER_VERSION } from "../version"
import { downloadStopped, friendlyError } from "../errors"
import { EmptyState } from "../components/feedback/empty-state"
import { SkeletonList } from "../components/feedback/skeleton"
import { useToast } from "../components/feedback/toast"
import { useConfirm } from "../components/feedback/confirm"
import { PageHeader } from "../components/layout/page-header"
import { Avatar, Meter, Sheet } from "../visual"

type Tab = "general" | "game" | "builds" | "about"

const TABS: { id: Tab; label: string; icon: "sliders" | "hardDrive" | "box" | "info" }[] = [
  { id: "general", label: "General", icon: "sliders" },
  { id: "game", label: "Game files", icon: "hardDrive" },
  { id: "builds", label: "Builds", icon: "box" },
  { id: "about", label: "About", icon: "info" },
]

const MODES: { value: "stay" | "minimize" | "hide"; label: string }[] = [
  { value: "stay", label: "Stay" },
  { value: "minimize", label: "Minimize" },
  { value: "hide", label: "Hide" },
]

function megabytes(bytes: number): string {
  if (bytes >= 1_000_000_000) return `${(bytes / 1e9).toFixed(1)} GB`
  if (bytes <= 0) return "0 MB"
  return `${Math.round(bytes / 1e6)} MB`
}

function SectionField({
  label,
  detail,
  children,
}: {
  label: string
  detail?: string
  children?: ReactNode
}) {
  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
      <Text size="sm" weight="medium">
        {label}
      </Text>
      {detail ? (
        <Text size="xs" tone="muted">
          {detail}
        </Text>
      ) : null}
      {children}
    </div>
  )
}

export function SettingsPage({
  data,
  dark,
  onToggleTheme,
  onRefresh,
  onLanguage,
}: {
  data: Snapshot
  dark: boolean
  onToggleTheme: () => void
  onRefresh: () => void
  onLanguage: (value: "en" | "ru" | "system") => void
}) {
  const theme = useTheme()
  const t = useT()
  const toast = useToast()
  const confirm = useConfirm()
  const { width } = useWindowSize()
  const narrow = width < 1000
  const files = data.minecraft

  const [tab, setTab] = useState<Tab>(() => {
    const requested = process.env.SCORIA_UI_SETTINGS_TAB
    return TABS.some((entry) => entry.id === requested) ? (requested as Tab) : "general"
  })
  const [name, setName] = useState(data.offlineName)
  const [nameError, setNameError] = useState<string | null>(null)
  const [nameHint, setNameHint] = useState<string | null>(null)
  const [busy, setBusy] = useState<"save" | "download" | "prepare" | null>(null)
  const [progress, setProgress] = useState<DownloadProgress | null>(null)
  const [log, setLog] = useState<string[]>([])
  const [details, setDetails] = useState(false)

  const [installed, setInstalled] = useState<VersionInfo[]>([])
  const [selected, setSelected] = useState<string | null>(null)
  const [versionBusy, setVersionBusy] = useState<string | null>(null)
  const [versionsLoading, setVersionsLoading] = useState(true)

  const refreshVersions = () => {
    setVersionsLoading(true)
    void versions()
      .then((state) => {
        setInstalled(state.installed)
        setSelected(state.selected)
      })
      .catch((error: Error) => {
        if (data.live) toast({ tone: "error", title: friendlyError(error, t) })
      })
      .finally(() => setVersionsLoading(false))
  }

  useEffect(refreshVersions, [])
  useEffect(() => setName(data.offlineName), [data.offlineName])

  const pushLog = (line: string) => setLog((lines) => [...lines.slice(-5), line])
  const done = (files.client ? 1 : 0) + files.libraries.done + files.assets.done
  const total = 1 + files.libraries.total + files.assets.total
  const percent = total > 0 ? Math.round((done / total) * 100) : 0

  const download = async () => {
    setBusy("download")
    setProgress(null)
    setLog([])
    try {
      await downloadMinecraft(pushLog, setProgress)
      toast({ tone: "success", title: t("Game files are ready.") })
      onRefresh()
    } catch (error) {
      toast({ tone: "error", title: downloadStopped(error, t) })
    } finally {
      setBusy(null)
      setProgress(null)
    }
  }

  const prepareAll = async () => {
    setBusy("prepare")
    setLog([])
    setProgress(null)
    try {
      if (!files.installed) await downloadMinecraft(pushLog, setProgress)
      const said = await prepare(undefined, pushLog, false)
      if (said.startsWith("Preparation failed")) {
        toast({ tone: "error", title: friendlyError(said, t) })
      } else {
        toast({ tone: "success", title: t("Resources prepared.") })
      }
      onRefresh()
    } catch (error) {
      toast({ tone: "error", title: downloadStopped(error, t) })
    } finally {
      setBusy(null)
      setProgress(null)
    }
  }

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 18 }}>
      <PageHeader
        title={t("Settings")}
        subtitle={t("Profile, appearance, game files and builds in one place.")}
      />
      <Sheet
      style={{
        flexDirection: narrow ? "column" : "row",
        padding: 0,
        gap: 0,
        minHeight: 600,
        overflow: "hidden",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: narrow ? "row" : "column",
          gap: 6,
          width: narrow ? "100%" : 210,
          flexShrink: 0,
          padding: 20,
          borderRightWidth: narrow ? 0 : 1,
          borderBottomWidth: narrow ? 1 : 0,
          borderStyle: "solid",
          borderColor: theme.colors.border,
          backgroundColor: theme.appearance === "dark" ? "#FFFFFF05" : theme.colors.muted,
        }}
      >
        {TABS.map((entry) => {
          const active = tab === entry.id
          return (
            <div
              key={entry.id}
              role="button"
              tabIndex={0}
              onClick={() => setTab(entry.id)}
              style={{
                display: "flex",
                flexDirection: "row",
                alignItems: "center",
                gap: 9,
                height: 36,
                paddingLeft: 12,
                paddingRight: 12,
                borderRadius: 12,
                cursor: "pointer",
                backgroundColor: active ? "#FF8A4C24" : "transparent",
                hover: { backgroundColor: active ? "#FF8A4C2E" : theme.colors.accent },
              }}
            >
              <Icon
                name={entry.icon}
                size={15}
                color={active ? theme.colors.primary : theme.colors.mutedForeground}
              />
              <Text size="xs" weight={active ? "semibold" : "regular"}>
                {t(entry.label)}
              </Text>
            </div>
          )
        })}
      </div>

      <div style={{ display: "flex", flexDirection: "column", gap: 22, flexGrow: 1, padding: narrow ? 20 : 30 }}>
        {tab === "general" ? (
          <>
            <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
              <Text size="xl" weight="semibold">
                {t("Profile")}
              </Text>
              <div style={{ display: "flex", flexDirection: narrow ? "column" : "row", alignItems: narrow ? "stretch" : "flex-start", gap: 10 }}>
                <Avatar name={name || data.offlineName} size={42} style={{ marginTop: 20 }} />
                <Field
                  label={t("Player name")}
                  hint={nameHint ?? t("The name the game shows in single player and on LAN.")}
                  error={nameError ?? undefined}
                  style={{ width: narrow ? "100%" : 280 }}
                >
                  <Input
                    value={name}
                    placeholder="Builder_1"
                    invalid={nameError !== null}
                    onChange={(event) => {
                      setName(event.value ?? "")
                      setNameError(null)
                      setNameHint(null)
                    }}
                  />
                </Field>
                <Button
                  variant="primary"
                  loading={busy === "save"}
                  onClick={() => {
                    const trimmed = name.trim()
                    if (!/^[\p{L}\p{N}_]{3,16}$/u.test(trimmed)) {
                      setNameHint(null)
                      setNameError(t("A name needs 3 to 16 letters, digits or _."))
                      return
                    }
                    setBusy("save")
                    void setOfflineName(trimmed)
                      .then(() => {
                        setNameError(null)
                        setNameHint(t("Saved."))
                        onRefresh()
                      })
                      .catch((error: Error) =>
                        setNameError(
                          /name/i.test(error.message)
                            ? t("A name needs 3 to 16 letters, digits or _.")
                            : friendlyError(error, t)
                        )
                      )
                      .finally(() => setBusy(null))
                  }}
                  style={{ borderRadius: 999, height: 38, marginTop: 20, paddingLeft: 18, paddingRight: 18 }}
                >
                  {t("Save")}
                </Button>
              </div>
            </div>

            <div style={{ height: 1, backgroundColor: theme.colors.border }} />

            <SectionField label={t("Appearance")} detail={t("Dark or light.")}>
              <div style={{ display: "flex", flexDirection: narrow ? "column" : "row", gap: 10 }}>
                {[
                  { key: "dark", label: t("Night"), detail: t("Ember and basalt"), colors: ["#09080A", "#FF8A4C", "#F7EFE7"] },
                  { key: "light", label: t("Ember"), detail: t("Warm paper, ember ink"), colors: ["#FBF5EE", "#B7410E", "#1D1510"] },
                ].map((option) => {
                  const active = (option.key === "dark") === dark
                  return (
                    <div
                      key={option.key}
                      onClick={() => {
                        if (!active) onToggleTheme()
                      }}
                      style={{
                        display: "flex",
                        flexDirection: "row",
                        alignItems: "center",
                        gap: 12,
                        width: narrow ? "100%" : 240,
                        padding: 12,
                        borderRadius: 16,
                        borderWidth: 1,
                        borderStyle: "solid",
                        borderColor: active ? theme.colors.primary : theme.colors.border,
                        backgroundColor: active ? "#FF8A4C14" : "transparent",
                        cursor: active ? "default" : "pointer",
                        hover: active ? {} : { borderColor: theme.colors.input },
                      }}
                    >
                      <div style={{ display: "flex", flexDirection: "row", width: 46, height: 34, borderRadius: 10, overflow: "hidden" }}>
                        {option.colors.map((color) => (
                          <div key={color} style={{ flexGrow: 1, backgroundColor: color }} />
                        ))}
                      </div>
                      <div style={{ display: "flex", flexDirection: "column", gap: 2, flexGrow: 1 }}>
                        <Text size="xs" weight="medium">
                          {option.label}
                        </Text>
                        <Text mono size="xs" tone="muted">
                          {option.detail}
                        </Text>
                      </div>
                      <Icon
                        name={active ? "check" : "circle"}
                        size={13}
                        color={active ? theme.colors.primary : theme.colors.mutedForeground}
                      />
                    </div>
                  )
                })}
              </div>
            </SectionField>

            <SectionField label={t("Language")}>
              <div style={{ display: "flex", flexDirection: "row", gap: 6, flexWrap: "wrap" }}>
                {[
                  ["en", "English"],
                  ["ru", "Русский"],
                  ["system", t("System")],
                ].map(([value, label]) => (
                  <Button
                    key={value}
                    size="sm"
                    variant={data.language === value ? "secondary" : "ghost"}
                    onClick={() => onLanguage(value as "en" | "ru" | "system")}
                    style={{ borderRadius: 999, height: 32, paddingLeft: 14, paddingRight: 14 }}
                  >
                    {label}
                  </Button>
                ))}
              </div>
            </SectionField>

            <SectionField label={t("While playing")} detail={t("What this window does when the game starts.")}>
              <div style={{ display: "flex", flexDirection: "row", gap: 6, flexWrap: "wrap" }}>
                {MODES.map((mode) => (
                  <Button
                    key={mode.value}
                    size="sm"
                    variant={data.whilePlaying === mode.value ? "secondary" : "ghost"}
                    onClick={() => {
                      void setWhilePlaying(mode.value)
                        .then(() => onRefresh())
                        .catch((error: Error) => toast({ tone: "error", title: friendlyError(error, t) }))
                    }}
                    style={{ borderRadius: 999, height: 32, paddingLeft: 14, paddingRight: 14 }}
                  >
                    {t(mode.label)}
                  </Button>
                ))}
              </div>
            </SectionField>

          </>
        ) : null}

        {tab === "game" ? (
          <>
            <div style={{ display: "flex", flexDirection: "row", alignItems: "flex-start", gap: 12 }}>
              <div style={{ display: "flex", flexDirection: "column", gap: 4, flexGrow: 1 }}>
                <Text size="xl" weight="semibold">
                  {`Minecraft ${files.version}`}
                </Text>
                <Text size="sm" tone="muted">
                  {files.installed && data.prepared
                    ? t("Ready to play.")
                    : !files.installed
                      ? t("Download Minecraft 26.3 once.")
                      : t("The files are here. Prepare the resources.")}
                </Text>
              </div>
              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: 6,
                  height: 28,
                  paddingLeft: 10,
                  paddingRight: 10,
                  borderRadius: 999,
                  backgroundColor: files.installed && data.prepared ? "#7FD8A41A" : "#FFFFFF0D",
                }}
              >
                <div
                  style={{
                    width: 6,
                    height: 6,
                    borderRadius: 3,
                    backgroundColor: files.installed && data.prepared ? theme.colors.success : "#D8B26A",
                  }}
                />
                <Text mono size="xs" tone="muted">
                  {files.installed && data.prepared ? t("ready") : t("needs attention")}
                </Text>
              </div>
            </div>

            <Meter value={progress ? progress.done : done} total={progress ? progress.total : total} />
            <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
              <Text mono size="xs" tone="muted">
                {progress
                  ? t("Downloading…")
                  : files.installed
                    ? t("all files verified")
                    : files.manifest
                      ? `${megabytes(files.missingBytes)} ${t("to download")}`
                      : t("not downloaded yet")}
              </Text>
              <Text mono size="xs" tone="muted">
                {`${progress ? (progress.total > 0 ? Math.round((progress.done / progress.total) * 100) : 0) : percent}%`}
              </Text>
            </div>

            <Button
              variant="primary"
              loading={busy !== null}
              onClick={() => void (files.installed && data.prepared ? download() : prepareAll())}
              style={{ borderRadius: 999, height: 38, paddingLeft: 18, paddingRight: 18, alignSelf: "flex-start" }}
            >
              <Icon
                name={files.installed && data.prepared ? "download" : files.installed ? "refresh" : "download"}
                size={14}
                color={theme.colors.primaryForeground}
              />
              {files.installed && data.prepared
                ? t("Verify files")
                : files.installed
                  ? t("Prepare resources")
                  : t("Download and prepare")}
            </Button>

            <div
              role="button"
              tabIndex={0}
              onClick={() => setDetails((value) => !value)}
              style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 6, cursor: "pointer", alignSelf: "flex-start" }}
            >
              <Icon name={details ? "chevronUp" : "chevronDown"} size={13} color={theme.colors.mutedForeground} />
              <Text size="xs" tone="muted">
                {details ? t("Hide details") : t("Show details")}
              </Text>
            </div>

            {details ? (
              <div style={{ display: "flex", flexDirection: "column", gap: 6, padding: 14, borderRadius: 14, backgroundColor: theme.colors.muted }}>
                {[
                  [t("Folder"), files.path],
                  [t("Version manifest"), files.manifestSha1 ?? t("unknown")],
                  [t("Client"), files.client ? t("ready") : t("missing")],
                  [t("Libraries"), `${files.libraries.done} / ${files.libraries.total}`],
                  [t("Assets"), `${files.assets.done} / ${files.assets.total}`],
                ].map(([label, value]) => (
                  <div key={label} style={{ display: "flex", flexDirection: "row", gap: 12 }}>
                    <Text mono size="xs" tone="muted" style={{ width: 130, flexShrink: 0 }}>
                      {label}
                    </Text>
                    <Text mono size="xs" style={{ flexGrow: 1 }}>
                      {value}
                    </Text>
                  </div>
                ))}
                <Text size="xs" tone="muted">
                  {t("Every file is checked against Mojang's sha1. A stopped download resumes where it stopped.")}
                </Text>
                {log.length > 0 ? (
                  <div style={{ display: "flex", flexDirection: "column", gap: 3, marginTop: 4 }}>
                    {log.map((line, index) => (
                      <Text key={index} mono size="xs" tone="muted" truncate>
                        {line}
                      </Text>
                    ))}
                  </div>
                ) : null}
              </div>
            ) : null}

          </>
        ) : null}

        {tab === "builds" ? (
          <>
            <div style={{ display: "flex", flexDirection: "row", alignItems: "flex-end", gap: 12 }}>
              <div style={{ display: "flex", flexDirection: "column", gap: 4, flexGrow: 1 }}>
                <Text size="xl" weight="semibold">
                  {t("Launcher builds")}
                </Text>
                <Text size="sm" tone="muted">
                  {t("Versions of Scoria installed on this machine.")}
                </Text>
              </div>
              <Button
                variant="secondary"
                loading={versionBusy === "latest"}
                onClick={() => {
                  setVersionBusy("latest")
                  void installLatest()
                    .then(() => {
                      toast({ tone: "success", title: t("Installed the latest build.") })
                      refreshVersions()
                      onRefresh()
                    })
                    .catch((error: Error) => toast({ tone: "error", title: friendlyError(error, t) }))
                    .finally(() => setVersionBusy(null))
                }}
                style={{ borderRadius: 999, height: 34, paddingLeft: 14, paddingRight: 14 }}
              >
                <Icon name="download" size={13} color={theme.colors.foreground} />
                {t("Install latest")}
              </Button>
            </div>

            <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
              {versionsLoading && installed.length === 0 ? (
                <SkeletonList rows={2} height={56} />
              ) : installed.length === 0 ? (
                <EmptyState
                  compact
                  icon="layers"
                  title={t("No builds installed.")}
                  description={t("Install the latest build to start.")}
                />
              ) : (
                installed.map((version) => (
                  <div
                    key={version.tag}
                    style={{
                      display: "flex",
                      flexDirection: narrow ? "column" : "row",
                      alignItems: narrow ? "stretch" : "center",
                      gap: 12,
                      padding: 12,
                      borderRadius: 14,
                      backgroundColor: theme.colors.muted,
                    }}
                  >
                    <Text mono size="sm" style={{ flexGrow: 1 }}>
                      {version.tag}
                    </Text>
                    <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 6 }}>
                      <div
                        style={{
                          width: 5,
                          height: 5,
                          borderRadius: 3,
                          backgroundColor: version.prepared ? theme.colors.success : theme.colors.mutedForeground,
                        }}
                      />
                      <Text mono size="xs" tone="muted">
                        {version.prepared ? t("prepared") : t("needs preparing")}
                      </Text>
                    </div>
                    {selected === version.tag ? (
                      <Text mono size="xs" style={{ color: theme.colors.primary }}>
                        {t("selected")}
                      </Text>
                    ) : (
                      <Button
                        size="sm"
                        variant="ghost"
                        loading={versionBusy === version.tag}
                        onClick={() => {
                          setVersionBusy(version.tag)
                          void selectVersion(version.tag)
                            .then(() => {
                              refreshVersions()
                              onRefresh()
                            })
                            .catch((error: Error) => toast({ tone: "error", title: friendlyError(error, t) }))
                            .finally(() => setVersionBusy(null))
                        }}
                        style={{ borderRadius: 999, height: 28, paddingLeft: 12, paddingRight: 12 }}
                      >
                        {t("Use")}
                      </Button>
                    )}
                    <Button
                      size="sm"
                      variant="ghost"
                      onClick={async () => {
                        const ok = await confirm({
                          title: t("Remove build?"),
                          description: t("The build's files are deleted from this machine. Worlds are kept."),
                          confirmLabel: t("Remove"),
                          tone: "destructive",
                        })
                        if (!ok) return
                        try {
                          await removeVersion(version.tag)
                          toast({ tone: "success", title: t("Build removed."), description: version.tag })
                          refreshVersions()
                          onRefresh()
                        } catch (error) {
                          toast({ tone: "error", title: friendlyError(error, t) })
                        }
                      }}
                      style={{ borderRadius: 999, height: 28, paddingLeft: 12, paddingRight: 12 }}
                    >
                      {t("Remove")}
                    </Button>
                  </div>
                ))
              )}
            </div>

          </>
        ) : null}

        {tab === "about" ? (
          <>
            <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
              <Text size="xl" weight="semibold">
                {t("About")}
              </Text>
              <Text size="sm" tone="muted">
                {t("No account, no session, no telemetry.")}
              </Text>
            </div>
            <div style={{ display: "flex", flexDirection: "column", borderRadius: 16, overflow: "hidden", borderWidth: 1, borderStyle: "solid", borderColor: theme.colors.border }}>
              {[
                [t("Launcher"), `${LAUNCHER_VERSION} · ${t(LAUNCHER_CHANNEL)}`],
                [t("Engine"), data.version],
                [t("Minecraft"), data.minecraft.version],
                [t("Profile"), data.offlineName],
              ].map(([label, value], index) => (
                <div
                  key={label}
                  style={{
                    display: "flex",
                    flexDirection: "row",
                    alignItems: "center",
                    paddingLeft: 14,
                    paddingRight: 14,
                    paddingTop: 11,
                    paddingBottom: 11,
                    borderTopWidth: index === 0 ? 0 : 1,
                    borderStyle: "solid",
                    borderColor: theme.colors.border,
                  }}
                >
                  <Text size="xs" tone="muted" style={{ flexGrow: 1 }}>
                    {label}
                  </Text>
                  <Text mono size="xs">
                    {value}
                  </Text>
                </div>
              ))}
            </div>
            <Text size="xs" tone="muted">
              {t("Files come from Mojang only when you ask for them.")}
            </Text>
            <Text size="xs" tone="muted">
              {t("Typeface: Medeo Sans and Medeo Mono by DNFDEVS, SIL OFL.")}
            </Text>
            <Text size="xs" tone="muted">
              {t("Engine: Horizon, MIT.")}
            </Text>
          </>
        ) : null}
      </div>
      </Sheet>
    </div>
  )
}

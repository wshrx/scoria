import { createContext, useContext, type ReactNode } from "react"

export type Lang = "en" | "ru"

/**
 * The UI's own strings. Engine data (world names, modes, errors) is shown as it comes.
 * English is the key: a missing translation falls back to it instead of showing a key.
 */
const RU: Record<string, string> = {
  Home: "Главная",
  Instances: "Инстансы",
  Servers: "Серверы",
  Settings: "Настройки",
  Overview: "Обзор",
  engine: "движок",
  preview: "превью",

  Play: "Играть",
  Prepare: "Подготовить",
  "Playing as": "Играем как",
  offline: "оффлайн",
  Worlds: "Миры",
  "No worlds yet. Play and create one.": "Миров пока нет. Запустите игру и создайте мир.",
  PROFILE: "ПРОФИЛЬ",
  FILES: "ФАЙЛЫ",
  ENGINE: "ДВИЖОК",
  "NO ACCOUNT": "БЕЗ АККАУНТА",
  "No Microsoft account. Online-mode servers are out of scope.":
    "Без Microsoft. Online-mode серверы не поддерживаются.",
  "LAN, offline-mode servers and Play with friends work. The game never receives a session.":
    "LAN, offline-mode серверы и Play with friends работают. Игра не получает сессию.",
  version: "версия",
  branch: "ветка",
  resources: "ресурсы",
  mode: "режим",
  prepared: "подготовлено",
  missing: "нет",
  live: "живой",
  "Your own Minecraft files, prepared locally. No account, no session, no telemetry.":
    "Свои файлы Minecraft, подготовленные локально. Без аккаунта, сессии и телеметрии.",

  "Separate game folders, each with its own worlds, options and packs.":
    "Отдельные папки игры: свои миры, настройки и паки.",
  "New instance": "Новый инстанс",
  Create: "Создать",
  Cancel: "Отмена",
  INSTANCE: "ИНСТАНС",
  MINECRAFT: "MINECRAFT",
  KIND: "ТИП",
  STATUS: "СТАТУС",
  selected: "выбран",
  ready: "готово",
  prepare: "подготовить",
  "DETECTED INSTALLS": "НАЙДЕННЫЕ УСТАНОВКИ",
  "The official launcher, Prism, Freesm, MultiMC and the Modrinth App are found automatically. Nothing is downloaded; preparation reads the files you already have.":
    "Официальный лаунчер, Prism, Freesm, MultiMC и Modrinth App находятся автоматически. Ничего не скачивается: подготовка читает уже имеющиеся файлы.",
  "A name, 1 to 40 characters": "Имя, от 1 до 40 символов",

  "Saved in this instance's servers.dat; offline-mode servers only.":
    "Хранятся в servers.dat инстанса; только offline-mode серверы.",
  "Add server": "Добавить сервер",
  "NEW SERVER": "НОВЫЙ СЕРВЕР",
  Name: "Имя",
  Add: "Добавить",
  Join: "Подключиться",
  Remove: "Удалить",
  Confirm: "Подтвердить",
  "No servers yet. Add an offline-mode server, or open a world to LAN.":
    "Серверов пока нет. Добавьте offline-mode сервер или откройте мир в LAN.",
  "ONLINE-MODE": "ONLINE-MODE",
  "Scoria has no Microsoft session, so online-mode servers refuse it. Use an offline-mode server, Open to LAN, or Play with friends.":
    "У Scoria нет сессии Microsoft, поэтому online-mode серверы её отклоняют. Используйте offline-mode сервер, Open to LAN или Play with friends.",

  "OFFLINE PROFILE": "ОФФЛАЙН-ПРОФИЛЬ",
  "Player name": "Имя игрока",
  Save: "Сохранить",
  "3 to 16 letters, digits or _. Kept in accounts.json; no token leaves the machine.":
    "3–16 букв, цифр или _. Хранится в accounts.json; токены никуда не уходят.",
  "WHILE PLAYING": "ПОКА ИГРАЕМ",
  "What this window does when the game starts.": "Что делает это окно при запуске игры.",
  Stay: "Остаётся",
  Minimize: "Свернуть",
  Hide: "Скрыть",
  "Dark appearance": "Тёмное оформление",
  "Basalt and ember; the light theme is warm paper.":
    "Базальт и ember; светлая тема — тёплая бумага.",
  LAUNCHER: "ЛАУНЧЕР",
  VERSIONS: "ВЕРСИИ",
  "Install latest": "Установить последнюю",
  Select: "Выбрать",
  "No versions installed.": "Версии не установлены.",
  DATA: "ДАННЫЕ",
  "No telemetry, no consent screen, no analytics. The launcher talks to GitHub only when you ask for a build.":
    "Без телеметрии, экрана согласия и аналитики. Лаунчер обращается к GitHub только когда вы просите сборку.",
  LANGUAGE: "ЯЗЫК",
  System: "Системный",
}

const LangContext = createContext<Lang>("en")

export function LangProvider({ lang, children }: { lang: Lang; children?: ReactNode }) {
  return <LangContext.Provider value={lang}>{children}</LangContext.Provider>
}

export function useLang(): Lang {
  return useContext(LangContext)
}

export function useT(): (text: string) => string {
  const lang = useLang()
  return (text: string) => (lang === "ru" ? RU[text] ?? text : text)
}

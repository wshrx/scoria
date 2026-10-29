import { createContext, useContext, type ReactNode } from "react"

export type Lang = "en" | "ru"

/**
 * The UI's own strings. Engine data (world names, modes, errors) is shown as it comes.
 * English is the key: a missing translation falls back to it instead of showing a key.
 */
export const RU: Record<string, string> = {
  "Build removed.": "Сборка удалена.",
  "Instance created.": "Инстанс создан.",
  "Playing": "Игра идёт",
  "Typeface: Medeo Sans and Medeo Mono by DNFDEVS, SIL OFL.":
    "Шрифты: Medeo Sans и Medeo Mono от DNFDEVS, SIL OFL.",
  "Engine: Horizon, MIT.": "Движок: Horizon, MIT.",
  "The game is already running. Close its window to play again.":
    "Игра уже запущена. Закройте её окно, чтобы запустить снова.",
  "Dismiss": "Закрыть",
  "Game files are ready.": "Файлы игры готовы.",
  "Install the latest build to start.": "Установите последнюю сборку, чтобы начать.",
  "Installed mods and resource packs appear here.": "Установленные моды и ресурспаки появятся здесь.",
  "It is removed from the game folder.": "Он будет удалён из папки игры.",
  "No access to the folder. Choose another one.": "Нет доступа к папке. Выберите другую.",
  "No builds installed.": "Сборки не установлены.",
  "Not enough disk space.": "Не хватает места на диске.",
  "Remove build?": "Удалить сборку?",
  "Remove mod?": "Удалить мод?",
  "Remove server?": "Удалить сервер?",
  "Resources prepared.": "Ресурсы подготовлены.",
  "Retry": "Повторить",
  "Server added.": "Сервер добавлен.",
  "Server removed.": "Сервер удалён.",
  "Something went wrong. Try again.": "Что-то пошло не так. Попробуйте ещё раз.",
  "The build's files are deleted from this machine. Worlds are kept.":
    "Файлы сборки будут удалены с этого компьютера. Миры сохранятся.",
  "The connection dropped. Check your network and try again.":
    "Соединение пропало. Проверьте сеть и попробуйте снова.",
  "The console has the full error.": "Полная ошибка есть в консоли.",
  "The download is corrupted. Try again.": "Файл скачался повреждённым. Попробуйте ещё раз.",
  "The file was not found on the server.": "Файл не найден на сервере.",
  "The launcher hit an unexpected error.": "Лаунчер столкнулся с неожиданной ошибкой.",
  "The server is removed from this list. Its address and name are forgotten.":
    "Сервер исчезнет из списка. Имя и адрес будут забыты.",
  "The service is rate-limiting requests. Try again in a minute.":
    "Сервис временно ограничил запросы. Попробуйте через минуту.",
  "Try another search or install from Modrinth.": "Попробуйте другой запрос или установите мод с Modrinth.",
  Play: "Играть",
  Instances: "Инстансы",
  "Game files": "Файлы игры",
  Servers: "Серверы",
  Settings: "Настройки",
  engine: "движок",
  preview: "превью",
  offline: "оффлайн",
  "Toggle theme": "Сменить тему",

  Mods: "Моды",
  "Resource packs": "Ресурспаки",
  "Resource pack": "Ресурспак",
  Mod: "Мод",
  "Modrinth mods and resource packs. Java mods are ported; packs install as they are.":
    "Моды и ресурспаки с Modrinth. Java-моды портируются, ресурспаки ставятся как есть.",
  "Search Modrinth": "Поиск в Modrinth",
  Search: "Найти",
  "Searching…": "Ищу…",
  "Nothing found.": "Ничего не найдено.",
  Install: "Установить",
  "Installing…": "Установка…",
  Installed: "Установлено",
  "Nothing installed from Modrinth yet.": "С Modrinth пока ничего не установлено.",
  Works: "Работает",
  Partly: "Частично",
  "Doesn't run": "Не запустится",
  "Installed.": "Установлено.",
  "Ready to play as {name}.": "Готово к игре: {name}.",
  "Current instance": "Текущий инстанс",
  "Your worlds, mods and servers in one place.":
    "Ваши миры, моды и серверы в одном месте.",
  "Profile, appearance, game files and builds in one place.":
    "Профиль, оформление, файлы игры и сборки в одном месте.",
  "Open Mods": "Открыть моды",
  "Open Servers": "Открыть серверы",
  results: "результатов",
  "Modrinth mods and resource packs.": "Моды и ресурспаки с Modrinth.",
  "Offline-mode servers and LAN.": "Offline-mode серверы и LAN.",
  "Player name": "Имя игрока",
  "A name needs 3 to 16 letters, digits or _.":
    "Имя — от 3 до 16 букв, цифр или _.",
  beta: "бета",
  "Removed.": "Удалено.",
  Status: "Статус",
  General: "Общие",
  Builds: "Сборки",
  About: "О программе",
  Configure: "Настроить",
  Continue: "Продолжить",
  "Download 26.3": "Скачать 26.3",
  Prepare: "Подготовить",
  "Download and prepare": "Скачать и подготовить",
  "The connection dropped. Press the button again to continue from where it stopped.":
    "Связь оборвалась. Нажмите кнопку ещё раз — уже скачанное сохранится.",
  "The download stopped. Press the button again to continue.":
    "Загрузка остановилась. Нажмите кнопку ещё раз, чтобы продолжить.",
  "Verify files": "Проверить файлы",
  "to download": "скачать",
  "needs game files": "нужны файлы игры",
  "needs attention": "нужно внимание",
  "Working…": "Работаем…",
  "Ready to play.": "Готово к игре.",
  "Ready to play. Your worlds and settings stay on this machine.":
    "Готово к игре. Миры и настройки остаются на этой машине.",
  "Minecraft 26.3 is not here yet. Download it once and it is yours.":
    "Minecraft 26.3 ещё нет на этой машине. Скачайте один раз — и он ваш.",
  "The files are here. One more step prepares the resources.":
    "Файлы на месте. Остался один шаг — подготовить ресурсы.",
  "The files are here. Prepare the resources.":
    "Файлы на месте. Подготовьте ресурсы.",
  "Download the pinned version from Mojang.":
    "Скачайте закреплённую версию с Mojang.",
  "Every file is checked against Mojang's sha1. A stopped download resumes where it stopped.":
    "Каждый файл сверяется с sha1 от Mojang. Прерванная загрузка продолжается с того же места.",
  "Show details": "Показать детали",
  "Hide details": "Скрыть детали",
  "not downloaded yet": "ещё не скачано",
  "Offline-mode servers, saved in this instance.":
    "Offline-mode серверы, сохранённые в этом инстансе.",
  "Online-mode servers need a Microsoft session. Scoria has none, so use offline-mode servers or LAN.":
    "Для online-mode нужна сессия Microsoft. У Scoria её нет: используйте offline-mode или LAN.",
  "Two surfaces, the same violet light.": "Две поверхности, один фиолетовый свет.",
  "Dark or light.": "Тёмная или светлая.",
  "Downloading…": "Загрузка…",
  "Download Minecraft 26.3 once.": "Скачайте Minecraft 26.3 один раз.",
  "No account, no session, no telemetry.": "Без аккаунта, сессии и телеметрии.",
  "Files come from Mojang only when you ask for them.": "Файлы скачиваются с Mojang только когда вы об этом просите.",
  Night: "Ночь",
  Lilac: "Сирень",
  Ember: "Янтарь",
  "Indigo and iris": "Индиго и ирис",
  "Ember and basalt": "Янтарь и базальт",
  "Warm paper, ember ink": "Тёплая бумага, янтарные чернила",
  "Versions of Scoria installed on this machine.": "Установленные на этой машине версии Scoria.",
  "A local-first launcher: no account, no session, no telemetry.":
    "Лаунчер, который живёт локально: без аккаунта, сессии и телеметрии.",
  Saved: "Сохранено",
  "Saved.": "Сохранено.",
  Client: "Клиент",
  Maximize: "Развернуть",
  Close: "Закрыть",
  "Minecraft Java Edition": "Minecraft Java Edition",
  "Your worlds, lit from below.": "Ваши миры, освещённые снизу.",
  "Minecraft 26.3, prepared from your own files and run natively. No account, no session, nothing leaves this machine.":
    "Minecraft 26.3 из ваших файлов, запуск нативный. Без аккаунта, сессии и отправки данных.",
  "Ready to play": "Готово к игре",
  "Minecraft files are not on this machine yet": "Файлов Minecraft на этой машине ещё нет",
  "Game files ready. Resources need preparing": "Файлы игры готовы. Ресурсы нужно подготовить",
  "Prepare resources": "Подготовить ресурсы",
  "Download game files": "Скачать файлы игры",
  "Preparing…": "Подготовка…",
  files: "файлов",
  saved: "миров",
  "A world opens straight into the game": "Мир открывается сразу в игре",
  "No worlds yet.": "Миров пока нет.",
  "Play once, create a world, and it will appear here.":
    "Запустите игру, создайте мир — он появится здесь.",
  open: "открыть",
  "This install": "Эта установка",
  "resources prepared": "ресурсы готовы",
  "resources not prepared": "ресурсы не готовы",
  "Client jar": "Клиентский jar",
  Libraries: "Библиотеки",
  Assets: "Ассеты",
  Resources: "Ресурсы",
  prepared: "готово",
  "not prepared": "не готово",
  "all files verified": "все файлы проверены",
  "to fetch": "скачать",
  "Versions and files from piston-meta.mojang.com": "Версии и файлы с piston-meta.mojang.com",
  "Verify and prepare": "Проверить и подготовить",
  Profile: "Профиль",
  Survival: "Выживание",
  Creative: "Творческий",
  Adventure: "Приключение",
  Spectator: "Наблюдатель",
  Hardcore: "Хардкор",
  Save: "Сохранить",
  Engine: "Движок",
  release: "релиз",
  modpack: "модпак",
  "ported mod": "порт мода",
  Launcher: "Лаунчер",
  Account: "Аккаунт",
  "offline only": "только оффлайн",
  World: "Мир",
  Worlds: "Миры",
  Minecraft: "Minecraft",
  Instance: "Инстанс",
  "Game files ready": "Файлы игры готовы",
  ready: "готово",
  "not fetched": "не скачано",
  missing: "нет",
  verified: "проверено",
  installed: "установлено",
  incomplete: "неполно",
  unknown: "неизвестно",
  Download: "Скачать",
  "Verify and repair": "Проверить и восстановить",
  "Every file is verified against Mojang's sha1.": "Каждый файл сверяется с sha1 от Mojang.",
  "Every file the version needs is present.": "Все нужные версии файлы на месте.",
  "still to fetch.": "ещё скачать.",
  "No files fetched yet.": "Файлы ещё не скачаны.",
  "Where they live": "Где они лежат",
  Folder: "Папка",
  "Version manifest": "Манифест версии",
  "Version file": "Файл версии",
  "from piston-meta": "из piston-meta",
  "Mojang's sha1 is checked for every file. A download stopped half way keeps what it finished; the next one fetches only the rest.":
    "Каждый файл сверяется с sha1 от Mojang. Прерванная загрузка сохраняет готовое; следующая докачает только остальное.",
  "Already on disk": "Уже на диске",
  "Launcher builds": "Сборки лаунчера",
  "Install latest": "Установить последнюю",
  "Installed the latest build.": "Последняя сборка установлена.",
  "No versions installed.": "Версии не установлены.",
  "needs preparing": "нужна подготовка",
  selected: "выбран",
  Use: "Использовать",
  Remove: "Удалить",
  Confirm: "Подтвердить",
  Cancel: "Отмена",
  "New instance": "Новый инстанс",
  Create: "Создать",
  "A name, 1 to 40 characters": "Имя, от 1 до 40 символов",
  "Separate game folders, each with its own worlds, options and packs.":
    "Отдельные папки игры: свои миры, настройки и паки.",
  "A new folder starts empty. Use it for a pack, a port, or a clean survival world.":
    "Новая папка пустая. Для модпака, порта или чистого выживания.",
  "in use": "используется",
  "Scoria also reads the launchers already on this machine":
    "Scoria также читает лаунчеры, уже установленные на машине",
  "The official launcher, Prism, Freesm, MultiMC, the Modrinth App and ATLauncher are found automatically. If none has Minecraft 26.3, Scoria downloads it from Mojang into its own folder.":
    "Официальный лаунчер, Prism, Freesm, MultiMC, Modrinth App и ATLauncher находятся автоматически. Если ни в одном нет Minecraft 26.3, Scoria скачает его с Mojang в свою папку.",
  "Scoria runs Minecraft 26.3. The launcher reads an install you already have, or fetches the pinned version straight from Mojang's metadata.":
    "Scoria работает с Minecraft 26.3. Лаунчер читает уже установленную игру или скачивает закреплённую версию прямо из метаданных Mojang.",
  "Saved in this instance's servers.dat; offline-mode servers only.":
    "Хранятся в servers.dat инстанса; только offline-mode серверы.",
  "Add server": "Добавить сервер",
  "No servers yet.": "Серверов пока нет.",
  "Add an offline-mode server, or open a world to LAN.":
    "Добавьте offline-mode сервер или откройте мир в LAN.",
  "A name is only a label. The address is what the game connects to.":
    "Имя — просто метка. Игра подключается по адресу.",
  Name: "Имя",
  Add: "Добавить",
  Join: "Подключиться",
  "offline-mode": "offline-mode",
  "Online-mode servers need a Microsoft session": "Для online-mode нужна сессия Microsoft",
  "Scoria has no Microsoft session, so online-mode servers refuse it. Use an offline-mode server, Open to LAN, or Play with friends.":
    "У Scoria нет сессии Microsoft, поэтому online-mode серверы её отклоняют. Используйте offline-mode сервер, Open to LAN или Play with friends.",
  "The launcher keeps everything on this machine. These are the only switches there are.":
    "Всё остаётся на этой машине. Вот и все переключатели.",
  "The name the game shows in single player and on LAN.":
    "Имя, которое игра показывает в одиночной игре и LAN.",
  "Saved. The next game starts with this name.": "Сохранено. Следующая игра начнётся с этим именем.",
  "3 to 16 letters, digits or _. Kept in accounts.json; no token leaves the machine.":
    "3–16 букв, цифр или _. Хранится в accounts.json; токены никуда не уходят.",
  "offline profile": "оффлайн-профиль",
  Appearance: "Оформление",
  "Two surfaces, the same blue fire.": "Две поверхности, один синий огонь.",
  Basalt: "Базальт",
  Paper: "Бумага",
  "Cold rock, bone text": "Холодный камень, костяной текст",
  "Warm paper, ink": "Тёплая бумага, чернила",
  "While playing": "Пока играем",
  "What this window does when the game starts.": "Что делает это окно при запуске игры.",
  Stay: "Остаётся",
  Minimize: "Свернуть",
  Hide: "Скрыть",
  Language: "Язык",
  "The launcher's own words. Game text comes from your packs.":
    "Слова самого лаунчера. Текст игры берётся из ваших паков.",
  System: "Системный",
  "This machine": "Эта машина",
  "Read-only facts about the build that is running.": "Факты о запущенной сборке.",
  connected: "подключён",
  "preview snapshot": "превью-снимок",
  basalt: "базальт",
  paper: "бумага",
  "No telemetry, no consent screen, no analytics. The launcher talks to Mojang and GitHub only when you ask for files or a build.":
    "Без телеметрии, экрана согласия и аналитики. Лаунчер обращается к Mojang и GitHub только когда вы просите файлы или сборку.",
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

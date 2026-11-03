//! The window's own strings. The engine's tables are for the CLI; the UI keeps its own small
//! dictionary so a new label does not have to be a CLI string.
use scoria_core::i18n::Language;

pub fn tr(language: Language, english: &'static str) -> &'static str {
    match language {
        Language::Russian => russian(english),
        _ => english,
    }
}

fn russian(english: &'static str) -> &'static str {
    match english {
        "Play" => "Играть",
        "Dark" => "Тёмная",
        "Light" => "Светлая",
        "System" => "Система",
        "Preparing…" => "Подготовка…",
        "Works" => "Работает",
        "Partly" => "Частично",
        "Doesn't run" => "Не работает",
        "Playing" => "Играет",
        "Mods" => "Моды",
        "Servers" => "Серверы",
        "Settings" => "Настройки",
        "Download 26.3" => "Скачать 26.3",
        "Prepared" => "Готово",
        "Not prepared" => "Не готово",
        "Not installed" => "Не установлено",
        "What Download does" => "Что делает загрузка",
        "Fetches Minecraft 26.3 and the Horizon engine from Mojang." => {
            "Скачивает Minecraft 26.3 и движок Horizon с серверов Mojang."
        }
        "One download, then the game starts offline." => {
            "Одна загрузка, дальше игра запускается офлайн."
        }
        "Worlds, mods and settings stay in this instance." => {
            "Миры, моды и настройки остаются в этом профиле."
        }
        "Cancel" => "Отмена",
        "Try again" => "Повторить",
        "The game did not start" => "Игра не запустилась",
        "Your worlds" => "Ваши миры",
        "Creative" => "Творческий",
        "Hardcore" => "Хардкор",
        "Adventure" => "Приключение",
        "Survival" => "Выживание",
        "No version installed. Download 26.3 to install the engine." => {
            "Версия не установлена. Скачайте 26.3, чтобы установить движок."
        }
        "Offline profile. No Microsoft account." => {
            "Офлайн-профиль. Без аккаунта Microsoft."
        }
        "The game is running in another window." => {
            "Игра запущена в другом окне."
        }
        "Could not load the catalogue" => "Не удалось загрузить каталог",
        "No servers yet" => "Серверов пока нет",
        "Could not load the servers" => "Не удалось загрузить серверы",
        "Join" => "Войти",
        "Search" => "Поиск",
        "Search mods" => "Поиск модов",
        "Install" => "Установить",
        "Installing…" => "Установка…",
        "Not installable" => "Не устанавливается",
        "Nothing yet" => "Пока пусто",
        "Search the registry or type a mod name." => "Найдите мод в реестре или введите название.",
        "Add" => "Добавить",
        "Remove" => "Удалить",
        "Name" => "Имя",
        "host:port" => "адрес:порт",
        "No saved servers yet. Add the address of an offline-mode server." => {
            "Серверов пока нет. Добавьте адрес offline-mode сервера."
        }
        "Saved servers for this instance. LAN worlds appear inside the game." => {
            "Сохранённые серверы этого профиля. LAN-миры видны в игре."
        }
        "Appearance" => "Оформление",
        "Language" => "Язык",
        "Nickname" => "Никнейм",
        "Save" => "Сохранить",
        "Your offline name" => "Ваше офлайн-имя",
        "No Microsoft account: this name is what servers see." => {
            "Без аккаунта Microsoft: это имя видят серверы."
        }
        "3 to 16 letters, digits or _ (any script)." => "3–16 букв, цифр или _ (любой алфавит).",
        "Appearance and language are saved to the launcher's settings." => {
            "Оформление и язык сохраняются в настройках лаунчера."
        }
        "Horizon's registry first, then Modrinth. Every result says how it runs on Scoria." => {
            "Сначала реестр Horizon, затем Modrinth. У каждого результата видно, как он работает в Scoria."
        }
        other => other,
    }
}

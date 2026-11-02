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
        "Playing" => "Играет",
        "Mods" => "Моды",
        "Servers" => "Серверы",
        "Settings" => "Настройки",
        "Download 26.3" => "Скачать 26.3",
        "Prepared" => "Готово",
        "Not prepared" => "Не готово",
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
        "Offline name" => "Офлайн-имя",
        "Save" => "Сохранить",
        "Your offline name" => "Ваше офлайн-имя",
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

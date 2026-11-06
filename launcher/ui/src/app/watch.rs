//! Watch the selected instance and the launcher's data folder, so a world, mod, shader pack or
//! `servers.json` written by the game (or by a developer's build) appears without a restart.
//!
//! The watcher itself only reports "something changed"; `Shell` debounces those events and asks
//! the engine for a fresh snapshot.
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

/// Keeps the watcher alive for as long as the window is open.
pub struct Keep {
    _watcher: RecommendedWatcher,
}

/// Start watching `home` and `data`. Returns the handle and the channel of changes.
pub fn start(home: &Path, data: &Path) -> Option<(Keep, Receiver<()>)> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
        if event.is_ok() {
            // A full channel means the window is already about to refresh; dropping the extra
            // event is exactly the debounce we want.
            let _ = tx.send(());
        }
    })
    .ok()?;

    for path in watched_paths(home, data) {
        if path.is_dir() {
            let _ = watcher.watch(&path, RecursiveMode::Recursive);
        } else if path.is_file() {
            let _ = watcher.watch(&path, RecursiveMode::NonRecursive);
        }
    }
    Some((Keep { _watcher: watcher }, rx))
}

/// The folders and files that change what the window shows. Kept separate so a test can name
/// them without starting an OS watcher.
pub fn watched_paths(home: &Path, data: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for folder in ["saves", "mods", "shaderpacks", "config", "screenshots"] {
        paths.push(home.join(folder));
    }
    paths.push(home.join("options.txt"));
    for file in ["servers.json", "settings.json"] {
        paths.push(data.join(file));
    }
    paths.push(data.join("scripts"));
    paths
}

/// The quiet period after the last event before the window refreshes.
pub const DEBOUNCE: Duration = Duration::from_millis(350);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_watch_list_covers_worlds_mods_packs_and_servers() {
        let home = Path::new("/game");
        let data = Path::new("/data");
        let paths = watched_paths(home, data);
        assert!(paths.contains(&PathBuf::from("/game/saves")));
        assert!(paths.contains(&PathBuf::from("/game/mods")));
        assert!(paths.contains(&PathBuf::from("/game/shaderpacks")));
        assert!(paths.contains(&PathBuf::from("/game/options.txt")));
        assert!(paths.contains(&PathBuf::from("/data/servers.json")));
        assert!(paths.contains(&PathBuf::from("/data/scripts")));
    }
}

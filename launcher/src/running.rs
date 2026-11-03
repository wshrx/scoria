//! One game per game folder.
//!
//! `play` is a spawn, not a wait: without a mark, pressing Play twice starts a second client on the
//! same saves (two session locks, two writers). The mark is the game's PID in `.scoria-running` in
//! that instance's home. A live PID refuses a second launch; a stale mark (the launcher died, the
//! game was killed) is removed and Play works again. The waiter thread clears it on a normal exit.
use std::fs;
use std::path::{Path, PathBuf};

/// The mark's name, beside `options.txt` in the game folder.
const MARK: &str = ".scoria-running";

pub fn path(home: &Path) -> PathBuf {
    home.join(MARK)
}

/// The PID of a game that is still running in `home`, if the mark names one.
pub fn running(home: &Path) -> Option<u32> {
    let pid: u32 = fs::read_to_string(path(home)).ok()?.trim().parse().ok()?;
    if alive(pid) {
        return Some(pid);
    }
    // A crash or a kill leaves the mark behind; the PID is gone, so it is stale.
    let _ = fs::remove_file(path(home));
    None
}

/// Take the mark for a spawned game.
pub fn hold(home: &Path, pid: u32) -> Result<(), String> {
    fs::create_dir_all(home).map_err(|e| format!("{}: {e}", home.display()))?;
    fs::write(path(home), pid.to_string()).map_err(|e| format!("{}: {e}", path(home).display()))
}

/// Clear the mark if it is still this game's, so an older waiter never clears a newer launch.
pub fn release(home: &Path, pid: u32) {
    let current = fs::read_to_string(path(home))
        .ok()
        .and_then(|text| text.trim().parse::<u32>().ok());
    if current == Some(pid) {
        let _ = fs::remove_file(path(home));
    }
}

/// The game's process name. The launcher is `scoria`; a recycled launcher PID must not count as a
/// running game.
fn is_game_comm(name: &str) -> bool {
    name.contains("horizon-client")
}

/// Whether a `/proc/<pid>/stat` line belongs to a zombie: it has exited, but its parent has not
/// reaped it yet. A zombie keeps its old comm and would otherwise block Play.
fn is_zombie_stat(stat: &str) -> bool {
    stat.rsplit_once(')')
        .and_then(|(_, rest)| rest.split_whitespace().next())
        == Some("Z")
}

/// Whether `pid` belongs to a live game. Linux also checks the name, so a recycled PID does not
/// block Play forever; other Unix systems ask `kill -0`.
fn alive(pid: u32) -> bool {
    #[cfg(target_os = "linux")]
    {
        let proc = format!("/proc/{pid}");
        if !Path::new(&proc).exists() {
            return false;
        }
        // A child that has exited but not yet been reaped keeps /proc/<pid> and its old comm.
        // Treat it as gone, or Home stays "Playing" after the player closes the window.
        if let Ok(stat) = fs::read_to_string(format!("{proc}/stat")) {
            if is_zombie_stat(&stat) {
                return false;
            }
        }
        match fs::read(format!("{proc}/comm")) {
            Ok(name) => is_game_comm(&String::from_utf8_lossy(&name)),
            Err(_) => false,
        }
    }
    #[cfg(all(unix, not(target_os = "linux")))]
    {
        std::process::Command::new("kill")
            .args(["-0", &pid.to_string()])
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
    #[cfg(windows)]
    {
        // tasklist with a PID filter: the image name proves the PID is still the game, not a
        // recycled one. No shell, so no quoting problems with a hostile PID.
        std::process::Command::new("tasklist")
            .args(["/FI", &format!("PID eq {pid}"), "/FO", "CSV", "/NH"])
            .output()
            .map(|out| {
                String::from_utf8_lossy(&out.stdout)
                    .to_ascii_lowercase()
                    .contains("horizon-client")
            })
            .unwrap_or(false)
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = pid;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_mark_is_not_running() {
        let dir = std::env::temp_dir().join(format!("scoria-running-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        assert_eq!(running(&dir), None);
    }

    #[test]
    fn a_dead_pid_is_stale_and_removed() {
        let dir = std::env::temp_dir().join(format!("scoria-running-stale-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(path(&dir), "999999").unwrap();
        assert_eq!(running(&dir), None);
        assert!(!path(&dir).exists());
    }

    #[test]
    fn a_zombie_or_the_launcher_is_not_a_running_game() {
        assert!(is_game_comm("horizon-client\n"));
        assert!(!is_game_comm("scoria\n"));
        assert!(is_zombie_stat(
            "123 (horizon-client) Z 1 2 3 4 5 6 7 8 9 10"
        ));
        assert!(!is_zombie_stat(
            "123 (horizon-client) S 1 2 3 4 5 6 7 8 9 10"
        ));
    }

    #[test]
    fn release_only_clears_its_own_pid() {
        let dir = std::env::temp_dir().join(format!("scoria-running-own-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        hold(&dir, 123).unwrap();
        release(&dir, 124);
        assert_eq!(fs::read_to_string(path(&dir)).unwrap(), "123");
        release(&dir, 123);
        assert!(!path(&dir).exists());
    }
}

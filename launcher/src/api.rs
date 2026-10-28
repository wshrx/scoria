//! The launcher's machine interface: JSON lines on stdin/stdout, one request per line.
//!
//! `horizon-launcher api` serves the launcher UI (`launcher-ui/`, flint-ui on gpuix) and any
//! script. It is deliberately small: state, prepare, play, and changing the offline name. The
//! UI never touches the file system or starts processes itself.
//!
//! Request:  {"id": 1, "method": "state", "params": {}}
//! Response: {"id": 1, "ok": true, "result": {...}}
//!           {"id": 1, "ok": false, "error": "why"}

use crate::account::{Accounts, Player};
use crate::backend;
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

pub fn serve() -> Result<(), String> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.trim().is_empty() {
            continue;
        }
        let request: Value = match serde_json::from_str(&line) {
            Ok(value) => value,
            Err(e) => {
                writeln!(
                    stdout,
                    "{}",
                    json!({ "ok": false, "error": format!("bad JSON: {e}") })
                )
                .map_err(|e| e.to_string())?;
                stdout.flush().map_err(|e| e.to_string())?;
                continue;
            }
        };
        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let method = request
            .get("method")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let params = request.get("params").cloned().unwrap_or(Value::Null);
        let response = match dispatch(&method, &params) {
            Ok(result) => json!({ "id": id, "ok": true, "result": result }),
            Err(error) => json!({ "id": id, "ok": false, "error": error }),
        };
        writeln!(stdout, "{response}").map_err(|e| e.to_string())?;
        stdout.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn dispatch(method: &str, params: &Value) -> Result<Value, String> {
    match method {
        "state" => state(),
        "prepare" => prepare(),
        "play" => play(),
        "set_offline_name" => set_offline_name(params),
        other => Err(format!("unknown method {other}")),
    }
}

fn selected_tag() -> Result<String, String> {
    let settings = backend::Settings::load();
    settings
        .selected
        .or_else(|| backend::installed().last().cloned())
        .ok_or_else(|| "No version is installed yet.".to_string())
}

fn state() -> Result<Value, String> {
    let tag = selected_tag()?;
    let accounts = Accounts::load();
    let instance = crate::instances::selected();
    let prepared = backend::is_prepared(&tag);
    let offline = accounts
        .name_arg()
        .map(str::to_string)
        .unwrap_or_else(|| "Player".to_string());

    let worlds: Vec<Value> = crate::worlds::list(&instance.home)
        .into_iter()
        .map(|w| {
            json!({
                "name": w.name,
                "mode": w.mode,
                "lastPlayed": ago(w.last_played),
                "size": mb(w.size),
            })
        })
        .collect();

    let instances: Vec<Value> = crate::instances::list()
        .into_iter()
        .map(|i| {
            let version = i.version.clone().unwrap_or_else(|| tag.clone());
            let kind = match i.kind {
                crate::instances::Kind::Default | crate::instances::Kind::Own => "release",
                crate::instances::Kind::Port => "port",
                crate::instances::Kind::Pack => "modpack",
            };
            json!({
                "id": i.id,
                "name": i.name,
                "version": short_version(&version),
                "kind": kind,
                "prepared": backend::is_prepared(&version),
            })
        })
        .collect();

    Ok(json!({
        "version": tag,
        "branch": std::env::var("SCORIA_BRANCH").unwrap_or_else(|_| "dev".into()),
        "prepared": prepared,
        "offlineName": offline,
        "worlds": worlds,
        "instances": instances,
    }))
}

fn prepare() -> Result<Value, String> {
    let tag = selected_tag()?;
    let settings = backend::Settings::load();
    let minecraft = backend::minecraft_dir(settings.minecraft_dir.clone())?;
    let log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = {
        let log = log.clone();
        move |line: String| {
            let mut log = log.lock().unwrap();
            log.push(line);
            if log.len() > 200 {
                log.remove(0);
            }
        }
    };
    let (preparer, minecraft) = backend::preparer(&tag, &minecraft, &sink)?;
    backend::sounds(&tag, &minecraft, &sink);
    match preparer {
        backend::Preparer::Process(mut cmd) => {
            let status = cmd.status().map_err(|e| e.to_string())?;
            if !status.success() {
                return Err(format!(
                    "The preparer stopped with {}.",
                    status.code().unwrap_or(-1)
                ));
            }
        }
        backend::Preparer::InProcess(options) => backend::prepare_in_process(&options, &sink)?,
    }
    Ok(json!({
        "prepared": backend::is_prepared(&tag),
        "log": log.lock().unwrap().clone(),
    }))
}

fn play() -> Result<Value, String> {
    let tag = selected_tag()?;
    let instance = crate::instances::selected();
    let accounts = Accounts::load();
    let mut cmd = backend::client(&tag)?;
    crate::instances::apply(&mut cmd, &instance)?;
    if let Some(name) = accounts.name_arg() {
        cmd.arg("--name").arg(name);
    }
    cmd.envs(accounts.look_env());
    let mut cmd = crate::launch::wrap(cmd, &instance.launch);
    let child = cmd
        .spawn()
        .map_err(|e| format!("The game did not start: {e}"))?;
    Ok(json!({ "pid": child.id(), "version": tag, "instance": instance.name }))
}

fn set_offline_name(params: &Value) -> Result<Value, String> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or("set_offline_name needs {\"name\": \"…\"}")?;
    let mut accounts = Accounts::load();
    accounts.play_as(Player::Offline(name.to_string()))?;
    accounts.save()?;
    state()
}

/// `v0.2.92` → `0.2.92` for the UI's compact labels.
fn short_version(tag: &str) -> String {
    tag.trim_start_matches('v').to_string()
}

fn mb(bytes: u64) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.1} GB", bytes as f64 / 1e9)
    } else {
        format!("{} MB", bytes.div_ceil(1_000_000))
    }
}

fn ago(time: SystemTime) -> String {
    let seconds = SystemTime::now()
        .duration_since(time)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    match seconds {
        0..=59 => "just now".into(),
        60..=3599 => format!("{} min ago", seconds / 60),
        3600..=86399 => format!("{} h ago", seconds / 3600),
        _ => format!("{} days ago", seconds / 86400),
    }
}

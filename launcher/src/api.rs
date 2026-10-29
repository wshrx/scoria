//! The launcher's machine interface: JSON lines on stdin/stdout, one request per line.
//!
//! `horizon-launcher api` serves the launcher UI (`launcher-ui/`, flint-ui on gpuix) and any
//! script. It is deliberately small: state, prepare, play, mods, and changing the offline name. The
//! UI never touches the file system or starts processes itself.
//!
//! Request:  {"id": 1, "method": "state", "params": {}}
//! Response: {"id": 1, "ok": true, "result": {...}}
//!           {"id": 1, "ok": false, "error": "why"}

use crate::account::{Accounts, Player};
use crate::backend;
use crate::browser::{self, Kind, Query, SourceId};
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

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
        let response = {
            let mut emit = |value: Value| {
                let _ = writeln!(stdout, "{value}");
                let _ = stdout.flush();
            };
            match dispatch(&id, &method, &params, &mut emit) {
                Ok(result) => json!({ "id": id, "ok": true, "result": result }),
                Err(error) => json!({ "id": id, "ok": false, "error": error }),
            }
        };
        writeln!(stdout, "{response}").map_err(|e| e.to_string())?;
        stdout.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn dispatch(
    id: &Value,
    method: &str,
    params: &Value,
    emit: &mut dyn FnMut(Value),
) -> Result<Value, String> {
    match method {
        "state" => state(),
        "prepare" => prepare(id, params, emit),
        "play" => play(params),
        "select_instance" => select_instance(params),
        "create_instance" => create_instance(params),
        "set_offline_name" => set_offline_name(params),
        "set_appearance" => set_appearance(params),
        "set_while_playing" => set_while_playing(params),
        "backup_world" => backup_world(params),
        "versions" => versions(),
        "select_version" => select_version(params),
        "remove_version" => remove_version(params),
        "install_latest" => install_latest(),
        "minecraft" => minecraft(),
        "download_minecraft" => download_minecraft(id, params, emit),
        "mods" => mods(),
        "search_mods" => search_mods(params),
        "install_mod" => install_mod(id, params, emit),
        "remove_mod" => remove_mod(params),
        "servers" => servers(),
        "add_server" => add_server(params),
        "remove_server" => remove_server(params),
        "set_language" => set_language(params),
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
    let settings = backend::Settings::load();
    let prepared = backend::is_prepared(&tag);
    let appearance = match settings.appearance {
        backend::Appearance::Dark => "dark",
        backend::Appearance::Light => "light",
        backend::Appearance::System => "system",
    };
    let offline = accounts
        .name_arg()
        .map(str::to_string)
        .unwrap_or_else(|| "Player".to_string());

    let worlds: Vec<Value> = crate::worlds::list(&instance.home)
        .into_iter()
        .map(|w| {
            json!({
                "folder": w.folder,
                "name": w.name,
                "mode": w.mode,
                "lastPlayed": ago(w.last_played),
                "lastPlayedAt": w
                    .last_played
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0),
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
                "minecraft": short_version(&backend::game_version(&version)),
                "kind": kind,
                "prepared": backend::is_prepared(&version),
                "selected": i.id == instance.id,
            })
        })
        .collect();

    let playing = crate::running::running(&instance.home);
    Ok(json!({
        "version": tag,
        "branch": std::env::var("SCORIA_BRANCH").unwrap_or_else(|_| "dev".into()),
        "prepared": prepared,
        "playing": playing.is_some(),
        "playingPid": playing,
        "offlineName": offline,
        "selectedInstance": instance.id,
        "appearance": appearance,
        "whilePlaying": settings.while_playing.name(),
        "language": settings.language.map(|l| l.code()).unwrap_or("system"),
        "worlds": worlds,
        "instances": instances,
        "minecraft": minecraft_json(&tag),
    }))
}

fn prepare(id: &Value, params: &Value, emit: &mut dyn FnMut(Value)) -> Result<Value, String> {
    let tag = requested_tag(params)?;
    if params
        .get("download")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        ensure_minecraft(&tag, id, emit)?;
    }
    let settings = backend::Settings::load();
    let minecraft = backend::minecraft_dir(settings.minecraft_dir.clone())?;
    let log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = {
        let log = log.clone();
        let id = id.clone();
        let emit = std::cell::RefCell::new(emit);
        move |line: String| {
            {
                let mut log = log.lock().unwrap();
                log.push(line.clone());
                if log.len() > 200 {
                    log.remove(0);
                }
            }
            if let Ok(mut emit) = emit.try_borrow_mut() {
                emit(json!({ "id": id, "event": "log", "line": line }));
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

fn play(params: &Value) -> Result<Value, String> {
    let instance = requested_instance(params)?;
    if let Some(pid) = crate::running::running(&instance.home) {
        return Err(format!(
            "Scoria is already running (PID {pid}). Close the game window first."
        ));
    }
    let tag = instance.resolve_version(None)?;
    let accounts = Accounts::load();
    let mut cmd = backend::client(&tag)?;
    crate::instances::apply(&mut cmd, &instance)?;
    if let Some(world) = params.get("world").and_then(Value::as_str) {
        cmd.arg("--world").arg(world);
    }
    if let Some(server) = params.get("server").and_then(Value::as_str) {
        cmd.arg("--server").arg(server);
    }
    if let Some(name) = accounts.name_arg() {
        cmd.arg("--name").arg(name);
    }
    cmd.envs(accounts.look_env());
    let mut cmd = crate::launch::wrap(cmd, &instance.launch);
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("The game did not start: {e}"))?;
    let pid = child.id();
    crate::running::hold(&instance.home, pid)?;
    let home = instance.home.clone();
    // Play returns at once; this waiter is what clears the mark when the player closes the game.
    std::thread::spawn(move || {
        let _ = child.wait();
        crate::running::release(&home, pid);
    });
    crate::instances::select(&instance.id)?;
    Ok(json!({ "pid": pid, "version": tag, "instance": instance.name }))
}

fn requested_instance(params: &Value) -> Result<crate::instances::Instance, String> {
    match params.get("instance").and_then(Value::as_str) {
        Some(id) => crate::instances::find(id),
        None => Ok(crate::instances::selected()),
    }
}

fn requested_tag(params: &Value) -> Result<String, String> {
    match params.get("instance").and_then(Value::as_str) {
        Some(id) => crate::instances::find(id)?.resolve_version(None),
        None => selected_tag(),
    }
}

fn select_instance(params: &Value) -> Result<Value, String> {
    let id = params
        .get("id")
        .and_then(Value::as_str)
        .ok_or("select_instance needs {\"id\": \"…\"}")?;
    crate::instances::select(id)?;
    state()
}

fn create_instance(params: &Value) -> Result<Value, String> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or("create_instance needs {\"name\": \"…\"}")?;
    let created = crate::instances::create(name, None)?;
    crate::instances::select(&created.id)?;
    state()
}

fn set_appearance(params: &Value) -> Result<Value, String> {
    let value = params
        .get("value")
        .and_then(Value::as_str)
        .ok_or("set_appearance needs {\"value\": \"dark|light|system\"}")?;
    let appearance = match value {
        "dark" => backend::Appearance::Dark,
        "light" => backend::Appearance::Light,
        "system" => backend::Appearance::System,
        other => return Err(format!("{other}: say dark, light or system")),
    };
    let mut settings = backend::Settings::load();
    settings.appearance = appearance;
    settings.save();
    state()
}

fn set_while_playing(params: &Value) -> Result<Value, String> {
    let value = params
        .get("value")
        .and_then(Value::as_str)
        .ok_or("set_while_playing needs {\"value\": \"stay|minimize|hide\"}")?;
    let mode = backend::WhilePlaying::parse(value)
        .ok_or_else(|| format!("{value}: say stay, minimize or hide"))?;
    let mut settings = backend::Settings::load();
    settings.while_playing = mode;
    settings.save();
    state()
}

fn backup_world(params: &Value) -> Result<Value, String> {
    let instance = requested_instance(params)?;
    let world = params
        .get("world")
        .and_then(Value::as_str)
        .ok_or("backup_world needs {\"world\": \"…\"}")?;
    let found = crate::worlds::find(&instance.home, world)?;
    let path = crate::worlds::backup(&instance.home, &found)?;
    Ok(json!({ "path": path }))
}

fn versions() -> Result<Value, String> {
    let settings = backend::Settings::load();
    let selected = settings.selected.clone();
    let installed: Vec<Value> = backend::installed()
        .into_iter()
        .map(|tag| {
            json!({
                "tag": tag,
                "prepared": backend::is_prepared(&tag),
                "selected": selected.as_deref() == Some(tag.as_str()),
            })
        })
        .collect();
    Ok(json!({ "installed": installed, "selected": selected }))
}

fn select_version(params: &Value) -> Result<Value, String> {
    let tag = params
        .get("tag")
        .and_then(Value::as_str)
        .ok_or("select_version needs {\"tag\": \"v…\"}")?;
    let tag = backend::resolve_version(Some(tag))?;
    let mut settings = backend::Settings::load();
    settings.selected = Some(tag);
    settings.save();
    versions()
}

fn remove_version(params: &Value) -> Result<Value, String> {
    let tag = params
        .get("tag")
        .and_then(Value::as_str)
        .ok_or("remove_version needs {\"tag\": \"v…\"}")?;
    backend::remove_version(tag)?;
    versions()
}

fn install_latest() -> Result<Value, String> {
    let releases = backend::releases()?;
    let release = releases
        .iter()
        .find(|r| r.asset.is_some())
        .ok_or("No build for this platform in the release feed.")?;
    backend::install_tag(&release.tag, false, |_| {})?;
    versions()
}

/// The selected version's Minecraft files: what is on disk, and where they came from.
fn minecraft() -> Result<Value, String> {
    let tag = selected_tag()?;
    Ok(minecraft_json(&tag))
}

fn minecraft_json(tag: &str) -> Value {
    let dest = backend::minecraft_folder();
    let pins = match backend::pins(tag) {
        Ok(pins) => pins,
        Err(error) => {
            return json!({
                "version": backend::game_version(tag),
                "path": dest.display().to_string(),
                "source": crate::piston::SOURCE,
                "manifestSha1": null,
                "installed": false,
                "manifest": false,
                "client": false,
                "libraries": { "done": 0, "total": 0 },
                "assets": { "done": 0, "total": 0 },
                "missingBytes": 0,
                "error": error,
            });
        }
    };
    let files = crate::piston::status(&pins, &dest);
    json!({
        "version": pins.version,
        "path": dest.display().to_string(),
        "source": crate::piston::SOURCE,
        "manifestSha1": pins.manifests.first(),
        "installed": files.installed,
        "manifest": files.manifest,
        "client": files.client,
        "libraries": { "done": files.libraries.done, "total": files.libraries.total },
        "assets": { "done": files.assets.done, "total": files.assets.total },
        "missingBytes": files.missing_bytes,
    })
}

fn download_minecraft(
    id: &Value,
    params: &Value,
    emit: &mut dyn FnMut(Value),
) -> Result<Value, String> {
    let tag = requested_tag(params)?;
    ensure_minecraft(&tag, id, emit)
}

/// Fetches from piston-meta whatever the version lacks; logs and progress go to the request.
fn ensure_minecraft(tag: &str, id: &Value, emit: &mut dyn FnMut(Value)) -> Result<Value, String> {
    let pins = backend::pins(tag)?;
    let dest = backend::minecraft_folder();
    if crate::piston::status(&pins, &dest).installed {
        return Ok(minecraft_json(tag));
    }
    crate::mojang::CANCEL.store(false, std::sync::atomic::Ordering::Relaxed);
    let log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let events = std::cell::RefCell::new(emit);
    let sink_log = log.clone();
    let sink = |line: String| {
        {
            let mut log = sink_log.lock().unwrap();
            log.push(line.clone());
            if log.len() > 200 {
                log.remove(0);
            }
        }
        if let Ok(mut emit) = events.try_borrow_mut() {
            emit(json!({ "id": id, "event": "log", "line": line }));
        }
    };
    let progress = |done: usize, total: usize| {
        if let Ok(mut emit) = events.try_borrow_mut() {
            emit(json!({ "id": id, "event": "progress", "done": done, "total": total }));
        }
    };
    crate::piston::download(
        &pins,
        &dest,
        crate::piston::JOBS,
        &|url| backend::fetch_bytes(url),
        &crate::mojang::CANCEL,
        &sink,
        &progress,
    )?;
    Ok(minecraft_json(tag))
}

fn project_json(project: &browser::Project) -> Value {
    let support = project.support();
    json!({
        "id": project.id,
        "slug": project.slug,
        "title": project.title,
        "summary": project.summary,
        "author": project.author,
        "page": project.page,
        "kind": project.kind,
        "support": support.label(),
        "reason": support.reason(),
        "installable": support.installable(),
    })
}

fn mods() -> Result<Value, String> {
    let instance = crate::instances::selected();
    let installed = browser::installed::Installed::load(&instance.home);
    Ok(json!({ "instance": instance.name, "entries": installed.entries }))
}

fn mod_kind(name: &str) -> Result<Kind, String> {
    match name {
        "mod" | "mods" => Ok(Kind::Mod),
        "resource-pack" | "resourcepack" | "resource" => Ok(Kind::ResourcePack),
        other => Err(format!("{other}: say mod or resource-pack")),
    }
}

fn mod_source() -> Result<Box<dyn browser::Source>, String> {
    browser::source(SourceId::Modrinth).ok_or_else(|| "Modrinth is not available.".to_string())
}

fn search_mods(params: &Value) -> Result<Value, String> {
    let text = params.get("query").and_then(Value::as_str).unwrap_or("");
    let kind = mod_kind(params.get("kind").and_then(Value::as_str).unwrap_or("mod"))?;
    let source = mod_source()?;
    let mut query = Query::new(text, kind);
    // Show what exists and mark how it runs; only installable results get a button.
    query.everything = true;
    query.limit = 24;
    let found = source.search(&query)?;
    Ok(json!({ "results": found.iter().map(project_json).collect::<Vec<_>>() }))
}

fn install_mod(id: &Value, params: &Value, emit: &mut dyn FnMut(Value)) -> Result<Value, String> {
    let project_id = params
        .get("id")
        .and_then(Value::as_str)
        .ok_or("install_mod needs {\"id\": \"…\"}")?;
    let source = mod_source()?;
    let project = source.project(project_id)?;
    let instance = crate::instances::selected();
    let events = std::cell::RefCell::new(emit);
    let progress = |label: &str, value: f32| {
        if let Ok(mut emit) = events.try_borrow_mut() {
            emit(json!({ "id": id, "event": "progress", "label": label, "value": value }));
        }
    };
    let entries =
        browser::installed::install(source.as_ref(), &project, true, &instance.home, &progress)?;
    Ok(json!({ "instance": instance.name, "entries": entries }))
}

fn remove_mod(params: &Value) -> Result<Value, String> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or("remove_mod needs {\"name\": \"…\"}")?;
    let instance = crate::instances::selected();
    browser::installed::remove(&instance.home, name)?;
    let installed = browser::installed::Installed::load(&instance.home);
    Ok(json!({ "instance": instance.name, "entries": installed.entries }))
}

fn servers() -> Result<Value, String> {
    let instance = crate::instances::selected();
    let list = crate::server_list::List::load(&instance.home);
    let servers: Vec<Value> = list
        .servers()
        .into_iter()
        .enumerate()
        .map(|(index, server)| json!({ "index": index, "name": server.name, "ip": server.ip }))
        .collect();
    Ok(json!({ "instance": instance.name, "servers": servers }))
}

fn add_server(params: &Value) -> Result<Value, String> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or("add_server needs {\"name\": \"…\", \"ip\": \"…\"}")?;
    let ip = params
        .get("ip")
        .and_then(Value::as_str)
        .ok_or("add_server needs {\"name\": \"…\", \"ip\": \"…\"}")?;
    let instance = crate::instances::selected();
    let mut list = crate::server_list::List::load(&instance.home);
    list.add(name, ip)?;
    list.save(&instance.home)?;
    servers()
}

fn remove_server(params: &Value) -> Result<Value, String> {
    let index = params
        .get("index")
        .and_then(Value::as_u64)
        .ok_or("remove_server needs {\"index\": 0}")? as usize;
    let instance = crate::instances::selected();
    let mut list = crate::server_list::List::load(&instance.home);
    list.remove(index)?;
    list.save(&instance.home)?;
    servers()
}

fn set_language(params: &Value) -> Result<Value, String> {
    use crate::i18n::Language;
    let value = params
        .get("value")
        .and_then(Value::as_str)
        .ok_or("set_language needs {\"value\": \"system|en|ru\"}")?;
    let mut settings = backend::Settings::load();
    settings.language = match value {
        "system" | "default" => None,
        code => Some(Language::parse(code).ok_or_else(|| format!("{code}: say system, en or ru"))?),
    };
    settings.save();
    state()
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

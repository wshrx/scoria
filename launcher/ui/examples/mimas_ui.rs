//! Experimental Mimas → GPUI launcher.
//!
//! The `.mim` script builds the whole window tree: rows, columns, text, buttons, inputs,
//! icons and images. Rust maps the tree to GPUI flex elements and exposes the launcher's
//! state and actions as host functions (`version()`, `world_count()`, `search_catalog()`…).
//!
//! Real data, real actions: Play/Stop call the launcher engine, search and install use the
//! same catalogue as the production UI, servers and offline accounts are the player's own.
//! With `SCORIA_MIMAS_DEMO=1` empty profiles show the demo rows used in the README.
//!
//!   cargo run -p scoria-ui --features dev-scripts --example mimas-ui
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::SystemTime,
};

use gpui::{
    div, img, point, prelude::*, px, rgb, rgba, size, svg, Bounds, Context, FocusHandle, Image,
    ImageFormat, ImageSource, IntoElement, KeyDownEvent, ObjectFit, Render, Rgba, Window,
    WindowBounds, WindowDecorations, WindowOptions,
};
use gpui_ce_platform::application;
use mimas::mimas;
use scoria_core::{
    account::{Accounts, Player},
    api,
    backend::{Appearance, Settings},
    browser::installed::Installed,
    instances,
};
use scoria_ui_kit::Theme;
use serde_json::{json, Value};

#[derive(Clone, Copy)]
enum Direction {
    Row,
    Column,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Align {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Justify {
    Start,
    Center,
    End,
    Between,
    Around,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Ready,
    Preparing,
    Playing,
}

static UI: Mutex<Mode> = Mutex::new(Mode::Ready);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    Mods,
    Servers,
    Settings,
}

static SCREEN: Mutex<Screen> = Mutex::new(Screen::Home);

#[derive(Clone)]
enum Node {
    Box {
        direction: Direction,
        gap: f32,
        pad_x: f32,
        pad_y: f32,
        width: Option<f32>,
        height: Option<f32>,
        color: Rgba,
        radius: f32,
        align: Option<Align>,
        justify: Option<Justify>,
        grow: Option<f32>,
        wrap: bool,
        scroll: bool,
        children: Vec<Node>,
    },
    Text {
        content: String,
        size: f32,
        color: Rgba,
        width: Option<f32>,
    },
    Button {
        label: String,
        action: String,
        fill: Rgba,
        color: Rgba,
        small: bool,
        width: Option<f32>,
    },
    Link {
        label: String,
        action: String,
        color: Rgba,
        size: f32,
    },
    Icon {
        name: String,
        size: f32,
        color: Rgba,
    },
    Image {
        name: String,
        width: f32,
        height: f32,
        radius: f32,
    },
    Input {
        id: String,
        placeholder: String,
        value: String,
        focused: bool,
        width: f32,
    },
    Spacer,
}

struct Frame {
    direction: Direction,
    gap: f32,
    pad_x: f32,
    pad_y: f32,
    width: Option<f32>,
    height: Option<f32>,
    color: Rgba,
    radius: f32,
    align: Option<Align>,
    justify: Option<Justify>,
    grow: Option<f32>,
    wrap: bool,
    scroll: bool,
    children: Vec<Node>,
}

#[derive(Default)]
struct Builder {
    stack: Vec<Frame>,
    root: Vec<Node>,
}

static BUILDER: Mutex<Option<Builder>> = Mutex::new(None);
static SCROLL_ID: AtomicU64 = AtomicU64::new(0);

/* -------------------------------------------------------------------------------------------
The launcher state the script reads
------------------------------------------------------------------------------------------- */

struct Backend {
    snapshot: Value,
    servers: Value,
    installed: Value,
    accounts: Accounts,
    results: Value,
    busy: bool,
    status: String,
    progress: f64,
    error: Option<String>,
}

static BACKEND: Mutex<Option<Backend>> = Mutex::new(None);
static INPUTS: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
static FOCUSED: Mutex<Option<String>> = Mutex::new(None);
static REVISION: AtomicU64 = AtomicU64::new(0);

fn demo() -> bool {
    std::env::var_os("SCORIA_MIMAS_DEMO").is_some()
}

impl Backend {
    fn load() -> Self {
        let mut backend = Backend {
            snapshot: Value::Null,
            servers: json!({ "servers": [] }),
            installed: json!({ "entries": [] }),
            accounts: Accounts::load(),
            results: json!({ "results": [] }),
            busy: false,
            status: String::new(),
            progress: 0.0,
            error: None,
        };
        backend.refresh();
        backend
    }

    fn refresh(&mut self) {
        self.snapshot = api::snapshot().unwrap_or(Value::Null);
        self.servers = api::saved_servers().unwrap_or_else(|_| json!({ "servers": [] }));
        let home = instances::selected().home;
        self.installed = serde_json::to_value(Installed::load(&home))
            .unwrap_or_else(|_| json!({ "entries": [] }));
        self.accounts = Accounts::load();
    }
}

fn with_backend<T>(fallback: T, f: impl FnOnce(&Backend) -> T) -> T {
    BACKEND
        .lock()
        .ok()
        .and_then(|guard| guard.as_ref().map(f))
        .unwrap_or(fallback)
}

fn snapshot_text(key: &str) -> String {
    with_backend(String::new(), |backend| {
        backend.snapshot[key]
            .as_str()
            .unwrap_or_default()
            .to_string()
    })
}

fn snapshot_bool(key: &str) -> bool {
    with_backend(false, |backend| {
        backend.snapshot[key].as_bool().unwrap_or(false)
    })
}

fn array_len(value: &Value) -> usize {
    value.as_array().map(Vec::len).unwrap_or(0)
}

fn array_text(value: &Value, index: usize, key: &str) -> String {
    value
        .as_array()
        .and_then(|items| items.get(index))
        .and_then(|item| item.get(key))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn bump_revision() {
    REVISION.fetch_add(1, Ordering::Relaxed);
}

fn backend_progress(percent: f64, line: &str) {
    if let Ok(mut guard) = BACKEND.lock() {
        if let Some(backend) = guard.as_mut() {
            backend.busy = true;
            backend.progress = percent;
            if !line.is_empty() {
                backend.status = line.to_string();
            }
        }
    }
    bump_revision();
}

fn backend_set_busy(status: &str) {
    if let Ok(mut guard) = BACKEND.lock() {
        if let Some(backend) = guard.as_mut() {
            backend.busy = true;
            backend.progress = 0.0;
            backend.status = status.to_string();
            backend.error = None;
        }
    }
    bump_revision();
}

fn backend_finish(result: Result<String, String>) {
    if let Ok(mut guard) = BACKEND.lock() {
        if let Some(backend) = guard.as_mut() {
            backend.busy = false;
            backend.progress = 0.0;
            match result {
                Ok(message) => {
                    backend.status = message;
                    backend.error = None;
                    backend.refresh();
                }
                Err(error) => {
                    backend.error = Some(error);
                    backend.status.clear();
                }
            }
        }
    }
    bump_revision();
}

fn backend_sync(result: Result<String, String>) {
    if let Ok(mut guard) = BACKEND.lock() {
        if let Some(backend) = guard.as_mut() {
            match result {
                Ok(message) => {
                    backend.status = message;
                    backend.error = None;
                }
                Err(error) => backend.error = Some(error),
            }
            backend.refresh();
        }
    }
    bump_revision();
}

fn backend_results(results: Value) {
    if let Ok(mut guard) = BACKEND.lock() {
        if let Some(backend) = guard.as_mut() {
            backend.results = results;
            backend.status.clear();
        }
    }
    bump_revision();
}

fn input_get(id: &str) -> String {
    INPUTS
        .lock()
        .ok()
        .and_then(|guard| guard.as_ref().and_then(|map| map.get(id).cloned()))
        .unwrap_or_default()
}

fn input_take(id: &str) -> String {
    INPUTS
        .lock()
        .ok()
        .and_then(|mut guard| guard.as_mut().and_then(|map| map.remove(id)))
        .unwrap_or_default()
}

fn focused_input() -> Option<String> {
    FOCUSED
        .lock()
        .ok()
        .and_then(|guard| guard.as_ref().cloned())
}

/* -------------------------------------------------------------------------------------------
The tree the script builds
------------------------------------------------------------------------------------------- */

fn color(value: &str) -> Rgba {
    let hex = value.trim().trim_start_matches('#');
    if hex.is_empty() {
        return rgba(0x00000000);
    }
    let parsed = u32::from_str_radix(hex, 16).unwrap_or(0xFFFFFF);
    if hex.len() == 8 {
        rgba(parsed)
    } else {
        rgb(parsed)
    }
}

fn begin(direction: Direction, gap: f64, padding: f64, w: f64, h: f64, fill: &str, radius: f64) {
    let mut guard = BUILDER.lock().expect("builder");
    let builder = guard.get_or_insert_with(Builder::default);
    builder.stack.push(Frame {
        direction,
        gap: gap as f32,
        pad_x: padding as f32,
        pad_y: padding as f32,
        width: (w > 0.0).then_some(w as f32),
        height: (h > 0.0).then_some(h as f32),
        color: color(fill),
        radius: radius as f32,
        align: Some(Align::Start),
        justify: Some(Justify::Start),
        grow: None,
        wrap: false,
        scroll: false,
        children: Vec::new(),
    });
}

fn end() {
    let mut guard = BUILDER.lock().expect("builder");
    let builder = guard.get_or_insert_with(Builder::default);
    let Some(frame) = builder.stack.pop() else {
        return;
    };
    let node = Node::Box {
        direction: frame.direction,
        gap: frame.gap,
        pad_x: frame.pad_x,
        pad_y: frame.pad_y,
        width: frame.width,
        height: frame.height,
        color: frame.color,
        radius: frame.radius,
        align: frame.align,
        justify: frame.justify,
        grow: frame.grow,
        wrap: frame.wrap,
        scroll: frame.scroll,
        children: frame.children,
    };
    match builder.stack.last_mut() {
        Some(parent) => parent.children.push(node),
        None => builder.root.push(node),
    }
}

fn push(node: Node) {
    let mut guard = BUILDER.lock().expect("builder");
    let builder = guard.get_or_insert_with(Builder::default);
    match builder.stack.last_mut() {
        Some(parent) => parent.children.push(node),
        None => builder.root.push(node),
    }
}

#[mimas]
fn clear() {
    *BUILDER.lock().expect("builder") = Some(Builder::default());
}

#[mimas]
fn row(gap: f64, padding: f64, w: f64, h: f64, fill: &str, radius: f64) {
    begin(Direction::Row, gap, padding, w, h, fill, radius);
}

#[mimas]
fn column(gap: f64, padding: f64, w: f64, h: f64, fill: &str, radius: f64) {
    begin(Direction::Column, gap, padding, w, h, fill, radius);
}

/// A vertically scrollable column with a fixed height.
#[mimas]
fn scroll(gap: f64, padding: f64, w: f64, h: f64, fill: &str, radius: f64) {
    begin(Direction::Column, gap, padding, w, h, fill, radius);
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.scroll = true;
    }
}

/// Separate horizontal and vertical padding, after the frame is opened.
#[mimas]
fn pad(x: f64, y: f64) {
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.pad_x = x as f32;
        frame.pad_y = y as f32;
    }
}

#[mimas]
fn end_box() {
    end();
}

#[mimas]
fn align(value: &str) {
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.align = Some(match value {
            "center" => Align::Center,
            "end" => Align::End,
            "stretch" => Align::Stretch,
            _ => Align::Start,
        });
    }
}

#[mimas]
fn justify(value: &str) {
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.justify = Some(match value {
            "center" => Justify::Center,
            "end" => Justify::End,
            "between" => Justify::Between,
            "around" => Justify::Around,
            _ => Justify::Start,
        });
    }
}

#[mimas]
fn grow(value: f64) {
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.grow = Some(value as f32);
    }
}

#[mimas]
fn wrap() {
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.wrap = true;
    }
}

#[mimas]
fn text(content: &str, size: f64, fill: &str) {
    push(Node::Text {
        content: content.to_string(),
        size: size as f32,
        color: color(fill),
        width: None,
    });
}

/// Text with a maximum width, so long summaries and mod descriptions wrap.
#[mimas]
fn text_w(content: &str, size: f64, fill: &str, width: f64) {
    push(Node::Text {
        content: content.to_string(),
        size: size as f32,
        color: color(fill),
        width: Some(width as f32),
    });
}

#[mimas]
fn button(label: &str, action: &str, fill: &str) {
    button_with_width(label, action, fill, 192.0);
}

#[mimas]
fn button_w(label: &str, action: &str, fill: &str, width: f64) {
    button_with_width(label, action, fill, width);
}

fn button_with_width(label: &str, action: &str, fill: &str, width: f64) {
    let fill = color(fill);
    let light = fill.color.red > 0.6 && fill.color.green < 0.6;
    push(Node::Button {
        label: label.to_string(),
        action: action.to_string(),
        fill,
        color: if light { rgb(0x1A0A04) } else { rgb(0xF4F0EA) },
        small: false,
        width: Some(width as f32),
    });
}

#[mimas]
fn small_button(label: &str, action: &str, fill: &str) {
    small_with_width(label, action, fill, 112.0);
}

#[mimas]
fn small_button_w(label: &str, action: &str, fill: &str, width: f64) {
    small_with_width(label, action, fill, width);
}

fn small_with_width(label: &str, action: &str, fill: &str, width: f64) {
    let fill = color(fill);
    let light = fill.color.red > 0.6 && fill.color.green < 0.6;
    push(Node::Button {
        label: label.to_string(),
        action: action.to_string(),
        fill,
        color: if light { rgb(0x1A0A04) } else { rgb(0xF4F0EA) },
        small: true,
        width: Some(width as f32),
    });
}

#[mimas]
fn link(label: &str, action: &str, fill: &str, size: f64) {
    push(Node::Link {
        label: label.to_string(),
        action: action.to_string(),
        color: color(fill),
        size: size as f32,
    });
}

#[mimas]
fn spacer() {
    push(Node::Spacer);
}

#[mimas]
fn icon(name: &str, size: f64, fill: &str) {
    push(Node::Icon {
        name: name.to_string(),
        size: size as f32,
        color: color(fill),
    });
}

#[mimas]
fn image(name: &str, w: f64, h: f64, radius: f64) {
    push(Node::Image {
        name: name.to_string(),
        width: w as f32,
        height: h as f32,
        radius: radius as f32,
    });
}

/// A real, editable text field. Click it, type; `Enter` submits the field's action.
#[mimas]
fn input(id: &str, placeholder: &str, width: f64) {
    let value = input_get(id);
    let focused = focused_input().as_deref() == Some(id);
    push(Node::Input {
        id: id.to_string(),
        placeholder: placeholder.to_string(),
        value,
        focused,
        width: width as f32,
    });
}

/* -------------------------------------------------------------------------------------------
State for the script
------------------------------------------------------------------------------------------- */

#[mimas]
fn current_screen() -> String {
    match *SCREEN.lock().expect("screen") {
        Screen::Home => "home",
        Screen::Mods => "mods",
        Screen::Servers => "servers",
        Screen::Settings => "settings",
    }
    .to_string()
}

#[mimas]
fn preparing() -> bool {
    if *UI.lock().expect("mode") == Mode::Preparing {
        return true;
    }
    with_backend(false, |backend| backend.busy && backend.progress < 100.0)
}

#[mimas]
fn playing() -> bool {
    if *UI.lock().expect("mode") == Mode::Playing {
        return true;
    }
    snapshot_bool("playing")
}

#[mimas]
fn version() -> String {
    let version = snapshot_text("version");
    if version.is_empty() && demo() {
        return "Minecraft 26.3".to_string();
    }
    version
}

#[mimas]
fn branch() -> String {
    let branch = snapshot_text("branch");
    if branch.is_empty() && demo() {
        return "stable".to_string();
    }
    branch
}

#[mimas]
fn prepared() -> bool {
    snapshot_bool("prepared") || (demo() && !snapshot_bool("playing"))
}

#[mimas]
fn offline_name() -> String {
    let name = snapshot_text("offlineName");
    if name.is_empty() && demo() {
        return "Player".to_string();
    }
    name
}

#[mimas]
fn status_text() -> String {
    let text = with_backend(String::new(), |backend| backend.status.clone());
    if !text.is_empty() {
        return text;
    }
    let error = with_backend(None, |backend| backend.error.clone());
    if let Some(error) = error {
        return error;
    }
    if demo() {
        return "Готово к запуску".to_string();
    }
    String::new()
}

#[mimas]
fn progress() -> f64 {
    with_backend(0.0, |backend| backend.progress)
}

#[mimas]
fn busy() -> bool {
    with_backend(false, |backend| backend.busy)
}

#[mimas]
fn last_error() -> String {
    with_backend(String::new(), |backend| {
        backend.error.clone().unwrap_or_default()
    })
}

#[mimas]
fn appearance() -> String {
    let appearance = snapshot_text("appearance");
    if appearance.is_empty() {
        return "dark".to_string();
    }
    appearance
}

#[mimas]
fn world_count() -> f64 {
    let count = with_backend(0, |backend| array_len(&backend.snapshot["worlds"]));
    if count == 0 && demo() {
        return 3.0;
    }
    count as f64
}

#[mimas]
fn world_name(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.snapshot["worlds"]));
    if count == 0 && demo() {
        return match index as usize {
            0 => "New World",
            1 => "Build",
            _ => "Skyblock",
        }
        .to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.snapshot["worlds"], index as usize, "name")
    })
}

#[mimas]
fn world_folder(index: f64) -> String {
    with_backend(String::new(), |backend| {
        array_text(&backend.snapshot["worlds"], index as usize, "folder")
    })
}

#[mimas]
fn world_mode(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.snapshot["worlds"]));
    if count == 0 && demo() {
        return match index as usize {
            0 | 2 => "Выживание",
            _ => "Творческий",
        }
        .to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.snapshot["worlds"], index as usize, "mode")
    })
}

#[mimas]
fn world_last(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.snapshot["worlds"]));
    if count == 0 && demo() {
        return match index as usize {
            0 => "2 ч назад",
            1 => "3 дня назад",
            _ => "вчера",
        }
        .to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.snapshot["worlds"], index as usize, "lastPlayed")
    })
}

#[mimas]
fn world_size(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.snapshot["worlds"]));
    if count == 0 && demo() {
        return "48 MB".to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.snapshot["worlds"], index as usize, "size")
    })
}

#[mimas]
fn server_count() -> f64 {
    let count = with_backend(0, |backend| array_len(&backend.servers["servers"]));
    if count == 0 && demo() {
        return 2.0;
    }
    count as f64
}

#[mimas]
fn server_name(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.servers["servers"]));
    if count == 0 && demo() {
        return match index as usize {
            0 => "SMP",
            _ => "Creative",
        }
        .to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.servers["servers"], index as usize, "name")
    })
}

#[mimas]
fn server_ip(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.servers["servers"]));
    if count == 0 && demo() {
        return match index as usize {
            0 => "play.example.net",
            _ => "127.0.0.1:25565",
        }
        .to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.servers["servers"], index as usize, "ip")
    })
}

#[mimas]
fn mod_count() -> f64 {
    let count = with_backend(0, |backend| array_len(&backend.installed["entries"]));
    if count == 0 && demo() {
        return 2.0;
    }
    count as f64
}

#[mimas]
fn mod_title(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.installed["entries"]));
    if count == 0 && demo() {
        return match index as usize {
            0 => "Zoom",
            _ => "WorldEdit",
        }
        .to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.installed["entries"], index as usize, "title")
    })
}

#[mimas]
fn mod_version(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.installed["entries"]));
    if count == 0 && demo() {
        return match index as usize {
            0 => "1.2.0",
            _ => "7.3.1",
        }
        .to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.installed["entries"], index as usize, "version")
    })
}

#[mimas]
fn mod_source(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.installed["entries"]));
    if count == 0 && demo() {
        return "Horizon".to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.installed["entries"], index as usize, "source")
    })
}

#[mimas]
fn mod_verdict(index: f64) -> String {
    with_backend(String::new(), |backend| {
        array_text(&backend.installed["entries"], index as usize, "verdict")
    })
}

/// A loop index as the text an action string uses (`server-remove:0`).
#[mimas]
fn index_text(index: f64) -> String {
    format!("{}", index as usize)
}

#[mimas]
fn account_count() -> f64 {
    let count = with_backend(0, |backend| backend.accounts.offline.len());
    if count == 0 && demo() {
        return 2.0;
    }
    count as f64
}

#[mimas]
fn account_name(index: f64) -> String {
    let count = with_backend(0, |backend| backend.accounts.offline.len());
    if count == 0 && demo() {
        return match index as usize {
            0 => "Player",
            _ => "Builder",
        }
        .to_string();
    }
    with_backend(String::new(), |backend| {
        backend
            .accounts
            .offline
            .get(index as usize)
            .cloned()
            .unwrap_or_default()
    })
}

#[mimas]
fn account_selected(index: f64) -> bool {
    let count = with_backend(0, |backend| backend.accounts.offline.len());
    if count == 0 && demo() {
        return index == 0.0;
    }
    with_backend(false, |backend| {
        backend
            .accounts
            .offline
            .get(index as usize)
            .is_some_and(|name| backend.accounts.offline_selected.as_deref() == Some(name.as_str()))
    })
}

#[mimas]
fn result_count() -> f64 {
    let count = with_backend(0, |backend| array_len(&backend.results["results"]));
    if count == 0 && demo() {
        return 3.0;
    }
    count as f64
}

#[mimas]
fn result_title(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.results["results"]));
    if count == 0 && demo() {
        return match index as usize {
            0 => "Zoom",
            1 => "WorldEdit",
            _ => "Sodium",
        }
        .to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.results["results"], index as usize, "title")
    })
}

#[mimas]
fn result_summary(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.results["results"]));
    if count == 0 && demo() {
        return match index as usize {
            0 => "Hold a key to zoom in; the mouse wheel zooms further.",
            1 => "Edit thousands of blocks at once with brushes and schematics.",
            _ => "A modern rendering engine for Minecraft.",
        }
        .to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.results["results"], index as usize, "summary")
    })
}

#[mimas]
fn result_id(index: f64) -> String {
    with_backend(String::new(), |backend| {
        array_text(&backend.results["results"], index as usize, "id")
    })
}

#[mimas]
fn result_source(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.results["results"]));
    if count == 0 && demo() {
        return "Horizon".to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.results["results"], index as usize, "source")
    })
}

#[mimas]
fn result_support(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.results["results"]));
    if count == 0 && demo() {
        return "Works".to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.results["results"], index as usize, "support")
    })
}

#[mimas]
fn result_installable(index: f64) -> bool {
    let count = with_backend(0, |backend| array_len(&backend.results["results"]));
    if count == 0 && demo() {
        return true;
    }
    with_backend(false, |backend| {
        backend.results["results"]
            .as_array()
            .and_then(|items| items.get(index as usize))
            .and_then(|item| item["installable"].as_bool())
            .unwrap_or(false)
    })
}

#[mimas]
fn instance_count() -> f64 {
    let count = with_backend(0, |backend| array_len(&backend.snapshot["instances"]));
    if count == 0 && demo() {
        return 1.0;
    }
    count as f64
}

#[mimas]
fn instance_name(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.snapshot["instances"]));
    if count == 0 && demo() {
        return "Scoria".to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.snapshot["instances"], index as usize, "name")
    })
}

#[mimas]
fn instance_minecraft(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.snapshot["instances"]));
    if count == 0 && demo() {
        return "26.3".to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.snapshot["instances"], index as usize, "minecraft")
    })
}

#[mimas]
fn instance_id(index: f64) -> String {
    let count = with_backend(0, |backend| array_len(&backend.snapshot["instances"]));
    if count == 0 && demo() {
        return "default".to_string();
    }
    with_backend(String::new(), |backend| {
        array_text(&backend.snapshot["instances"], index as usize, "id")
    })
}

#[mimas]
fn instance_selected(index: f64) -> bool {
    let count = with_backend(0, |backend| array_len(&backend.snapshot["instances"]));
    if count == 0 && demo() {
        return true;
    }
    with_backend(false, |backend| {
        backend.snapshot["instances"]
            .as_array()
            .and_then(|items| items.get(index as usize))
            .and_then(|item| item["selected"].as_bool())
            .unwrap_or(false)
    })
}

/* -------------------------------------------------------------------------------------------
Assets and rendering
------------------------------------------------------------------------------------------- */

fn icon_bytes(name: &str) -> Option<&'static [u8]> {
    Some(match name {
        "play" => include_bytes!("../assets/icons/play.svg"),
        "mods" => include_bytes!("../assets/icons/mods.svg"),
        "servers" => include_bytes!("../assets/icons/servers.svg"),
        "settings" => include_bytes!("../assets/icons/settings.svg"),
        "search" => include_bytes!("../assets/icons/search.svg"),
        "download" => include_bytes!("../assets/icons/download.svg"),
        "check" => include_bytes!("../assets/icons/check.svg"),
        "clock" => include_bytes!("../assets/icons/clock.svg"),
        "chevron" => include_bytes!("../assets/icons/chevron.svg"),
        "map-pin" => include_bytes!("../assets/icons/map-pin.svg"),
        "arrow" => include_bytes!("../assets/icons/arrow-up-right.svg"),
        "stop" => include_bytes!("../assets/icons/stop.svg"),
        "refresh" => include_bytes!("../assets/icons/refresh.svg"),
        "help" => include_bytes!("../assets/icons/help.svg"),
        "menu" => include_bytes!("../assets/icons/menu.svg"),
        "min" => include_bytes!("../assets/icons/win-min.svg"),
        "max" => include_bytes!("../assets/icons/win-max.svg"),
        "close" => include_bytes!("../assets/icons/win-close.svg"),
        "terminal" => include_bytes!("../assets/icons/terminal.svg"),
        _ => return None,
    })
}

fn image_bytes(name: &str) -> Option<(&'static [u8], ImageFormat)> {
    Some(match name {
        "mark" => (
            include_bytes!("../assets/scoria-mark.png"),
            ImageFormat::Png,
        ),
        "backdrop" => (include_bytes!("../assets/backdrop.jpg"), ImageFormat::Jpeg),
        "backdrop-light" => (
            include_bytes!("../assets/backdrop-light.jpg"),
            ImageFormat::Jpeg,
        ),
        _ => return None,
    })
}

fn render_node(node: Node, theme: Theme, cx: &mut Context<Preview>) -> gpui::AnyElement {
    match node {
        Node::Text {
            content,
            size,
            color,
            width,
        } => {
            let mut element = div()
                .text_size(px(size))
                .font_family(theme.type_scale.sans)
                .text_color(color)
                .child(content);
            if let Some(width) = width {
                element = element.max_w(px(width));
            }
            element.into_any_element()
        }
        Node::Button {
            label,
            action,
            fill,
            color,
            small,
            width,
        } => {
            let width = width.unwrap_or(if small { 112.0 } else { 192.0 });
            div()
                .id(format!("mimas-{action}-{label}"))
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0()
                .w(px(width))
                .h(px(if small { 36.0 } else { 52.0 }))
                .px(px(if small { 12.0 } else { 20.0 }))
                .rounded(px(if small { 10.0 } else { 14.0 }))
                .bg(fill)
                .text_color(color)
                .text_size(px(if small { 13.0 } else { 16.0 }))
                .cursor_pointer()
                .hover(move |style| style.opacity(0.9))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.dispatch(action.clone());
                    cx.notify();
                }))
                .child(label)
                .into_any_element()
        }
        Node::Link {
            label,
            action,
            color,
            size,
        } => div()
            .id(format!("mimas-link-{action}-{label}"))
            .text_size(px(size))
            .text_color(color)
            .cursor_pointer()
            .hover(move |style| style.opacity(0.8))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.dispatch(action.clone());
                cx.notify();
            }))
            .child(label)
            .into_any_element(),
        Node::Icon { name, size, color } => icon_bytes(&name)
            .map(|bytes| {
                svg()
                    .data(bytes)
                    .size(px(size))
                    .text_color(color)
                    .into_any_element()
            })
            .unwrap_or_else(|| div().into_any_element()),
        Node::Image {
            name,
            width,
            height,
            radius,
        } => image_bytes(&name)
            .map(|(bytes, format)| {
                img(ImageSource::Image(Arc::new(Image::from_bytes(
                    format,
                    bytes.to_vec(),
                ))))
                .flex_shrink_0()
                .w(px(width))
                .h(px(height))
                .rounded(px(radius))
                .object_fit(ObjectFit::Cover)
                .into_any_element()
            })
            .unwrap_or_else(|| div().into_any_element()),
        Node::Input {
            id,
            placeholder,
            value,
            focused,
            width,
        } => {
            let empty = value.is_empty();
            let shown = if empty {
                placeholder.as_str()
            } else {
                value.as_str()
            };
            let id_for_click = id.clone();
            div()
                .id(format!("mimas-input-{id}"))
                .h(px(40.0))
                .w(px(width))
                .px(px(12.0))
                .flex()
                .items_center()
                .gap(px(6.0))
                .rounded(px(12.0))
                .border_1()
                .border_color(if focused {
                    theme.colors.accent
                } else {
                    theme.colors.border
                })
                .bg(theme.colors.surface_secondary)
                .text_size(px(13.0))
                .text_color(if empty {
                    theme.colors.muted
                } else {
                    theme.colors.foreground
                })
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, window, cx| {
                    *FOCUSED.lock().expect("focus") = Some(id_for_click.clone());
                    window.focus(&this.focus_handle, cx);
                    this.redraw();
                    cx.notify();
                }))
                .children(if focused {
                    Some(
                        div()
                            .w(px(1.5))
                            .h(px(18.0))
                            .bg(theme.colors.accent)
                            .into_any_element(),
                    )
                } else {
                    None
                })
                .child(shown.to_string())
                .into_any_element()
        }
        Node::Spacer => div().flex_grow(1.0).into_any_element(),
        Node::Box {
            direction,
            gap,
            pad_x,
            pad_y,
            width,
            height,
            color,
            radius,
            align,
            justify,
            grow,
            wrap,
            scroll,
            children,
        } => {
            let mut element = div()
                .id(format!(
                    "mimas-box-{}",
                    SCROLL_ID.fetch_add(1, Ordering::Relaxed)
                ))
                .flex()
                .gap(px(gap))
                .px(px(pad_x))
                .py(px(pad_y))
                .rounded(px(radius))
                .bg(color)
                .when(matches!(direction, Direction::Row), |element| {
                    element.flex_row()
                })
                .when(matches!(direction, Direction::Column), |element| {
                    element.flex_col()
                })
                .when(align == Some(Align::Start), |element| element.items_start())
                .when(align == Some(Align::Center), |element| {
                    element.items_center()
                })
                .when(align == Some(Align::End), |element| element.items_end())
                .when(align == Some(Align::Stretch), |element| {
                    element.items_stretch()
                })
                .when(justify == Some(Justify::Start), |element| {
                    element.justify_start()
                })
                .when(justify == Some(Justify::Center), |element| {
                    element.justify_center()
                })
                .when(justify == Some(Justify::End), |element| {
                    element.justify_end()
                })
                .when(justify == Some(Justify::Between), |element| {
                    element.justify_between()
                })
                .when(justify == Some(Justify::Around), |element| {
                    element.justify_around()
                })
                .when(wrap, |element| element.flex_wrap())
                .when(scroll, |element| element.overflow_y_scroll());
            if let Some(width) = width {
                element = element.w(px(width));
            }
            if let Some(height) = height {
                element = element.h(px(height));
            }
            if let Some(grow) = grow {
                element = element.flex_grow(grow);
            }
            element
                .children(
                    children
                        .into_iter()
                        .map(|child| render_node(child, theme, cx)),
                )
                .into_any_element()
        }
    }
}

/* -------------------------------------------------------------------------------------------
The preview window
------------------------------------------------------------------------------------------- */

fn start_scene(source: &str, name: &str) -> Result<(mimas::Vm, Vec<Node>), String> {
    let mut vm = mimas::compile_source(source).map_err(|error| format!("{name}: {error:?}"))?;
    vm.run().map_err(|error| format!("{name}: {error:?}"))?;
    *BUILDER.lock().map_err(|e| e.to_string())? = Some(Builder::default());
    vm.call::<()>("draw", ())
        .map_err(|error| format!("{name}: draw: {error:?}"))?;
    let nodes = std::mem::take(
        &mut BUILDER
            .lock()
            .map_err(|e| e.to_string())?
            .get_or_insert_with(Builder::default)
            .root,
    );
    Ok((vm, nodes))
}

struct Preview {
    theme: Theme,
    nodes: Arc<Mutex<Vec<Node>>>,
    vm: Option<mimas::Vm>,
    script: std::path::PathBuf,
    modified: Option<SystemTime>,
    error: Option<String>,
    focus_handle: FocusHandle,
    revision: u64,
}

impl Preview {
    fn redraw(&mut self) {
        let Some(vm) = self.vm.as_mut() else {
            return;
        };
        *BUILDER.lock().expect("builder") = Some(Builder::default());
        match vm.call::<()>("draw", ()) {
            Ok(()) => {
                let nodes = std::mem::take(
                    &mut BUILDER
                        .lock()
                        .expect("builder")
                        .get_or_insert_with(Builder::default)
                        .root,
                );
                if let Ok(mut scene) = self.nodes.lock() {
                    *scene = nodes;
                }
                self.error = None;
            }
            Err(error) => {
                let error = format!("draw: {error:?}");
                eprintln!("{error}");
                self.error = Some(error);
            }
        }
    }

    fn reload(&mut self) {
        match std::fs::read_to_string(&self.script) {
            Ok(source) => match start_scene(&source, &self.script.display().to_string()) {
                Ok((vm, nodes)) => {
                    self.vm = Some(vm);
                    if let Ok(mut scene) = self.nodes.lock() {
                        *scene = nodes;
                    }
                    self.error = None;
                }
                Err(error) => {
                    eprintln!("{error}");
                    self.error = Some(error);
                }
            },
            Err(error) => {
                let error = format!("{}: {error}", self.script.display());
                eprintln!("{error}");
                self.error = Some(error);
            }
        }
        self.modified = std::fs::metadata(&self.script)
            .and_then(|meta| meta.modified())
            .ok();
    }

    fn spawn_action<F>(&mut self, busy: &str, action: F)
    where
        F: FnOnce() -> Result<String, String> + Send + 'static,
    {
        backend_set_busy(busy);
        self.redraw();
        std::thread::spawn(move || backend_finish(action()));
    }

    fn submit(&mut self, id: &str) {
        match id {
            "mods-search" => self.dispatch("search".into()),
            "server-name" => {
                *FOCUSED.lock().expect("focus") = Some("server-ip".into());
                self.redraw();
            }
            "server-ip" => self.dispatch("server-add".into()),
            "account-name" => self.dispatch("account-add".into()),
            _ => {}
        }
    }

    fn key(&mut self, event: &KeyDownEvent) -> bool {
        if std::env::var_os("SCORIA_MIMAS_DEBUG_KEYS").is_some() {
            eprintln!(
                "key={:?} char={:?} ctrl={} alt={} platform={}",
                event.keystroke.key,
                event.keystroke.key_char,
                event.keystroke.modifiers.control,
                event.keystroke.modifiers.alt,
                event.keystroke.modifiers.platform
            );
        }
        let Some(id) = focused_input() else {
            let key = event.keystroke.key.as_str();
            let control = event.keystroke.modifiers.control || event.keystroke.modifiers.platform;
            if control && matches!(key, "1" | "2" | "3" | "4") {
                let screen = match key {
                    "1" => "home",
                    "2" => "mods",
                    "3" => "servers",
                    _ => "settings",
                };
                self.dispatch(format!("screen:{screen}"));
                return true;
            }
            if control && key == "enter" {
                self.dispatch("play".into());
                return true;
            }
            if key == "escape" {
                self.dispatch("quit".into());
                return true;
            }
            return false;
        };
        let key = event.keystroke.key.as_str();
        if key == "escape" {
            *FOCUSED.lock().expect("focus") = None;
            self.redraw();
            return true;
        }
        if key == "enter" {
            self.submit(&id);
            return true;
        }
        let mut guard = INPUTS.lock().expect("inputs");
        let inputs = guard.get_or_insert_with(HashMap::new);
        let value = inputs.entry(id.clone()).or_default();
        if key == "backspace" || key == "delete" {
            value.pop();
        } else if key == "space" {
            value.push(' ');
        } else if let Some(chars) = event.keystroke.key_char.as_ref() {
            if event.keystroke.modifiers.control
                || event.keystroke.modifiers.alt
                || event.keystroke.modifiers.platform
            {
                return false;
            }
            value.push_str(chars);
        } else {
            return false;
        }
        drop(guard);
        self.redraw();
        true
    }

    fn dispatch(&mut self, action: String) {
        if let Some(screen) = action.strip_prefix("screen:") {
            *SCREEN.lock().expect("screen") = match screen {
                "mods" => Screen::Mods,
                "servers" => Screen::Servers,
                "settings" => Screen::Settings,
                _ => Screen::Home,
            };
            self.redraw();
            return;
        }
        if action == "refresh" {
            if let Ok(mut guard) = BACKEND.lock() {
                if let Some(backend) = guard.as_mut() {
                    backend.refresh();
                    backend.error = None;
                    backend.status = "Обновлено".into();
                }
            }
            self.redraw();
            return;
        }
        if let Some(index) = action.strip_prefix("server-remove:") {
            let index = index.parse::<usize>().unwrap_or(0);
            self.spawn_action("Удаляем сервер…", move || {
                api::remove_saved_server(index)?;
                Ok("Сервер удалён".into())
            });
            return;
        }
        if let Some(ip) = action.strip_prefix("server-join:") {
            let ip = ip.to_string();
            self.spawn_action("Подключаемся…", move || {
                let result = api::start_server(&ip)?;
                Ok(format!(
                    "Игра запущена: {}",
                    result["instance"].as_str().unwrap_or("сервер")
                ))
            });
            return;
        }
        if let Some(folder) = action.strip_prefix("play-world:") {
            let folder = folder.to_string();
            self.spawn_action("Запускаем мир…", move || {
                let result = api::start_game(Some(&folder))?;
                Ok(format!(
                    "Игра запущена: {}",
                    result["instance"].as_str().unwrap_or("мир")
                ))
            });
            return;
        }
        if let Some(id) = action.strip_prefix("install:") {
            let id = id.to_string();
            self.spawn_action("Устанавливаем мод…", move || {
                api::install_project(&id)?;
                Ok("Мод установлен".into())
            });
            return;
        }
        if let Some(name) = action.strip_prefix("account-select:") {
            let name = name.to_string();
            let result = (|| {
                let mut accounts = Accounts::load();
                accounts.play_as(Player::Offline(name.clone()))?;
                accounts.save()?;
                Ok(format!("Играем как {name}"))
            })();
            backend_sync(result);
            self.redraw();
            return;
        }
        if let Some(name) = action.strip_prefix("account-remove:") {
            let name = name.to_string();
            let result = (|| {
                let mut accounts = Accounts::load();
                accounts.remove_offline(&name)?;
                accounts.save()?;
                Ok(format!("{name} удалён"))
            })();
            backend_sync(result);
            self.redraw();
            return;
        }
        if let Some(id) = action.strip_prefix("instance-select:") {
            let id = id.to_string();
            let result = (|| {
                let instance = instances::select(&id)?;
                Ok(format!("Инстанс: {}", instance.name))
            })();
            backend_sync(result);
            self.redraw();
            return;
        }
        if let Some(value) = action.strip_prefix("appearance:") {
            let appearance = match value {
                "light" => Appearance::Light,
                "system" => Appearance::System,
                _ => Appearance::Dark,
            };
            let result = (|| {
                let mut settings = Settings::load();
                settings.appearance = appearance;
                settings.save();
                Ok(format!("Оформление: {value}"))
            })();
            backend_sync(result);
            self.redraw();
            return;
        }
        match action.as_str() {
            "prepare" => {
                *UI.lock().expect("mode") = Mode::Preparing;
                self.spawn_action("Скачиваем и готовим игру…", || {
                    let mut line = String::new();
                    let result = api::prepare_game_with_progress(move |event| {
                        if event["event"] == "progress" {
                            let done = event["done"].as_f64();
                            let total = event["total"].as_f64();
                            if let (Some(done), Some(total)) = (done, total) {
                                if total > 0.0 {
                                    backend_progress(done / total * 100.0, &line);
                                }
                            }
                        } else if event["event"] == "log" {
                            line = event["line"].as_str().unwrap_or_default().to_string();
                            backend_progress(-1.0, &line);
                        }
                    });
                    result?;
                    Ok("Готово к запуску".into())
                });
            }
            "cancel" => {
                api::cancel_prepare();
                *UI.lock().expect("mode") = Mode::Ready;
                backend_set_busy("Отменяем…");
                self.redraw();
            }
            "play" => {
                *UI.lock().expect("mode") = Mode::Playing;
                self.spawn_action("Запускаем игру…", || {
                    let result = api::start_game(None)?;
                    Ok(format!(
                        "Игра запущена: {}",
                        result["instance"].as_str().unwrap_or("Scoria")
                    ))
                });
            }
            "stop" => {
                *UI.lock().expect("mode") = Mode::Ready;
                self.spawn_action("Останавливаем игру…", || {
                    api::stop_game()?;
                    Ok("Игра остановлена".into())
                });
            }
            "install-latest" => {
                self.spawn_action("Устанавливаем сборку…", || {
                    api::install_latest_game()?;
                    Ok("Сборка установлена".into())
                });
            }
            "search" => {
                let query = input_get("mods-search");
                if query.trim().is_empty() {
                    backend_sync(Err("Введите запрос".into()));
                    self.redraw();
                    return;
                }
                self.spawn_action("Ищем в каталоге…", move || {
                    let results = api::search_catalog(&query)?;
                    let found = results["results"].as_array().map(Vec::len).unwrap_or(0);
                    backend_results(results);
                    Ok(format!("Найдено: {found}"))
                });
            }
            "server-add" => {
                let name = input_take("server-name");
                let ip = input_take("server-ip");
                if name.trim().is_empty() || ip.trim().is_empty() {
                    backend_sync(Err("Нужны имя и адрес".into()));
                    self.redraw();
                    return;
                }
                self.spawn_action("Добавляем сервер…", move || {
                    api::add_saved_server(&name, &ip)?;
                    Ok(format!("Сервер {name} добавлен"))
                });
            }
            "quit" => std::process::exit(0),
            "account-add" => {
                let name = input_take("account-name");
                let result = (|| {
                    let mut accounts = Accounts::load();
                    accounts.play_as(Player::Offline(name.clone()))?;
                    accounts.save()?;
                    Ok(format!("Играем как {name}"))
                })();
                backend_sync(result);
                self.redraw();
            }
            _ => {}
        }
    }

    fn watch(&self, cx: &mut Context<Self>) {
        let script = self.script.clone();
        cx.spawn(async move |this, cx| loop {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(250))
                .await;
            let modified = std::fs::metadata(&script)
                .and_then(|meta| meta.modified())
                .ok();
            let revision = REVISION.load(Ordering::Relaxed);
            let alive = this
                .update(cx, |this, cx| {
                    if modified != this.modified {
                        this.reload();
                        cx.notify();
                    } else if revision != this.revision {
                        this.revision = revision;
                        this.redraw();
                        cx.notify();
                    }
                })
                .is_ok();
            if !alive {
                break;
            }
        })
        .detach();
    }
}

impl Render for Preview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let nodes = self
            .nodes
            .lock()
            .map(|nodes| nodes.clone())
            .unwrap_or_default();
        div()
            .relative()
            .size_full()
            .overflow_hidden()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                if this.key(event) {
                    cx.notify();
                }
            }))
            .bg(self.theme.colors.background)
            .children(
                nodes
                    .into_iter()
                    .map(|node| render_node(node, self.theme, cx)),
            )
            .children(self.error.as_ref().map(|error| {
                div()
                    .absolute()
                    .left(px(24.0))
                    .bottom(px(24.0))
                    .max_w(px(900.0))
                    .rounded(px(10.0))
                    .bg(rgba(0x2B1A1AEE))
                    .px(px(14.0))
                    .py(px(10.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0xE5484D))
                    .child(error.clone())
            }))
    }
}

fn main() {
    if let Ok(screen) = std::env::var("SCORIA_MIMAS_SCREEN") {
        *SCREEN.lock().expect("screen") = match screen.as_str() {
            "mods" => Screen::Mods,
            "servers" => Screen::Servers,
            "settings" => Screen::Settings,
            _ => Screen::Home,
        };
    }
    if let Ok(mode) = std::env::var("SCORIA_MIMAS_MODE") {
        *UI.lock().expect("mode") = match mode.as_str() {
            "preparing" => Mode::Preparing,
            "playing" => Mode::Playing,
            _ => Mode::Ready,
        };
    }
    *BACKEND.lock().expect("backend") = Some(Backend::load());
    if let Ok(inputs) = std::env::var("SCORIA_MIMAS_INPUTS") {
        let mut map = HashMap::new();
        for pair in inputs.split(';') {
            if let Some((id, value)) = pair.split_once('=') {
                map.insert(id.trim().to_string(), value.to_string());
            }
        }
        *INPUTS.lock().expect("inputs") = Some(map);
    }
    if let Ok(id) = std::env::var("SCORIA_MIMAS_FOCUS") {
        *FOCUSED.lock().expect("focus") = Some(id);
    }
    let startup_action = std::env::var("SCORIA_MIMAS_ACTION").ok();
    let script = std::env::var_os("SCORIA_MIMAS_UI")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("dev/mimas-ui/scene.mim")
        });
    let nodes = Arc::new(Mutex::new(Vec::new()));

    application().run(move |cx| {
        let bounds = Bounds::new(point(px(0.0), px(0.0)), size(px(1280.0), px(800.0)));
        cx.open_window(
            WindowOptions::new()
                .window_bounds(Some(WindowBounds::Windowed(bounds)))
                .window_decorations(WindowDecorations::Client)
                .focus(true),
            {
                let script = script.clone();
                let nodes = nodes.clone();
                let startup_action = startup_action.clone();
                move |_, cx| {
                    cx.new(|cx| {
                        let mut preview = Preview {
                            theme: Theme::dark(),
                            nodes,
                            vm: None,
                            script,
                            modified: None,
                            error: None,
                            focus_handle: cx.focus_handle(),
                            revision: 0,
                        };
                        preview.reload();
                        if let Some(action) = startup_action.clone() {
                            preview.dispatch(action);
                        }
                        preview.watch(cx);
                        preview
                    })
                }
            },
        )
        .expect("the preview window");
        cx.activate(true);
    });
}

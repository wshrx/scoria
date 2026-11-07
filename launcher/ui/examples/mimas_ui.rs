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
    div, img, point, prelude::*, px, rgb, rgba, size, svg, Bounds, Context, Entity, FocusHandle,
    Focusable, Image, ImageFormat, ImageSource, IntoElement, KeyDownEvent, MouseButton, ObjectFit,
    Render, Rgba, Window, WindowBounds, WindowControlArea, WindowDecorations, WindowOptions,
};
use gpui_ce_elements::editable_text::{
    actions::{default_bindings, DEFAULT_INPUT_CONTEXT},
    text_input, EditableTextState, StringStorage, TextChanged,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        id: String,
        direction: Direction,
        gap: f32,
        pad_x: f32,
        pad_y: f32,
        width: Option<f32>,
        width_pct: Option<f32>,
        height: Option<f32>,
        w_full: bool,
        h_full: bool,
        color: Rgba,
        radius: f32,
        align: Option<Align>,
        justify: Option<Justify>,
        grow: Option<f32>,
        wrap: bool,
        scroll: bool,
        drag: bool,
        border_bottom: Option<(f32, Rgba)>,
        children: Vec<Node>,
    },
    Text {
        content: String,
        size: f32,
        color: Rgba,
        width: Option<f32>,
        fill: bool,
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
        width: f32,
    },
    WindowControl {
        kind: String,
        size: f32,
        color: Rgba,
    },
    Spacer,
}

struct Frame {
    id: String,
    direction: Direction,
    gap: f32,
    pad_x: f32,
    pad_y: f32,
    width: Option<f32>,
    width_pct: Option<f32>,
    height: Option<f32>,
    w_full: bool,
    h_full: bool,
    color: Rgba,
    radius: f32,
    align: Option<Align>,
    justify: Option<Justify>,
    grow: Option<f32>,
    wrap: bool,
    scroll: bool,
    drag: bool,
    border_bottom: Option<(f32, Rgba)>,
    children: Vec<Node>,
}

#[derive(Default)]
struct Builder {
    stack: Vec<Frame>,
    root: Vec<Node>,
    next_id: usize,
}

static BUILDER: Mutex<Option<Builder>> = Mutex::new(None);
static THEME: Mutex<Option<Theme>> = Mutex::new(None);

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

fn theme_from_settings() -> Theme {
    match Settings::load().appearance {
        Appearance::Light => Theme::light(),
        _ => Theme::dark(),
    }
}

fn set_theme(theme: Theme) {
    if let Ok(mut guard) = THEME.lock() {
        *guard = Some(theme);
    }
}

fn current_theme() -> Theme {
    THEME
        .lock()
        .ok()
        .and_then(|guard| *guard)
        .unwrap_or_else(Theme::dark)
}

fn hex(color: Rgba) -> String {
    let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    let rgb = color.color;
    if color.alpha >= 0.999 {
        format!(
            "#{:02X}{:02X}{:02X}",
            channel(rgb.red),
            channel(rgb.green),
            channel(rgb.blue)
        )
    } else {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            channel(rgb.red),
            channel(rgb.green),
            channel(rgb.blue),
            channel(color.alpha)
        )
    }
}

/// A semantic colour from the active launcher theme. The script never hardcodes a palette.
#[mimas]
fn theme_color(role: &str) -> String {
    let colors = current_theme().colors;
    let color = match role {
        "bg" | "background" => colors.background,
        "surface" => colors.surface,
        "surface2" | "surface_secondary" => colors.surface_secondary,
        "surface3" | "surface_tertiary" => colors.surface_tertiary,
        "overlay" => colors.overlay,
        "text" | "foreground" => colors.foreground,
        "muted" => colors.muted,
        "border" => colors.border,
        "separator" => colors.separator,
        "accent" => colors.accent,
        "accent_fg" | "accent_foreground" => colors.accent_foreground,
        "accent_soft" => colors.accent_soft,
        "success" => colors.success,
        "warning" => colors.warning,
        "danger" => colors.danger,
        "transparent" => return "#00000000".to_string(),
        _ => colors.foreground,
    };
    hex(color)
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
    let id = format!("mimas-{}", builder.next_id);
    builder.next_id += 1;
    builder.stack.push(Frame {
        id,
        direction,
        gap: gap as f32,
        pad_x: padding as f32,
        pad_y: padding as f32,
        width: (w > 0.0).then_some(w as f32),
        width_pct: None,
        height: (h > 0.0).then_some(h as f32),
        w_full: false,
        h_full: false,
        color: color(fill),
        radius: radius as f32,
        align: Some(Align::Start),
        justify: Some(Justify::Start),
        grow: None,
        wrap: false,
        scroll: false,
        drag: false,
        border_bottom: None,
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
        id: frame.id,
        direction: frame.direction,
        gap: frame.gap,
        pad_x: frame.pad_x,
        pad_y: frame.pad_y,
        width: frame.width,
        width_pct: frame.width_pct,
        height: frame.height,
        w_full: frame.w_full,
        h_full: frame.h_full,
        color: frame.color,
        radius: frame.radius,
        align: frame.align,
        justify: frame.justify,
        grow: frame.grow,
        wrap: frame.wrap,
        scroll: frame.scroll,
        drag: frame.drag,
        border_bottom: frame.border_bottom,
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

/// The window root: a column that fills the whole window.
#[mimas]
fn screen(fill: &str) {
    begin(Direction::Column, 0.0, 0.0, 0.0, 0.0, fill, 0.0);
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.w_full = true;
        frame.h_full = true;
    }
}

/// Set the current box's width as a fraction of its parent (0.0–1.0).
#[mimas]
fn w_pct(value: f64) {
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.width_pct = Some(value as f32);
    }
}

/// Make the current box fill its parent's width.
#[mimas]
fn fill_w() {
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.w_full = true;
    }
}

/// Make the current box fill its parent's height.
#[mimas]
fn fill_h() {
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.h_full = true;
    }
}

/// A bottom border on the current box.
#[mimas]
fn border_bottom(width: f64, fill: &str) {
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.border_bottom = Some((width as f32, color(fill)));
    }
}

/// Let the current box drag the window.
#[mimas]
fn drag_window() {
    let mut guard = BUILDER.lock().expect("builder");
    if let Some(frame) = guard.get_or_insert_with(Builder::default).stack.last_mut() {
        frame.drag = true;
    }
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
        fill: false,
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
        fill: false,
    });
}

/// Wrapping text that fills the parent's width.
#[mimas]
fn text_fill(content: &str, size: f64, fill: &str) {
    push(Node::Text {
        content: content.to_string(),
        size: size as f32,
        color: color(fill),
        width: None,
        fill: true,
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

fn readable_on(fill: Rgba) -> Rgba {
    let channels = fill.color;
    let luminance = 0.299 * channels.red + 0.587 * channels.green + 0.114 * channels.blue;
    if luminance > 0.5 {
        rgb(0x111214)
    } else {
        rgb(0xF4F0EA)
    }
}

fn button_with_width(label: &str, action: &str, fill: &str, width: f64) {
    let fill = color(fill);
    let text_color = if fill.alpha < 0.01 {
        current_theme().colors.foreground
    } else {
        readable_on(fill)
    };
    push(Node::Button {
        label: label.to_string(),
        action: action.to_string(),
        fill,
        color: text_color,
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
    let text_color = if fill.alpha < 0.01 {
        current_theme().colors.foreground
    } else {
        readable_on(fill)
    };
    push(Node::Button {
        label: label.to_string(),
        action: action.to_string(),
        fill,
        color: text_color,
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

/// An image that fills its parent's width.
#[mimas]
fn image_fill(name: &str, h: f64, radius: f64) {
    push(Node::Image {
        name: name.to_string(),
        width: 0.0,
        height: h as f32,
        radius: radius as f32,
    });
}

/// A real, editable text field backed by `EditableTextState`.
#[mimas]
fn input(id: &str, placeholder: &str, width: f64) {
    push(Node::Input {
        id: id.to_string(),
        placeholder: placeholder.to_string(),
        width: width as f32,
    });
}

/// A real window button: minimize, maximize or close.
#[mimas]
fn window_control(kind: &str, size: f64, fill: &str) {
    push(Node::WindowControl {
        kind: kind.to_string(),
        size: size as f32,
        color: color(fill),
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
    if !appearance.is_empty() {
        return appearance;
    }
    match Settings::load().appearance {
        Appearance::Light => "light".to_string(),
        Appearance::System => "system".to_string(),
        Appearance::Dark => "dark".to_string(),
    }
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

#[derive(Clone)]
enum FocusItem {
    Input(String),
    Action { key: String, action: String },
}

impl FocusItem {
    fn key(&self) -> String {
        match self {
            FocusItem::Input(id) => format!("input:{id}"),
            FocusItem::Action { key, .. } => key.clone(),
        }
    }
}

fn collect_focus(nodes: &[Node], out: &mut Vec<FocusItem>) {
    for node in nodes {
        match node {
            Node::Input { id, .. } => out.push(FocusItem::Input(id.clone())),
            Node::Button { label, action, .. } | Node::Link { label, action, .. } => {
                out.push(FocusItem::Action {
                    key: format!("action:{action}:{label}"),
                    action: action.clone(),
                });
                let _ = label;
            }
            Node::Box { children, .. } => collect_focus(children, out),
            _ => {}
        }
    }
}

fn render_node(
    node: Node,
    theme: Theme,
    inputs: &HashMap<String, Entity<EditableTextState>>,
    focused: Option<&str>,
    cx: &mut Context<Preview>,
) -> gpui::AnyElement {
    match node {
        Node::Text {
            content,
            size,
            color,
            width,
            fill,
        } => {
            let mut element = div()
                .text_size(px(size))
                .font_family(theme.type_scale.sans)
                .text_color(color)
                .child(content);
            if let Some(width) = width {
                element = element.max_w(px(width));
            }
            if fill {
                element = element.w_full().min_w(px(0.0)).whitespace_normal();
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
            let key = format!("action:{action}:{label}");
            let ring = focused == Some(key.as_str());
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
                .border_2()
                .border_color(if ring {
                    theme.colors.foreground
                } else {
                    rgba(0x00000000)
                })
                .bg(fill)
                .text_color(color)
                .text_size(px(if small { 13.0 } else { 16.0 }))
                .cursor_pointer()
                .hover(move |style| style.opacity(0.9))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.focused_key = Some(key.clone());
                    this.dispatch(action.clone(), cx);
                }))
                .child(label)
                .into_any_element()
        }
        Node::Link {
            label,
            action,
            color,
            size,
        } => {
            let key = format!("action:{action}:{label}");
            let ring = focused == Some(key.as_str());
            div()
                .id(format!("mimas-link-{action}-{label}"))
                .px(px(4.0))
                .rounded(px(4.0))
                .border_1()
                .border_color(if ring {
                    theme.colors.accent
                } else {
                    rgba(0x00000000)
                })
                .text_size(px(size))
                .text_color(color)
                .cursor_pointer()
                .hover(move |style| style.opacity(0.8))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.focused_key = Some(key.clone());
                    this.dispatch(action.clone(), cx);
                }))
                .child(label)
                .into_any_element()
        }
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
                let image = img(ImageSource::Image(Arc::new(Image::from_bytes(
                    format,
                    bytes.to_vec(),
                ))))
                .flex_shrink_0()
                .when(width > 0.0, |image| image.w(px(width)))
                .when(width <= 0.0, |image| image.w_full())
                .h(px(height))
                .rounded(px(radius))
                .object_fit(ObjectFit::Cover);
                image.into_any_element()
            })
            .unwrap_or_else(|| div().into_any_element()),
        Node::Input {
            id,
            placeholder,
            width,
        } => {
            let key = format!("input:{id}");
            let ring = focused == Some(key.as_str());
            let Some(state) = inputs.get(&id).cloned() else {
                return div().into_any_element();
            };
            let click_id = id.clone();
            let click_key = key.clone();
            text_input(id)
                .state(state.downgrade())
                .h(px(40.0))
                .w(px(width))
                .px(px(12.0))
                .py(px(10.0))
                .whitespace_nowrap()
                .rounded(px(12.0))
                .border_1()
                .border_color(if ring {
                    theme.colors.accent
                } else {
                    theme.colors.border
                })
                .bg(theme.colors.surface_secondary)
                .text_size(px(13.0))
                .text_color(theme.colors.foreground)
                .placeholder(placeholder)
                .placeholder_color(theme.colors.muted)
                .caret_color(gpui::rgb_to_hsla(theme.colors.accent))
                .selection_color(gpui::rgb_to_hsla(theme.colors.accent_soft))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.focused_key = Some(click_key.clone());
                    if let Some(entity) = this.inputs.get(&click_id).cloned() {
                        window.focus(&entity.read(cx).focus_handle(cx), cx);
                    }
                    this.redraw();
                    cx.notify();
                }))
                .into_any_element()
        }
        Node::WindowControl { kind, size, color } => {
            let control = kind.clone();
            div()
                .id(format!("mimas-window-{kind}"))
                .flex()
                .items_center()
                .justify_center()
                .w(px(size + 12.0))
                .h(px(size + 12.0))
                .rounded(px(6.0))
                .cursor_pointer()
                .hover(move |style| style.bg(rgba(0xFFFFFF10)))
                .on_click(
                    cx.listener(move |_this, _, window, _cx| match control.as_str() {
                        "min" => window.minimize_window(),
                        "max" => window.zoom_window(),
                        _ => window.remove_window(),
                    }),
                )
                .children(icon_bytes(&kind).map(|bytes| {
                    svg()
                        .data(bytes)
                        .size(px(size))
                        .text_color(color)
                        .into_any_element()
                }))
                .into_any_element()
        }
        Node::Spacer => div().flex_grow(1.0).into_any_element(),
        Node::Box {
            id,
            direction,
            gap,
            pad_x,
            pad_y,
            width,
            width_pct,
            height,
            w_full,
            h_full,
            color,
            radius,
            align,
            justify,
            grow,
            wrap,
            scroll,
            drag,
            border_bottom,
            children,
        } => {
            let mut element = div()
                .id(id)
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
                .when(scroll, |element| element.overflow_y_scroll())
                .when(w_full, |element| element.w_full())
                .when(h_full, |element| element.h_full())
                .when(drag, |element| {
                    element
                        .window_control_area(WindowControlArea::Drag)
                        .on_mouse_down(MouseButton::Left, |_, window, _| window.start_window_move())
                });
            if let Some((border_width, border_color)) = border_bottom {
                element = element
                    .border_b(px(border_width))
                    .border_color(border_color);
            }
            if let Some(pct) = width_pct {
                element = element.w(gpui::relative(pct));
            } else if let Some(width) = width {
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
                        .map(|child| render_node(child, theme, inputs, focused, cx)),
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
    inputs: HashMap<String, Entity<EditableTextState>>,
    _subscriptions: Vec<gpui::Subscription>,
    focused_key: Option<String>,
    focus_order: Vec<FocusItem>,
    initial_focus: bool,
}

impl Preview {
    fn new(
        cx: &mut Context<Self>,
        script: std::path::PathBuf,
        nodes: Arc<Mutex<Vec<Node>>>,
    ) -> Self {
        let theme = theme_from_settings();
        set_theme(theme);
        let mut inputs = HashMap::new();
        for id in ["mods-search", "server-name", "server-ip", "account-name"] {
            inputs.insert(
                id.to_string(),
                cx.new(|cx| EditableTextState::new(StringStorage::default(), cx)),
            );
        }
        if let Ok(seeded) = std::env::var("SCORIA_MIMAS_INPUTS") {
            for pair in seeded.split(';') {
                if let Some((id, value)) = pair.split_once('=') {
                    if let Some(entity) = inputs.get(id.trim()).cloned() {
                        entity.update(cx, |state, cx| state.emplace(value, cx));
                        if std::env::var_os("SCORIA_MIMAS_DEBUG_KEYS").is_some() {
                            eprintln!("seeded {}={:?}", id.trim(), entity.read(cx).as_str());
                        }
                    }
                }
            }
        }
        let mut subscriptions = Vec::new();
        for (id, entity) in inputs.clone() {
            subscriptions.push(cx.subscribe(
                &entity,
                move |this, _entity, _event: &TextChanged, cx| {
                    this.focused_key = Some(format!("input:{id}"));
                    cx.notify();
                },
            ));
        }
        let focused_key = std::env::var("SCORIA_MIMAS_FOCUS")
            .ok()
            .map(|id| format!("input:{id}"));
        let mut preview = Self {
            theme,
            nodes,
            vm: None,
            script,
            modified: None,
            error: None,
            focus_handle: cx.focus_handle(),
            revision: 0,
            inputs,
            _subscriptions: subscriptions,
            focused_key,
            focus_order: Vec::new(),
            initial_focus: false,
        };
        preview.reload();
        preview
    }

    fn set_nodes(&mut self, nodes: Vec<Node>) {
        if let Ok(mut scene) = self.nodes.lock() {
            *scene = nodes.clone();
        }
        let mut order = Vec::new();
        collect_focus(&nodes, &mut order);
        self.focus_order = order;
        if let Some(key) = self.focused_key.clone() {
            if !self.focus_order.iter().any(|item| item.key() == key) {
                self.focused_key = None;
            }
        }
    }

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
                self.set_nodes(nodes);
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
                    self.set_nodes(nodes);
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

    fn input_text(&self, id: &str, cx: &Context<Self>) -> String {
        self.inputs
            .get(id)
            .map(|entity| entity.read(cx).as_str().to_string())
            .unwrap_or_default()
    }

    fn take_input(&mut self, id: &str, cx: &mut Context<Self>) -> String {
        let value = self.input_text(id, cx);
        if let Some(entity) = self.inputs.get(id).cloned() {
            entity.update(cx, |state, cx| state.emplace("", cx));
        }
        value
    }

    fn focus_input(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(entity) = self.inputs.get(id).cloned() {
            let handle = entity.read(cx).focus_handle(cx);
            window.focus(&handle, cx);
            self.focused_key = Some(format!("input:{id}"));
            self.redraw();
        }
    }

    fn focused_item(&mut self, window: &Window, cx: &Context<Self>) -> Option<FocusItem> {
        for (id, entity) in &self.inputs {
            if entity.read(cx).focus_handle(cx).is_focused(window) {
                return Some(FocusItem::Input(id.clone()));
            }
        }
        let key = self.focused_key.clone()?;
        self.focus_order
            .iter()
            .find(|item| item.key() == key)
            .cloned()
    }

    fn move_focus(&mut self, backwards: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.focus_order.is_empty() {
            return;
        }
        let current = self
            .focused_item(window, cx)
            .map(|item| item.key())
            .and_then(|key| self.focus_order.iter().position(|item| item.key() == key));
        let count = self.focus_order.len();
        let index = match (current, backwards) {
            (Some(index), false) => (index + 1) % count,
            (Some(index), true) => (index + count - 1) % count,
            (None, false) => 0,
            (None, true) => count - 1,
        };
        let item = self.focus_order[index].clone();
        if std::env::var_os("SCORIA_MIMAS_DEBUG_KEYS").is_some() {
            eprintln!(
                "tab: current={:?} index={} next={:?}",
                current,
                index,
                item.key()
            );
        }
        self.focused_key = Some(item.key());
        match item {
            FocusItem::Input(id) => self.focus_input(&id, window, cx),
            FocusItem::Action { .. } => {
                window.focus(&self.focus_handle, cx);
                self.redraw();
            }
        }
    }

    fn submit(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        match id {
            "mods-search" => self.dispatch("search".into(), cx),
            "server-name" => self.focus_input("server-ip", window, cx),
            "server-ip" => self.dispatch("server-add".into(), cx),
            "account-name" => self.dispatch("account-add".into(), cx),
            _ => {}
        }
    }

    fn handle_key(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
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
        let key = event.keystroke.key.as_str();
        let control = event.keystroke.modifiers.control || event.keystroke.modifiers.platform;
        if control && matches!(key, "1" | "2" | "3" | "4") {
            let screen = match key {
                "1" => "home",
                "2" => "mods",
                "3" => "servers",
                _ => "settings",
            };
            self.dispatch(format!("screen:{screen}"), cx);
            return true;
        }
        if control && key == "enter" {
            self.dispatch("play".into(), cx);
            return true;
        }
        if key == "tab" {
            self.move_focus(event.keystroke.modifiers.shift, window, cx);
            return true;
        }
        if key == "enter" {
            let item = self.focused_item(window, cx);
            if std::env::var_os("SCORIA_MIMAS_DEBUG_KEYS").is_some() {
                eprintln!("enter: item={:?}", item.as_ref().map(FocusItem::key));
            }
            match item {
                Some(FocusItem::Input(id)) => self.submit(&id, window, cx),
                Some(FocusItem::Action { action, .. }) => self.dispatch(action, cx),
                None => {}
            }
            return true;
        }
        if key == "escape" {
            if self
                .focused_item(window, cx)
                .is_some_and(|item| matches!(item, FocusItem::Input(_)))
            {
                window.focus(&self.focus_handle, cx);
                self.focused_key = None;
                self.redraw();
            } else {
                self.dispatch("quit".into(), cx);
            }
            return true;
        }
        false
    }

    fn dispatch(&mut self, action: String, cx: &mut Context<Self>) {
        if std::env::var_os("SCORIA_MIMAS_DEBUG_KEYS").is_some() {
            eprintln!("dispatch {:?} focused={:?}", action, self.focused_key);
        }
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
        if action == "dismiss-error" {
            if let Ok(mut guard) = BACKEND.lock() {
                if let Some(backend) = guard.as_mut() {
                    backend.error = None;
                }
            }
            self.error = None;
            bump_revision();
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
            self.theme = match appearance {
                Appearance::Light => Theme::light(),
                _ => Theme::dark(),
            };
            set_theme(self.theme);
            let label = match value {
                "light" => "светлая",
                "system" => "системная",
                _ => "тёмная",
            };
            let result = (|| {
                let mut settings = Settings::load();
                settings.appearance = appearance;
                settings.save();
                Ok(format!("Оформление: {label}"))
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
                let query = self.input_text("mods-search", cx);
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
                let name = self.take_input("server-name", cx);
                let ip = self.take_input("server-ip", cx);
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
                let name = self.take_input("account-name", cx);
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.initial_focus {
            self.initial_focus = true;
            if let Some(id) = self
                .focused_key
                .clone()
                .and_then(|key| key.strip_prefix("input:").map(str::to_string))
            {
                if let Some(entity) = self.inputs.get(&id).cloned() {
                    let handle = entity.read(cx).focus_handle(cx);
                    window.focus(&handle, cx);
                }
            }
        }
        let nodes = self
            .nodes
            .lock()
            .map(|nodes| nodes.clone())
            .unwrap_or_default();
        let inputs = self.inputs.clone();
        let focused = self.focused_key.clone();
        let theme = self.theme;
        let error = self
            .error
            .clone()
            .or_else(|| with_backend(None, |backend| backend.error.clone()));
        div()
            .relative()
            .size_full()
            .overflow_hidden()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                if this.handle_key(event, window, cx) {
                    cx.stop_propagation();
                    cx.notify();
                }
            }))
            .bg(theme.colors.background)
            .children(
                nodes
                    .into_iter()
                    .map(|node| render_node(node, theme, &inputs, focused.as_deref(), cx)),
            )
            .children(error.map(|error| {
                let text_color = theme.colors.foreground;
                div()
                    .absolute()
                    .right(px(24.0))
                    .bottom(px(56.0))
                    .max_w(px(520.0))
                    .flex()
                    .items_start()
                    .gap(px(12.0))
                    .rounded(px(12.0))
                    .border_1()
                    .border_color(theme.colors.danger)
                    .bg(theme.colors.surface)
                    .px(px(16.0))
                    .py(px(14.0))
                    .child(
                        div()
                            .w(px(3.0))
                            .h(px(28.0))
                            .rounded_full()
                            .bg(theme.colors.danger),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.colors.muted)
                                    .child("Ошибка"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(text_color)
                                    .child(error),
                            ),
                    )
                    .child(
                        div()
                            .id("mimas-toast-close")
                            .w(px(22.0))
                            .h(px(22.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(6.0))
                            .cursor_pointer()
                            .hover(|style| style.bg(theme.colors.surface_secondary))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.dispatch("dismiss-error".into(), cx);
                            }))
                            .child("×"),
                    )
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
    let startup_action = std::env::var("SCORIA_MIMAS_ACTION").ok();
    if std::env::var_os("SCORIA_MIMAS_DEBUG_KEYS").is_some() {
        eprintln!(
            "screen={:?} action={:?}",
            *SCREEN.lock().expect("screen"),
            startup_action
        );
    }
    let script = std::env::var_os("SCORIA_MIMAS_UI")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("dev/mimas-ui/scene.mim")
        });
    let nodes = Arc::new(Mutex::new(Vec::new()));

    application().run(move |cx| {
        cx.bind_keys(default_bindings().as_keybindings(Some(DEFAULT_INPUT_CONTEXT)));
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
                        let mut preview = Preview::new(cx, script, nodes);
                        if let Some(action) = startup_action.clone() {
                            preview.dispatch(action, cx);
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

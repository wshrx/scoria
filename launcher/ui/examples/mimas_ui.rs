//! Experimental Mimas → GPUI launcher preview.
//!
//! The script builds rows, columns, text and buttons; Rust maps the tree to GPUI flex
//! elements. Button actions update a tiny preview state and call `draw()` again, and the
//! file is watched for hot reload.
//!
//!   cargo run -p scoria-ui --features dev-scripts --example mimas-ui
use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use gpui::{
    div, point, prelude::*, px, rgb, rgba, size, Bounds, Context, IntoElement, Render, Rgba,
    Window, WindowBounds, WindowDecorations, WindowOptions,
};
use gpui_ce_platform::application;
use mimas::mimas;
use scoria_ui_kit::Theme;

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

#[derive(Clone)]
enum Node {
    Box {
        direction: Direction,
        gap: f32,
        padding: f32,
        width: Option<f32>,
        height: Option<f32>,
        color: Rgba,
        radius: f32,
        align: Option<Align>,
        justify: Option<Justify>,
        grow: Option<f32>,
        wrap: bool,
        children: Vec<Node>,
    },
    Text {
        content: String,
        size: f32,
        color: Rgba,
    },
    Button {
        label: String,
        action: String,
        fill: Rgba,
        color: Rgba,
    },
}

struct Frame {
    direction: Direction,
    gap: f32,
    padding: f32,
    width: Option<f32>,
    height: Option<f32>,
    color: Rgba,
    radius: f32,
    align: Option<Align>,
    justify: Option<Justify>,
    grow: Option<f32>,
    wrap: bool,
    children: Vec<Node>,
}

#[derive(Default)]
struct Builder {
    stack: Vec<Frame>,
    root: Vec<Node>,
}

static BUILDER: Mutex<Option<Builder>> = Mutex::new(None);

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
        padding: padding as f32,
        width: (w > 0.0).then_some(w as f32),
        height: (h > 0.0).then_some(h as f32),
        color: color(fill),
        radius: radius as f32,
        align: None,
        justify: None,
        grow: None,
        wrap: false,
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
        padding: frame.padding,
        width: frame.width,
        height: frame.height,
        color: frame.color,
        radius: frame.radius,
        align: frame.align,
        justify: frame.justify,
        grow: frame.grow,
        wrap: frame.wrap,
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
    });
}

#[mimas]
fn button(label: &str, action: &str, fill: &str) {
    let fill = color(fill);
    let light = fill.color.red > 0.6 && fill.color.green < 0.6;
    push(Node::Button {
        label: label.to_string(),
        action: action.to_string(),
        fill,
        color: if light { rgb(0x1A0A04) } else { rgb(0xF4F0EA) },
    });
}

#[mimas]
fn preparing() -> bool {
    *UI.lock().expect("mode") == Mode::Preparing
}

#[mimas]
fn playing() -> bool {
    *UI.lock().expect("mode") == Mode::Playing
}

fn render_node(node: Node, theme: Theme, cx: &mut Context<Preview>) -> gpui::AnyElement {
    match node {
        Node::Text {
            content,
            size,
            color,
        } => div()
            .text_size(px(size))
            .font_family(theme.type_scale.sans)
            .text_color(color)
            .child(content)
            .into_any_element(),
        Node::Button {
            label,
            action,
            fill,
            color,
        } => div()
            .id(format!("mimas-{action}"))
            .flex()
            .items_center()
            .justify_center()
            .w(px(192.0))
            .h(px(52.0))
            .px(px(20.0))
            .rounded(px(12.0))
            .bg(fill)
            .text_color(color)
            .text_size(px(16.0))
            .cursor_pointer()
            .hover(move |style| style.opacity(0.9))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.dispatch(action.clone());
                cx.notify();
            }))
            .child(label)
            .into_any_element(),
        Node::Box {
            direction,
            gap,
            padding,
            width,
            height,
            color,
            radius,
            align,
            justify,
            grow,
            wrap,
            children,
        } => {
            let mut element = div()
                .flex()
                .gap(px(gap))
                .p(px(padding))
                .rounded(px(radius))
                .bg(color)
                .when(matches!(direction, Direction::Row), |element| {
                    element.flex_row()
                })
                .when(matches!(direction, Direction::Column), |element| {
                    element.flex_col()
                })
                .when(align == Some(Align::Center), |element| {
                    element.items_center()
                })
                .when(align == Some(Align::End), |element| element.items_end())
                .when(align == Some(Align::Stretch), |element| {
                    element.items_stretch()
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
                .when(wrap, |element| element.flex_wrap());
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
            Err(error) => self.error = Some(format!("draw: {error:?}")),
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
                Err(error) => self.error = Some(error),
            },
            Err(error) => self.error = Some(format!("{}: {error}", self.script.display())),
        }
        self.modified = std::fs::metadata(&self.script)
            .and_then(|meta| meta.modified())
            .ok();
    }

    fn dispatch(&mut self, action: String) {
        let mode = match action.as_str() {
            "prepare" => Mode::Preparing,
            "play" => Mode::Playing,
            _ => Mode::Ready,
        };
        *UI.lock().expect("mode") = mode;
        self.redraw();
    }

    fn watch(&self, cx: &mut Context<Self>) {
        let script = self.script.clone();
        cx.spawn(async move |this, cx| loop {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(300))
                .await;
            let modified = std::fs::metadata(&script)
                .and_then(|meta| meta.modified())
                .ok();
            let alive = this
                .update(cx, |this, cx| {
                    if modified != this.modified {
                        this.reload();
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
    let script = std::env::var_os("SCORIA_MIMAS_UI")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("dev/mimas-ui/scene.mim")
        });
    let nodes = Arc::new(Mutex::new(Vec::new()));

    application().run(move |cx| {
        let bounds = Bounds::new(point(px(0.0), px(0.0)), size(px(1280.0), px(720.0)));
        cx.open_window(
            WindowOptions::new()
                .window_bounds(Some(WindowBounds::Windowed(bounds)))
                .window_decorations(WindowDecorations::Client)
                .focus(true),
            {
                let script = script.clone();
                let nodes = nodes.clone();
                move |_, cx| {
                    cx.new(|cx| {
                        let mut preview = Preview {
                            theme: Theme::dark(),
                            nodes,
                            vm: None,
                            script,
                            modified: None,
                            error: None,
                        };
                        preview.reload();
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

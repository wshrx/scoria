//! Experimental Mimas → GPUI UI tree.
//!
//! The script builds rows, columns, text and spacing; Rust maps the tree to GPUI flex
//! elements. The file is watched, so editing the script repaints the window.
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
        children: Vec<Node>,
    },
    Text {
        content: String,
        size: f32,
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
        children: frame.children,
    };
    match builder.stack.last_mut() {
        Some(parent) => parent.children.push(node),
        None => builder.root.push(node),
    }
}

#[mimas]
fn clear() {
    let mut guard = BUILDER.lock().expect("builder");
    *guard = Some(Builder::default());
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
fn text(content: &str, size: f64, fill: &str) {
    let mut guard = BUILDER.lock().expect("builder");
    let builder = guard.get_or_insert_with(Builder::default);
    let node = Node::Text {
        content: content.to_string(),
        size: size as f32,
        color: color(fill),
    };
    match builder.stack.last_mut() {
        Some(parent) => parent.children.push(node),
        None => builder.root.push(node),
    }
}

fn render_node(node: Node, theme: Theme) -> gpui::AnyElement {
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
        Node::Box {
            direction,
            gap,
            padding,
            width,
            height,
            color,
            radius,
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
                });
            if let Some(width) = width {
                element = element.w(px(width));
            }
            if let Some(height) = height {
                element = element.h(px(height));
            }
            element
                .children(children.into_iter().map(|child| render_node(child, theme)))
                .into_any_element()
        }
    }
}

fn load_scene(source: &str) -> Result<Vec<Node>, String> {
    {
        let mut guard = BUILDER.lock().map_err(|e| e.to_string())?;
        *guard = Some(Builder::default());
    }
    let mut vm = mimas::compile_source(source).map_err(|error| format!("{error:?}"))?;
    vm.run().map_err(|error| format!("{error:?}"))?;
    vm.call::<()>("draw", ())
        .map_err(|error| format!("draw: {error:?}"))?;
    let mut guard = BUILDER.lock().map_err(|e| e.to_string())?;
    Ok(std::mem::take(
        &mut guard.get_or_insert_with(Builder::default).root,
    ))
}

struct Preview {
    theme: Theme,
    nodes: Arc<Mutex<Vec<Node>>>,
    script: std::path::PathBuf,
    modified: Option<SystemTime>,
    error: Option<String>,
}

impl Preview {
    fn reload(&mut self) {
        match std::fs::read_to_string(&self.script) {
            Ok(source) => match load_scene(&source) {
                Ok(nodes) => {
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
            .children(nodes.into_iter().map(|node| render_node(node, self.theme)))
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
    load_scene(include_str!("../dev/mimas-ui/scene.mim")).expect("the default scene");
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

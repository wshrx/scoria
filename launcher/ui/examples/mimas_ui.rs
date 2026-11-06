//! Experimental Mimas → GPUI layout preview.
//!
//! The script owns positions and sizes; Rust only maps the queued nodes to absolute GPUI
//! elements. Run with:
//!   cargo run -p scoria-ui --features dev-scripts --example mimas-ui
//! and point SCORIA_MIMAS_UI at another scene.
use std::sync::Mutex;

use gpui::{
    div, point, prelude::*, px, rgb, rgba, size, Bounds, Context, IntoElement, Render, Rgba,
    Window, WindowBounds, WindowDecorations, WindowOptions,
};
use gpui_ce_platform::application;
use mimas::mimas;
use scoria_ui_kit::Theme;

#[derive(Clone)]
enum Node {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Rgba,
        radius: f32,
    },
    Text {
        x: f32,
        y: f32,
        size: f32,
        color: Rgba,
        content: String,
    },
}

static SCENE: Mutex<Vec<Node>> = Mutex::new(Vec::new());

fn color(value: &str) -> Rgba {
    let hex = value.trim().trim_start_matches('#');
    let parsed = u32::from_str_radix(hex, 16).unwrap_or(0xFFFFFF);
    if hex.len() == 8 {
        rgba(parsed)
    } else {
        rgb(parsed)
    }
}

#[mimas]
fn clear() {
    if let Ok(mut scene) = SCENE.lock() {
        scene.clear();
    }
}

#[mimas]
fn rect(x: f64, y: f64, w: f64, h: f64, fill: &str, radius: f64) {
    if let Ok(mut scene) = SCENE.lock() {
        scene.push(Node::Rect {
            x: x as f32,
            y: y as f32,
            w: w as f32,
            h: h as f32,
            color: color(fill),
            radius: radius as f32,
        });
    }
}

#[mimas]
fn text(x: f64, y: f64, size: f64, fill: &str, content: &str) {
    if let Ok(mut scene) = SCENE.lock() {
        scene.push(Node::Text {
            x: x as f32,
            y: y as f32,
            size: size as f32,
            color: color(fill),
            content: content.to_string(),
        });
    }
}

struct Preview {
    theme: Theme,
    nodes: Vec<Node>,
}

impl Render for Preview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .overflow_hidden()
            .bg(self.theme.colors.background)
            .children(self.nodes.iter().cloned().map(|node| {
                match node {
                    Node::Rect {
                        x,
                        y,
                        w,
                        h,
                        color,
                        radius,
                    } => div()
                        .absolute()
                        .left(px(x))
                        .top(px(y))
                        .w(px(w))
                        .h(px(h))
                        .rounded(px(radius))
                        .bg(color)
                        .into_any_element(),
                    Node::Text {
                        x,
                        y,
                        size,
                        color,
                        content,
                    } => div()
                        .absolute()
                        .left(px(x))
                        .top(px(y))
                        .text_size(px(size))
                        .font_family(self.theme.type_scale.sans)
                        .text_color(color)
                        .child(content)
                        .into_any_element(),
                }
            }))
    }
}

fn main() {
    let script = std::env::var_os("SCORIA_MIMAS_UI")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("dev/mimas-ui/scene.mim")
        });
    let source = std::fs::read_to_string(&script)
        .unwrap_or_else(|error| panic!("{}: {error}", script.display()));
    let mut vm = mimas::compile_source(&source).expect("the scene compiles");
    vm.run().expect("the scene runs");
    vm.call::<()>("draw", ()).expect("scene.draw()");
    let nodes = SCENE.lock().expect("scene queue").clone();

    application().run(move |cx| {
        let bounds = Bounds::new(point(px(0.0), px(0.0)), size(px(1280.0), px(720.0)));
        cx.open_window(
            WindowOptions::new()
                .window_bounds(Some(WindowBounds::Windowed(bounds)))
                .window_decorations(WindowDecorations::Client)
                .focus(true),
            |_, cx| {
                cx.new(|_| Preview {
                    theme: Theme::dark(),
                    nodes: nodes.clone(),
                })
            },
        )
        .expect("the preview window");
        cx.activate(true);
    });
}

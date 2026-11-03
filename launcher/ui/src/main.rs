//! Scoria's launcher window: one process, gpui-ce, no Bun and no native addon.
// The release build is a GUI app: no console window behind the launcher on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod assets;

use std::borrow::Cow;

use gpui::{point, prelude::*, px, size, Bounds, WindowBounds, WindowDecorations, WindowOptions};
use gpui_ce_elements::editable_text::actions::{default_bindings, DEFAULT_INPUT_CONTEXT};
use gpui_ce_platform::application;

use app::{install_crash_log, start_screen, Shell};
use assets::FONTS;

fn main() {
    install_crash_log();
    if std::env::args().any(|arg| arg == "--crash-test") {
        panic!("crash-test: the crash log must contain this line");
    }

    let (screen, query, fullscreen, window_size) = start_screen();
    application().run(move |cx| {
        cx.text_system()
            .add_fonts(FONTS.iter().map(|font| Cow::Borrowed(*font)).collect())
            .expect("the bundled Medeo fonts");

        cx.bind_keys(default_bindings().as_keybindings(Some(DEFAULT_INPUT_CONTEXT)));

        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        // A fixed origin keeps headless screenshots reproducible.
        let window_bounds = if fullscreen {
            let display = cx
                .primary_display()
                .map(|display| display.bounds())
                .unwrap_or_else(|| {
                    Bounds::new(point(px(0.0), px(0.0)), size(px(1280.0), px(720.0)))
                });
            WindowBounds::Fullscreen(display)
        } else {
            WindowBounds::Windowed(Bounds::new(
                point(px(0.0), px(0.0)),
                match window_size {
                    Some((w, h)) => size(px(w), px(h)),
                    None => size(px(1280.0), px(720.0)),
                },
            ))
        };
        cx.open_window(
            WindowOptions::new()
                .window_bounds(Some(window_bounds))
                .window_decorations(WindowDecorations::Client)
                .focus(true)
                .titlebar(None),
            move |_, cx| cx.new(|cx| Shell::new(cx, screen, query.clone())),
        )
        .expect("the launcher window");
        cx.activate(true);
    });
}

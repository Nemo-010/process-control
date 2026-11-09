//! A fast, light task manager.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod apps;
mod config;
mod error;
mod fonts;
mod format;
mod icons;
mod platform;
#[cfg(test)]
mod preview;
mod sample;
mod sampler;
mod theme;
mod ui;

use app::App;
use eframe::egui;

/// Window and taskbar icon, generated from `assets/icon.svg`.
const ICON_PNG: &[u8] = include_bytes!("../assets/icons/256.png");

fn main() -> eframe::Result {
    let smoke = std::env::args().skip(1).any(|arg| arg == "--smoke");
    let icon =
        eframe::icon_data::from_png_bytes(ICON_PNG).expect("the bundled icon is a valid PNG");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(App::TITLE)
            .with_app_id(App::ID)
            .with_inner_size(app::DEFAULT_WINDOW_SIZE)
            .with_min_inner_size(app::MIN_WINDOW_SIZE)
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        App::TITLE,
        options,
        Box::new(move |cc| {
            fonts::install(&cc.egui_ctx);
            Ok(Box::new(App::new(&cc.egui_ctx, smoke)))
        }),
    )
}

//! Top-level application state and the root UI.

use eframe::egui;

/// Frames a smoke run draws before it closes the window.
const SMOKE_FRAMES: u32 = 3;

/// The application state eframe drives once per frame.
pub struct App {
    /// Whether the window closes itself after [`SMOKE_FRAMES`] frames.
    smoke: bool,
    /// Frames drawn so far, counted only during a smoke run.
    frames: u32,
}

impl App {
    /// Window title and human-readable app name.
    pub const TITLE: &str = "App Template";

    /// Application id. It names the desktop entry on Linux, so the
    /// compositor can match the window to its icon.
    pub const ID: &str = "app-template";

    /// Creates the app. `smoke` makes it close itself after a few frames,
    /// which CI uses to prove a build starts and renders.
    pub fn new(smoke: bool) -> Self {
        Self { smoke, frames: 0 }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(Self::TITLE);
        });

        if self.smoke {
            self.frames += 1;
            if self.frames >= SMOKE_FRAMES {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            } else {
                ui.ctx().request_repaint();
            }
        }
    }
}

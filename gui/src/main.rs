mod content_view;
mod log_view;
mod settings_view;

use content_view::ContentView;
use eframe::egui;
use log_view::LogView;
use settings_view::SettingsView;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Mist", options, Box::new(|_cc| Box::new(App::default())))
}

#[derive(Default)]
struct App {
    view: View,
    content: ContentView,
    logs: LogView,
    settings: SettingsView,
}

enum View {
    Content,
    Logs,
    Settings,
}

impl Default for View {
    fn default() -> Self {
        View::Content
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Content").clicked() {
                    self.view = View::Content;
                }
                if ui.button("Logs").clicked() {
                    self.view = View::Logs;
                }
                if ui.button("Settings").clicked() {
                    self.view = View::Settings;
                }
            });
        });

        match self.view {
            View::Content => self.content.ui(ctx, &mut self.logs),
            View::Logs => self.logs.ui(ctx),
            View::Settings => self.settings.ui(ctx),
        }
    }
}

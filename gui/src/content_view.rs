use eframe::egui;
use mist_core::installers;

use crate::log_view::LogView;

#[derive(Default)]
pub struct ContentView {
    installers: Vec<installers::Installer>,
}

impl ContentView {
    pub fn ui(&mut self, ctx: &egui::Context, logs: &mut LogView) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.installers.is_empty() {
                self.installers = installers::list_installers();
            }

            for inst in &self.installers {
                ui.horizontal(|ui| {
                    ui.label(&inst.name);
                    if ui.button("Download").clicked() {
                        logs.push(format!("Requested {}", inst.name));
                    }
                });
            }
        });
    }
}

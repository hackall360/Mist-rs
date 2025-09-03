use eframe::egui;

#[derive(Default)]
pub struct SettingsView {
    pub dark_mode: bool,
}

impl SettingsView {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Settings");
            ui.checkbox(&mut self.dark_mode, "Dark mode");
        });
    }
}

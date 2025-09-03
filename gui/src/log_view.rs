use eframe::egui;

#[derive(Default)]
pub struct LogView {
    entries: Vec<String>,
}

impl LogView {
    pub fn push(&mut self, entry: String) {
        self.entries.push(entry);
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for log in &self.entries {
                    ui.label(log);
                }
            });
        });
    }
}

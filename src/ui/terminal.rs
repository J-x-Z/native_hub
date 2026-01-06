use eframe::egui;

pub struct Terminal {
    // logs: Vec<String>,
}

impl Terminal {
    pub fn new() -> Self {
        Self { }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Terminal Output");
        ui.separator();
        ui.monospace("> System check complete.");
        ui.monospace("> Ready to engage.");
    }
}

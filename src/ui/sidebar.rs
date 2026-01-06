use eframe::egui;

pub struct Sidebar {
    // repo_list: Vec<String>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self { }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Repositories");
        ui.separator();
        ui.label("List coming soon...");
    }
}

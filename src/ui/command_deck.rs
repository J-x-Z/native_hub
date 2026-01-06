use eframe::egui::{self, Color32, RichText, Stroke, Ui, Vec2};

pub struct CommandDeck;

impl CommandDeck {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            
            // Status Header
            ui.label(RichText::new("STATUS: ONLINE").color(Color32::GREEN));
            ui.add_space(10.0);
            
            // Action Grid
            egui::Grid::new("command_deck_grid")
                .spacing(Vec2::new(10.0, 10.0))
                .show(ui, |ui| {
                    if self.action_btn(ui, "⚡ CONNECT", true).clicked() {
                        // Action
                    }
                    if self.action_btn(ui, "📥 PULL", true).clicked() {
                         // Action
                    }
                    if self.action_btn(ui, "📤 PUSH", false).clicked() {
                         // Action
                    }
                    ui.end_row();
                    
                    if self.action_btn(ui, "🔄 SYNC", true).clicked() {
                         // Action
                    }
                    if self.action_btn(ui, "🔎 ISSUES", true).clicked() {
                         // Action
                    }
                    if self.action_btn(ui, "🔧 CONFIG", true).clicked() {
                         // Action
                    }
                    ui.end_row();
                });
        });
    }
    
    fn action_btn(&self, ui: &mut Ui, text: &str, enabled: bool) -> egui::Response {
        let color = if enabled { Color32::from_rgb(0, 255, 136) } else { Color32::GRAY };
        let text = RichText::new(text).color(Color32::WHITE).strong(); // Text is white
        
        // Custom button style
        let btn = egui::Button::new(text)
            .min_size(Vec2::new(100.0, 60.0)) // Big blocky buttons
            .stroke(Stroke::new(1.5, color))
            .fill(Color32::from_black_alpha(150));
            // .rounding(0.0) // Sharp corners for that Sci-Fi look
            
        ui.add_enabled(enabled, btn)
    }
}

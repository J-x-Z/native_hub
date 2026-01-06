use eframe::egui::{self, Color32, RichText, ScrollArea};
use std::collections::VecDeque;

pub struct LogViewer {
    logs: VecDeque<String>,
    max_logs: usize,
}

impl LogViewer {
    pub fn new() -> Self {
        let mut logs = VecDeque::new();
        logs.push_back("> SYSTEM INITIALIZED".to_string());
        logs.push_back("> AWAITING INPUT...".to_string());
        
        Self {
            logs,
            max_logs: 100,
        }
    }

    pub fn add_log(&mut self, msg: String) {
        if self.logs.len() >= self.max_logs {
            self.logs.pop_front();
        }
        // Timestamp could be added here
        self.logs.push_back(format!("> {}", msg));
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label(
                RichText::new("TERMINAL OUT //")
                    .size(10.0)
                    .color(Color32::from_gray(100))
            );
            
            ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 4.0);
                    
                    for log in &self.logs {
                        // TODO: Typewriter effect per line involves tracking state per line. 
                        // For now, static text but with the "Terminal" font style we configured.
                        ui.label(
                            RichText::new(log)
                                .color(Color32::from_rgb(0, 255, 136))
                                .font(egui::FontId::monospace(14.0))
                        );
                    }
                });
        });
    }
}

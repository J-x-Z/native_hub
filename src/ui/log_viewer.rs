use eframe::egui::{self, Color32, RichText, ScrollArea};
use std::collections::VecDeque;
use crate::i18n::I18n;

pub struct LogViewer {
    logs: VecDeque<String>,
    max_logs: usize,
}

impl LogViewer {
    pub fn new() -> Self {
        Self {
            logs: VecDeque::new(),
            max_logs: 100,
        }
    }
    
    pub fn init_logs(&mut self, i18n: &I18n) {
        if self.logs.is_empty() {
            self.logs.push_back(format!("> {}", i18n.t("log.system_online")));
            self.logs.push_back(format!("> {}", i18n.t("log.awaiting")));
        }
    }

    pub fn add_log(&mut self, msg: String) {
        if self.logs.len() >= self.max_logs {
            self.logs.pop_front();
        }
        self.logs.push_back(format!("> {}", msg));
    }

    pub fn show(&mut self, ui: &mut egui::Ui, i18n: &I18n) {
        self.init_logs(i18n);
        
        ui.vertical(|ui| {
            ui.label(
                RichText::new(format!("{} //", i18n.t("log.title")))
                    .size(10.0)
                    .color(Color32::from_gray(100))
            );
            
            ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 4.0);
                    
                    for log in &self.logs {
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

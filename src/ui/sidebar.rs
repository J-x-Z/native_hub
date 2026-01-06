use eframe::egui::{self, Color32, RichText};

pub struct Sidebar {
    pub active_tab: u8, // 0 = Issues, 1 = PRs (used in Browsing view)
}

impl Sidebar {
    pub fn new() -> Self {
        Self { active_tab: 0 }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // App logo/title
            ui.add_space(10.0);
            ui.label(RichText::new("⚡ NativeHub").size(22.0).color(Color32::from_rgb(0, 240, 255)).strong());
            ui.label(RichText::new("GitHub 原生客户端").size(11.0).color(Color32::GRAY));
            ui.add_space(20.0);
            
            ui.separator();
            
            // Navigation hints
            ui.add_space(10.0);
            ui.label(RichText::new("📂 导航").size(14.0).color(Color32::from_rgb(0, 180, 200)));
            ui.add_space(5.0);
            
            ui.label(RichText::new("• 主页 - 查看您的仓库").size(11.0).color(Color32::LIGHT_GRAY));
            ui.label(RichText::new("• 搜索 - 搜索 GitHub 仓库").size(11.0).color(Color32::LIGHT_GRAY));
            
            ui.add_space(20.0);
            ui.separator();
            
            // Quick tips
            ui.add_space(10.0);
            ui.label(RichText::new("💡 提示").size(14.0).color(Color32::from_rgb(0, 180, 200)));
            ui.add_space(5.0);
            
            ui.label(RichText::new("点击仓库卡片进入浏览模式").size(10.0).color(Color32::DARK_GRAY));
            ui.label(RichText::new("右侧面板可切换 Issues/PRs").size(10.0).color(Color32::DARK_GRAY));
            
            ui.add_space(20.0);
            ui.separator();
            
            // Version info at bottom
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                ui.label(RichText::new("v0.1.0").size(10.0).color(Color32::DARK_GRAY));
                ui.label(RichText::new("Made with Rust + egui").size(9.0).color(Color32::from_rgba_unmultiplied(100, 100, 100, 150)));
            });
        });
    }
}

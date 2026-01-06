use eframe::egui::{self, Color32, Rect, RichText, Sense, Stroke, StrokeKind, Ui, Vec2};

pub struct RetroModal;

impl RetroModal {
    pub fn show(
        ctx: &egui::Context,
        title: &str,
        content: impl FnOnce(&mut Ui),
    ) {
        // Overlay entire screen with a semi-transparent dark tint (modal blocker)
        egui::Area::new("modal_backdrop".into())
            .order(egui::Order::Middle) // Above grid, below modal window
            .fixed_pos(egui::Pos2::ZERO)
            .interactable(true) // Block interaction with below layers
            .show(ctx, |ui| {
                ui.painter().rect_filled(
                    ctx.screen_rect(),
                    0.0,
                    Color32::from_black_alpha(200)
                );
            });

        // The Modal Window
        egui::Window::new(title)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .title_bar(false) // We draw our own
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                // Dimensions
                let width = 450.0;
                let height = 350.0;
                
                let (rect, _resp) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
                
                // Draw Custom Tech Border
                let painter = ui.painter();
                let bg_color = Color32::from_rgb(10, 12, 20); // Opaque dark blue
                let border_color = Color32::from_rgb(0, 240, 255); // Cyan
                
                // Fill
                painter.rect_filled(rect, 0.0, bg_color);
                
                // Border
                painter.rect_stroke(
                    rect, 
                    0.0, 
                    Stroke::new(2.0, border_color), 
                    StrokeKind::Middle 
                );
                
                // Glow effect (Outer)
                painter.rect_stroke(
                    rect.expand(2.0), 
                    2.0, 
                    Stroke::new(2.0, border_color.gamma_multiply(0.3)), 
                    StrokeKind::Middle 
                );
                
                // Title Area
                let title_h = 40.0;
                let title_rect = Rect::from_min_size(rect.min, Vec2::new(width, title_h));
                
                // Title Separator
                painter.line_segment(
                    [
                        rect.min + Vec2::new(0.0, title_h),
                        rect.min + Vec2::new(width, title_h)
                    ],
                    Stroke::new(1.0, border_color)
                );
                
                // Title Text
                ui.allocate_new_ui(eframe::egui::UiBuilder::new().max_rect(title_rect), |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new(title)
                                .color(border_color)
                                .size(18.0)
                                .strong()
                        );
                    });
                });
                
                // Content Area
                let content_rect = Rect::from_min_max(
                    rect.min + Vec2::new(20.0, title_h + 20.0),
                    rect.max - Vec2::new(20.0, 20.0)
                );
                
                ui.allocate_new_ui(eframe::egui::UiBuilder::new().max_rect(content_rect), |ui| {
                    ui.vertical(|ui| {
                        content(ui);
                    });
                });
            });
    }
}

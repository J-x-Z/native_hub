use eframe::egui::{self, Color32, Rect, Response, RichText, Sense, Stroke, StrokeKind, Ui, Vec2};
use crate::i18n::{I18n, Lang};

pub enum LoginAction {
    Initiate,
    None,
}

pub fn render_login(ui: &mut Ui, error: &Option<String>, i18n: &mut I18n) -> LoginAction {
    let mut action = LoginAction::None;

    // Language selector at top-right
    egui::TopBottomPanel::top("lang_selector").show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new("🌐").size(16.0));
                egui::ComboBox::from_id_salt("lang_combo")
                    .selected_text(i18n.lang.name())
                    .show_ui(ui, |ui| {
                        for lang in Lang::all() {
                            ui.selectable_value(&mut i18n.lang, *lang, lang.name());
                        }
                    });
            });
        });
    });

    // Use a central layout with fixed width for better control
    ui.vertical_centered(|ui| {
        // Center vertically in the available space (approximate)
        let available_height = ui.available_height();
        ui.add_space(available_height * 0.25); // Push down by 25%

        // 1. The Tech Border Container for Title
        let title_height = 80.0;
        let title_width = 400.0;
        let (rect, _response) = ui.allocate_exact_size(
            Vec2::new(title_width, title_height), 
            Sense::hover()
        );
        
        // Custom Painter for Level 2 Style
        draw_tech_border(ui, rect, Color32::from_rgb(0, 240, 255));
        
        // Draw Text centered in rect
        ui.allocate_new_ui(eframe::egui::UiBuilder::new().max_rect(rect), |ui| {
            ui.centered_and_justified(|ui| {
                 ui.label(
                    RichText::new(i18n.t("app.title"))
                        .font(egui::FontId::proportional(32.0))
                        .color(Color32::from_rgb(0, 240, 255))
                        .strong()
                );
            });
        });

        ui.add_space(60.0);

        if let Some(err) = error {
            ui.colored_label(Color32::from_rgb(255, 0, 128), format!("⚠️  {}: {}", i18n.t("login.error_prefix"), err));
            ui.add_space(20.0);
        }

        // 2. The Login Button (Custom Painted)
        let btn_text = format!("{} {}", i18n.t("login.button_icon"), i18n.t("login.button"));
        if draw_tech_button(ui, &btn_text).clicked() {
            action = LoginAction::Initiate;
        }
    });

    action
}

fn draw_tech_border(ui: &mut Ui, rect: Rect, color: Color32) {
    let painter = ui.painter();
    let stroke = Stroke::new(2.0, color);
    
    // Opaque Background to block grid/particles
    painter.rect_filled(rect, 0.0, Color32::from_rgb(5, 5, 10)); 
    
    // "Bracket" Style
    let w = rect.width();
    let h = rect.height();
    let corner = 20.0;
    
    // ... brackets ...
    // Top Left
    painter.line_segment([rect.min, rect.min + Vec2::new(corner, 0.0)], stroke);
    painter.line_segment([rect.min, rect.min + Vec2::new(0.0, corner)], stroke);

    // Top Right
    painter.line_segment([rect.min + Vec2::new(w, 0.0), rect.min + Vec2::new(w - corner, 0.0)], stroke);
    painter.line_segment([rect.min + Vec2::new(w, 0.0), rect.min + Vec2::new(w, corner)], stroke);
    
    // Bottom Left
    painter.line_segment([rect.min + Vec2::new(0.0, h), rect.min + Vec2::new(0.0, h - corner)], stroke);
    painter.line_segment([rect.min + Vec2::new(0.0, h), rect.min + Vec2::new(corner, h)], stroke);

    // Bottom Right
    painter.line_segment([rect.min + Vec2::new(w, h), rect.min + Vec2::new(w, h - corner)], stroke);
    painter.line_segment([rect.min + Vec2::new(w, h), rect.min + Vec2::new(w - corner, h)], stroke);
    
    // Double lines for "Tech" feel
    let inner_padding = 4.0;
    let inner_rect = rect.shrink(inner_padding);
    let inner_stroke = Stroke::new(1.0, color.gamma_multiply(0.5));
    painter.rect_stroke(inner_rect, 0.0, inner_stroke, StrokeKind::Middle);
}

fn draw_tech_button(ui: &mut Ui, text: &str) -> Response {
    let desired_size = Vec2::new(300.0, 60.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
    
    // Set cursor to pointer on hover
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    
    let (color, _bg_alpha) = if response.hovered() {
        (Color32::from_rgb(0, 255, 255), 1.0)
    } else {
        (Color32::from_rgb(0, 200, 220), 1.0)
    };
    
    let painter = ui.painter();
    
    // Draw Background FIRST to be behind everything
    if response.is_pointer_button_down_on() {
        painter.rect_filled(rect, 4.0, Color32::from_rgb(40, 0, 20));
    } else {
        painter.rect_filled(rect, 4.0, Color32::from_rgb(5, 10, 15));
    }
    
    // Glow effect on hover
    if response.hovered() {
         painter.rect_stroke(rect.expand(2.0), 2.0, Stroke::new(2.0, color.gamma_multiply(0.3)), StrokeKind::Middle);
         painter.rect_stroke(rect.expand(4.0), 4.0, Stroke::new(4.0, color.gamma_multiply(0.1)), StrokeKind::Middle);
    }
    
    // Border
    painter.rect_stroke(rect, 4.0, Stroke::new(1.5, color), StrokeKind::Middle);

    // Draw Icon + Text using painter.text (no nested Ui)
    let icon = "⚡";
    let full_text = format!("{} {}", icon, text);
    
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        full_text,
        egui::FontId::proportional(20.0),
        color,
    );

    response
}

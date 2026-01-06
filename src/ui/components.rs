//! Custom Cyberpunk UI Components
//!
//! CyberButton: A button with "Tactical Corner Brackets" instead of a filled rectangle.

use eframe::egui::{self, Color32, Pos2, Response, RichText, Sense, Stroke, Ui, Vec2};
use super::style::colors;

/// A button with tactical corner brackets (sci-fi style)
pub struct CyberButton {
    text: String,
    min_size: Vec2,
}

impl CyberButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            min_size: Vec2::new(200.0, 50.0),
        }
    }
    
    pub fn min_size(mut self, size: Vec2) -> Self {
        self.min_size = size;
        self
    }
    
    pub fn show(self, ui: &mut Ui) -> Response {
        let desired_size = self.min_size;
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
        
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            
            // Determine state colors
            let (text_color, bg_color, border_color) = if response.is_pointer_button_down_on() {
                // Active: Black on Cyan
                (colors::BG_DARK, colors::ACCENT, colors::ACCENT)
            } else if response.hovered() {
                // Hovered: Cyan glow effect
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                (colors::ACCENT, Color32::from_rgba_unmultiplied(0, 60, 80, 150), colors::ACCENT)
            } else {
                // Inactive: Cyan on transparent
                (colors::ACCENT_DIM, Color32::TRANSPARENT, colors::ACCENT_DIM)
            };
            
            // Draw background (only if not transparent)
            if bg_color != Color32::TRANSPARENT {
                painter.rect_filled(rect, 0.0, bg_color);
            }
            
            // Draw tactical corner brackets
            draw_corner_brackets(painter, rect, border_color, response.hovered());
            
            // Draw text
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &self.text,
                egui::FontId::proportional(18.0),
                text_color,
            );
        }
        
        response
    }
}

/// Draw "tactical corner brackets" - only the 4 corners, not full border
fn draw_corner_brackets(painter: &egui::Painter, rect: egui::Rect, color: Color32, is_hovered: bool) {
    let stroke_width = if is_hovered { 2.0 } else { 1.5 };
    let stroke = Stroke::new(stroke_width, color);
    
    // Corner length (how long each bracket arm is)
    let corner_len = 15.0;
    
    let min = rect.min;
    let max = rect.max;
    let w = rect.width();
    let h = rect.height();
    
    // Top-left corner
    painter.line_segment([Pos2::new(min.x, min.y), Pos2::new(min.x + corner_len, min.y)], stroke);
    painter.line_segment([Pos2::new(min.x, min.y), Pos2::new(min.x, min.y + corner_len)], stroke);
    
    // Top-right corner
    painter.line_segment([Pos2::new(max.x, min.y), Pos2::new(max.x - corner_len, min.y)], stroke);
    painter.line_segment([Pos2::new(max.x, min.y), Pos2::new(max.x, min.y + corner_len)], stroke);
    
    // Bottom-left corner
    painter.line_segment([Pos2::new(min.x, max.y), Pos2::new(min.x + corner_len, max.y)], stroke);
    painter.line_segment([Pos2::new(min.x, max.y), Pos2::new(min.x, max.y - corner_len)], stroke);
    
    // Bottom-right corner
    painter.line_segment([Pos2::new(max.x, max.y), Pos2::new(max.x - corner_len, max.y)], stroke);
    painter.line_segment([Pos2::new(max.x, max.y), Pos2::new(max.x, max.y - corner_len)], stroke);
    
    // If hovered, add a subtle glow line connecting the corners
    if is_hovered {
        let glow_stroke = Stroke::new(1.0, color.gamma_multiply(0.3));
        painter.rect_stroke(rect, 0.0, glow_stroke, egui::StrokeKind::Middle);
    }
}

use eframe::egui::{self, Color32, Painter, Pos2, Rect, Stroke};

/// Draws a retro sci-fi grid background
pub fn draw_retro_grid(painter: &Painter, rect: Rect, time: f64) {
    
    // Grid settings
    let spacing = 40.0;
    let line_width = 1.0;
    let base_color = Color32::from_rgb(0, 50, 30); // Very dark green
    let _highlight_color = Color32::from_rgb(0, 100, 60);
    
    // Scrolling offset
    let scroll_x = (time * 10.0) % spacing as f64;
    let scroll_y = (time * 15.0) % spacing as f64;
    
    // Draw vertical lines
    let mut x = rect.left() - scroll_x as f32;
    while x < rect.right() {
        if x >= rect.left() {
             painter.line_segment(
                [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                Stroke::new(line_width, base_color),
            );
        }
        x += spacing;
    }
    
    // Draw horizontal lines
    let mut y = rect.top() - scroll_y as f32;
    while y < rect.bottom() {
        if y >= rect.top() {
            painter.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                Stroke::new(line_width, base_color),
            );
        }
        y += spacing;
    }
    
}

/// Draws a CRT-style overlay (Scanlines + Vignette)
pub fn draw_crt_overlay(painter: &eframe::egui::Painter, rect: Rect) {
    // 1. Scanlines
    // Draw horizontal lines every few pixels
    let line_spacing = 4.0;
    let line_color = Color32::from_black_alpha(50); // Very subtle
    let stroke = Stroke::new(1.0, line_color);
    
    let mut y = rect.top();
    while y < rect.bottom() {
        painter.line_segment(
            [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
            stroke
        );
        y += line_spacing;
    }
    
    // 2. Vignette (Custom Mesh)
    use eframe::egui::Mesh;
    
    let mut mesh = Mesh::default();
    
    let c = rect.center();
    // Colors
    let color_center = Color32::from_black_alpha(0);
    let color_edge = Color32::from_black_alpha(150); // Dark corners
    
    // Center vertex
    mesh.colored_vertex(c, color_center);
    
    // Corner vertices
    mesh.colored_vertex(rect.min, color_edge); // TL
    mesh.colored_vertex(Pos2::new(rect.right(), rect.top()), color_edge); // TR
    mesh.colored_vertex(rect.max, color_edge); // BR
    mesh.colored_vertex(Pos2::new(rect.left(), rect.bottom()), color_edge); // BL
    
    // Add triangles (indices)
    // 0 is center, 1=TL, 2=TR, 3=BR, 4=BL
    // Fan: 0-1-2, 0-2-3, 0-3-4, 0-4-1
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    mesh.add_triangle(0, 3, 4);
    mesh.add_triangle(0, 4, 1);
    
    painter.add(mesh);
}

/// Renders a "Typewriter" text effect
pub fn typewriter_label(ui: &mut egui::Ui, text: &str, animation_progress: f32) {
    let visible_chars = (text.len() as f32 * animation_progress) as usize;
    let visible_text = &text[..visible_chars.min(text.len())];
    
    let cursor = if animation_progress < 1.0 && (ui.input(|i| i.time) * 2.0) as i32 % 2 == 0 {
        "█"
    } else {
        ""
    };
    
    ui.label(
        egui::RichText::new(format!("{}{}", visible_text, cursor))
            .font(egui::FontId::monospace(14.0))
            .color(Color32::from_rgb(0, 255, 136))
    );
}

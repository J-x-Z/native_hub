pub mod sidebar;
pub mod log_viewer;
pub mod command_deck;
pub mod login_view;
pub mod particles;
pub mod retro_modal;
pub mod repo_browser;
pub mod app;
pub mod effects;

use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily};
pub use app::NativeHubApp;

/// Configure fonts to include CJK support for Chinese language
pub fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    
    // Try to load Microsoft YaHei from Windows fonts folder
    // This is pre-installed on all Windows systems
    let font_path = std::path::Path::new("C:/Windows/Fonts/msyh.ttc");
    
    if let Ok(font_data) = std::fs::read(font_path) {
        fonts.font_data.insert(
            "Microsoft YaHei".to_owned(),
            FontData::from_owned(font_data).into(),
        );
        
        // Add to proportional fonts (for UI text)
        fonts.families
            .entry(FontFamily::Proportional)
            .or_default()
            .push("Microsoft YaHei".to_owned());
            
        tracing::info!("Loaded Microsoft YaHei font for CJK support");
    } else {
        tracing::warn!("Could not load CJK font - Chinese may display as tofu");
    }
    
    ctx.set_fonts(fonts);
}

/// Configure the application style for a geek/terminal aesthetic
pub fn configure_style(ctx: &egui::Context) {
    configure_fonts(ctx);

    let mut style = (*ctx.style()).clone();
    
    // Darker, more "terminal" background colors
    // Slightly transparent to let the retro grid show through
    style.visuals.window_fill = Color32::from_rgba_premultiplied(5, 5, 12, 220); // Deep Cyberspace Blue
    style.visuals.panel_fill = Color32::from_rgba_premultiplied(5, 6, 10, 200);
    
    // Neon accent colors for that cyberpunk feel (Cyan & Magenta)
    style.visuals.hyperlink_color = Color32::from_rgb(0, 240, 255); // Neon Cyan
    style.visuals.selection.bg_fill = Color32::from_rgb(0, 240, 255).linear_multiply(0.3);
    style.visuals.selection.stroke = egui::Stroke::new(1.0, Color32::from_rgb(0, 240, 255));
    
    // Custom button style for standard widgets (if any used)
    style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgba_premultiplied(0, 20, 30, 150);
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Color32::from_rgb(0, 180, 255));
    
    style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgba_premultiplied(0, 50, 60, 200);
    style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, Color32::from_rgb(0, 255, 255));
    
    style.visuals.widgets.active.weak_bg_fill = Color32::from_rgba_premultiplied(0, 80, 100, 250);
    style.visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0, Color32::from_rgb(255, 0, 128)); // Magenta pop for active
    
    // Make lines crisp & Neon
    style.visuals.window_stroke = egui::Stroke::new(1.0, Color32::from_rgb(0, 100, 150));

    ctx.set_style(style);
}



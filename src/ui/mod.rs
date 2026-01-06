pub mod sidebar;
pub mod log_viewer;
pub mod command_deck;
pub mod login_view;
pub mod particles;
pub mod retro_modal;
pub mod repo_browser;
pub mod app;
pub mod effects;

use eframe::egui::{self, Color32};
pub use app::NativeHubApp;

/// Configure fonts to enforce a "Terminal" look with system font fallbacks for CJK.
/// Note: For true CJK support, embed a font file like NotoSansCJK.
pub fn configure_fonts(ctx: &egui::Context) {
    // For now, we use the default fonts which include basic Latin/Symbol support.
    // CJK characters may render as tofu unless a font is embedded.
    // This is a deliberate trade-off to avoid crashes. Localization is a Phase 5 task.
    // The default egui fonts are: Ubuntu-Light (Proportional) and Hack (Monospace).
    // They will be used automatically.
    let _ = ctx; // Suppress unused warning; fonts are set to default.
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



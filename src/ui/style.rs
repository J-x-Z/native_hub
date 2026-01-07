//! UI Style Configuration - Cyberpunk Theme
//! 
//! Colors:
//! - Background: #05080C (Deep Black/Blue)
//! - Primary Accent: #00F0FF (Neon Cyan)
//! - Secondary: #FF003C (Neon Red)

use eframe::egui::{self, Color32, Stroke};

/// Core theme colors
pub mod colors {
    use super::Color32;
    
    /// Deep black/blue background
    pub const BG_DARK: Color32 = Color32::from_rgb(5, 8, 12);
    /// Slightly lighter panel background
    pub const BG_PANEL: Color32 = Color32::from_rgb(8, 12, 18);
    /// Neon cyan primary accent
    pub const ACCENT: Color32 = Color32::from_rgb(0, 240, 255);
    /// Dimmed accent for inactive states
    pub const ACCENT_DIM: Color32 = Color32::from_rgb(0, 120, 128);
    /// Neon red secondary
    pub const SECONDARY: Color32 = Color32::from_rgb(255, 0, 60);
    /// Text color
    pub const TEXT: Color32 = Color32::from_rgb(220, 240, 255);
    /// Muted text
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(100, 120, 140);
}

/// Configure the full Cyberpunk theme
pub fn configure_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // ---------------------
    // COLORS: Cyberpunk Dark
    // ---------------------
    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(colors::TEXT);
    style.visuals.window_fill = colors::BG_DARK;
    style.visuals.panel_fill = colors::BG_PANEL;
    style.visuals.faint_bg_color = Color32::from_rgba_unmultiplied(0, 240, 255, 10);
    style.visuals.extreme_bg_color = colors::BG_DARK;
    
    // Selection
    style.visuals.selection.bg_fill = colors::ACCENT.gamma_multiply(0.3);
    style.visuals.selection.stroke = Stroke::new(1.0, colors::ACCENT);
    
    // Hyperlinks
    style.visuals.hyperlink_color = colors::ACCENT;
    
    // Window border
    style.visuals.window_stroke = Stroke::new(1.0, colors::ACCENT_DIM);
    
    // Selection
    style.visuals.selection.bg_fill = colors::ACCENT.gamma_multiply(0.3);
    style.visuals.selection.stroke = Stroke::new(1.0, colors::ACCENT);
    
    // Hyperlinks
    style.visuals.hyperlink_color = colors::ACCENT;
    
    // Strokes (borders)
    style.visuals.window_stroke = Stroke::new(1.0, colors::ACCENT_DIM);
    
    // ---------------------
    // WIDGETS: Neon Style
    // ---------------------
    
    // Non-interactive (labels, etc.)
    style.visuals.widgets.noninteractive.bg_fill = Color32::TRANSPARENT;
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, colors::TEXT_MUTED);
    
    // Inactive buttons
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgba_unmultiplied(0, 20, 30, 150);
    style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgba_unmultiplied(0, 20, 30, 100);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, colors::ACCENT_DIM);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, colors::ACCENT_DIM);
    
    // Hovered - Glow effect
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgba_unmultiplied(0, 60, 80, 200);
    style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgba_unmultiplied(0, 60, 80, 150);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, colors::ACCENT);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, colors::ACCENT);
    style.visuals.widgets.hovered.expansion = 2.0; // Subtle glow expansion
    
    // Active/Pressed
    style.visuals.widgets.active.bg_fill = colors::ACCENT;
    style.visuals.widgets.active.weak_bg_fill = colors::ACCENT.gamma_multiply(0.8);
    style.visuals.widgets.active.fg_stroke = Stroke::new(2.0, colors::BG_DARK);
    style.visuals.widgets.active.bg_stroke = Stroke::new(2.0, colors::ACCENT);
    
    // Open (dropdown menus, etc.)
    style.visuals.widgets.open.bg_fill = Color32::from_rgba_unmultiplied(0, 40, 60, 220);
    style.visuals.widgets.open.fg_stroke = Stroke::new(1.0, colors::ACCENT);
    style.visuals.widgets.open.bg_stroke = Stroke::new(1.0, colors::ACCENT);
    
    // ---------------------
    // SPACING & SIZING
    // ---------------------
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    
    ctx.set_style(style);
}

/// Configure fonts (called separately because it needs FontDefinitions)
pub fn configure_fonts(ctx: &egui::Context) {
    use egui::{FontData, FontDefinitions, FontFamily};
    
    let mut fonts = FontDefinitions::default();
    
    // Platform-specific CJK font paths
    #[cfg(target_os = "macos")]
    let cjk_font_paths: &[(&str, &str)] = &[
        // PingFang SC - Modern macOS Chinese font (best quality)
        ("PingFang SC", "/System/Library/Fonts/PingFang.ttc"),
        // Hiragino Sans GB - Available on older macOS
        ("Hiragino Sans GB", "/System/Library/Fonts/Hiragino Sans GB.ttc"),
        // STHeiti - Fallback Chinese font
        ("STHeiti", "/System/Library/Fonts/STHeiti Medium.ttc"),
    ];
    
    #[cfg(target_os = "windows")]
    let cjk_font_paths: &[(&str, &str)] = &[
        ("Microsoft YaHei", "C:/Windows/Fonts/msyh.ttc"),
        ("SimHei", "C:/Windows/Fonts/simhei.ttf"),
    ];
    
    #[cfg(target_os = "linux")]
    let cjk_font_paths: &[(&str, &str)] = &[
        ("Noto Sans CJK SC", "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"),
        ("WenQuanYi Micro Hei", "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc"),
    ];
    
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    let cjk_font_paths: &[(&str, &str)] = &[];
    
    // Collect successfully loaded CJK fonts
    let mut loaded_cjk_fonts: Vec<String> = Vec::new();
    
    for (font_name, font_path) in cjk_font_paths {
        let path = std::path::Path::new(font_path);
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                font_name.to_string(),
                FontData::from_owned(font_data).into(),
            );
            loaded_cjk_fonts.push(font_name.to_string());
            tracing::info!("Loaded CJK font: {} from {}", font_name, font_path);
        }
    }
    
    // Insert CJK fonts at the BEGINNING of font families for proper priority
    // This ensures CJK characters are rendered with CJK fonts, not fallback boxes
    if !loaded_cjk_fonts.is_empty() {
        // Get existing fonts and prepend CJK fonts
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            let existing = fonts.families.entry(family).or_default();
            let mut new_list = loaded_cjk_fonts.clone();
            new_list.extend(existing.drain(..));
            *existing = new_list;
        }
    }
    
    ctx.set_fonts(fonts);
}

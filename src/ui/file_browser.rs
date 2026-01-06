//! File Browser UI Component
//!
//! Displays a file tree and code viewer for browsing repository contents.

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};
use crate::app_event::{AppAction, FileNode};
use crate::i18n::I18n;
use tokio::sync::mpsc::Sender;

use super::style::colors;
use super::components::CyberButton;

/// Render the file browser UI
pub fn render_file_browser(
    ui: &mut egui::Ui,
    i18n: &I18n,
    repo_name: &str,
    current_path: &str,
    files: &[FileNode],
    viewing_code: &Option<(String, String)>,
    action_tx: &Sender<AppAction>,
) -> Option<BrowserAction> {
    let mut action = None;
    
    ui.vertical(|ui| {
        // Header with repo name and back button
        ui.horizontal(|ui| {
            // Back button
            if CyberButton::new("← 返回").min_size(Vec2::new(80.0, 35.0)).show(ui).clicked() {
                if current_path.is_empty() {
                    action = Some(BrowserAction::BackToRepoList);
                } else {
                    // Go up one directory level
                    let parent = parent_path(current_path);
                    action = Some(BrowserAction::NavigateTo(parent));
                }
            }
            
            ui.add_space(10.0);
            
            // Repo name / path breadcrumb
            ui.label(RichText::new(format!("📁 {} /{}", repo_name, current_path))
                .size(16.0)
                .color(colors::ACCENT));
        });
        
        ui.separator();
        
        // Check if viewing code
        if let Some((filename, content)) = viewing_code {
            // Code viewer
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("📄 {}", filename)).size(14.0).color(colors::ACCENT));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("✕ 关闭").clicked() {
                        action = Some(BrowserAction::CloseViewer);
                    }
                });
            });
            
            ui.separator();
            
            // Scrollable code area
            ScrollArea::both().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut content.as_str())
                        .font(egui::FontId::monospace(13.0))
                        .desired_width(f32::INFINITY)
                        .interactive(false)
                );
            });
        } else {
            // File list
            ScrollArea::vertical().show(ui, |ui| {
                ui.set_width(ui.available_width());
                
                // Sort: directories first, then files
                let mut sorted_files = files.to_vec();
                sorted_files.sort_by(|a, b| {
                    match (&a.node_type[..], &b.node_type[..]) {
                        ("dir", "file") => std::cmp::Ordering::Less,
                        ("file", "dir") => std::cmp::Ordering::Greater,
                        _ => a.name.cmp(&b.name),
                    }
                });
                
                for file in &sorted_files {
                    let is_dir = file.node_type == "dir";
                    let icon = if is_dir { "📁" } else { file_icon(&file.name) };
                    
                    let response = ui.add(
                        egui::Button::new(RichText::new(format!("{} {}", icon, file.name)).size(14.0))
                            .fill(Color32::TRANSPARENT)
                            .min_size(Vec2::new(ui.available_width(), 30.0))
                    );
                    
                    if response.clicked() {
                        if is_dir {
                            action = Some(BrowserAction::NavigateTo(file.path.clone()));
                        } else if let Some(ref url) = file.download_url {
                            action = Some(BrowserAction::OpenFile(file.name.clone(), url.clone()));
                        }
                    }
                    
                    if response.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                }
            });
        }
    });
    
    action
}

/// Actions that can be triggered from the file browser
pub enum BrowserAction {
    BackToRepoList,
    NavigateTo(String),  // path
    OpenFile(String, String), // (filename, download_url)
    CloseViewer,
}

/// Get parent path from a path string
fn parent_path(path: &str) -> String {
    if let Some(pos) = path.rfind('/') {
        path[..pos].to_string()
    } else {
        String::new()
    }
}

/// Get file icon based on extension
fn file_icon(filename: &str) -> &'static str {
    let ext = filename.rsplit('.').next().unwrap_or("");
    match ext.to_lowercase().as_str() {
        "rs" => "🦀",
        "py" => "🐍",
        "js" | "ts" | "jsx" | "tsx" => "📜",
        "html" | "css" => "🌐",
        "json" | "toml" | "yaml" | "yml" => "⚙️",
        "md" => "📝",
        "txt" => "📄",
        "png" | "jpg" | "jpeg" | "gif" | "svg" => "🖼️",
        _ => "📄",
    }
}

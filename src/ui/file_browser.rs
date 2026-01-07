//! File Browser UI Component
//!
//! Displays a file tree, repo info, and README for browsing repository contents.

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use crate::app_event::{AppAction, FileNode, RepoInfo};
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
    repo_info: &Option<RepoInfo>,
    readme_content: &Option<String>,
    action_tx: &Sender<AppAction>,
    markdown_cache: &mut CommonMarkCache,
) -> Option<BrowserAction> {
    let action = std::cell::RefCell::new(None);
    
    ui.vertical(|ui| {
        // ==================
        // HEADER: Repo Info
        // ==================
        ui.horizontal(|ui| {
            // Back button
            if CyberButton::new("← 返回").min_size(Vec2::new(80.0, 35.0)).show(ui).clicked() {
                if current_path.is_empty() {
                    *action.borrow_mut() = Some(BrowserAction::BackToRepoList);
                } else {
                    let parent = parent_path(current_path);
                    *action.borrow_mut() = Some(BrowserAction::NavigateTo(parent));
                }
            }
            
            ui.add_space(10.0);
            
            // Repo name and path
            ui.label(RichText::new(format!("📁 {} /{}", repo_name, current_path))
                .size(16.0)
                .color(colors::ACCENT));
            
            // Stats on the right
            if let Some(info) = repo_info {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("🍴 {}", info.forks_count))
                        .size(12.0).color(Color32::GRAY));
                    ui.add_space(15.0);
                    ui.label(RichText::new(format!("⭐ {}", info.stargazers_count))
                        .size(12.0).color(Color32::from_rgb(255, 215, 0)));
                    
                    if let Some(lang) = &info.language {
                        ui.add_space(15.0);
                        ui.label(RichText::new(format!("🔤 {}", lang))
                            .size(12.0).color(colors::ACCENT_DIM));
                    }
                });
            }
        });
        
        // Description
        if current_path.is_empty() {
            if let Some(info) = repo_info {
                if let Some(desc) = &info.description {
                    if !desc.is_empty() {
                        ui.add_space(5.0);
                        ui.label(RichText::new(desc).size(12.0).color(Color32::GRAY).italics());
                    }
                }
                
                // Topics
                if !info.topics.is_empty() {
                    ui.add_space(5.0);
                    ui.horizontal_wrapped(|ui| {
                        for topic in &info.topics {
                            ui.label(
                                RichText::new(format!(" {} ", topic))
                                    .size(10.0)
                                    .color(colors::ACCENT)
                                    .background_color(Color32::from_rgba_unmultiplied(0, 240, 255, 30))
                            );
                        }
                    });
                }
            }
        }
        
        ui.separator();
        
        // ==================
        // MAIN CONTENT
        // ==================
        if let Some((filename, content)) = viewing_code {
            // Code viewer mode
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("📄 {}", filename)).size(14.0).color(colors::ACCENT));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("✕ 关闭").clicked() {
                        *action.borrow_mut() = Some(BrowserAction::CloseViewer);
                    }
                });
            });
            
            ui.separator();
            
            ScrollArea::both().show(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.monospace(content);
            });
        } else {
            // Two-column layout: Files | README
            ui.columns(2, |columns| {
                // LEFT: File list
                columns[0].vertical(|ui| {
                    ui.label(RichText::new("📂 文件").size(12.0).color(colors::TEXT_MUTED));
                    ui.separator();
                    
                    ScrollArea::vertical().id_salt("file_list").show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        
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
                                egui::Button::new(RichText::new(format!("{} {}", icon, file.name)).size(13.0))
                                    .fill(Color32::TRANSPARENT)
                                    .min_size(Vec2::new(ui.available_width(), 26.0))
                            );
                            
                            if response.clicked() {
                                if is_dir {
                                    *action.borrow_mut() = Some(BrowserAction::NavigateTo(file.path.clone()));
                                } else if let Some(ref url) = file.download_url {
                                    *action.borrow_mut() = Some(BrowserAction::OpenFile(file.name.clone(), url.clone()));
                                }
                            }
                            
                            if response.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            }
                        }
                    });
                });
                
                // RIGHT: README
                columns[1].vertical(|ui| {
                    ui.label(RichText::new("📝 README").size(12.0).color(colors::TEXT_MUTED));
                    ui.separator();
                    
                    ScrollArea::vertical().id_salt("readme_panel").show(ui, |ui| {
                        if let Some(readme) = readme_content {
                            // Convert HTML to Markdown for rendering
                            // (transforms <img> tags to markdown image syntax for fetch)
                            let converted_readme = html_to_markdown(readme);
                            CommonMarkViewer::new().show(ui, markdown_cache, &converted_readme);
                        } else {
                            ui.colored_label(Color32::GRAY, "无 README 文件");
                        }
                    });
                });
            });
        }
    });
    
    action.into_inner()
}

/// Actions that can be triggered from the file browser
pub enum BrowserAction {
    BackToRepoList,
    NavigateTo(String),
    OpenFile(String, String),
    CloseViewer,
}

fn parent_path(path: &str) -> String {
    if let Some(pos) = path.rfind('/') {
        path[..pos].to_string()
    } else {
        String::new()
    }
}

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

/// Convert HTML in README to clean Markdown for egui_commonmark rendering
/// Removes HTML tags (especially images) that can't be rendered natively
fn html_to_markdown(content: &str) -> String {
    let mut result = content.to_string();
    
    // Remove <img> tags completely (they can't be rendered reliably)
    let mut output = String::new();
    let mut remaining = result.as_str();
    
    while let Some(start) = remaining.find("<img") {
        // Add content before the tag
        output.push_str(&remaining[..start]);
        
        // Find the end of the tag and skip it
        if let Some(end_offset) = remaining[start..].find('>') {
            remaining = &remaining[start + end_offset + 1..];
        } else {
            remaining = &remaining[start + 4..];
        }
    }
    output.push_str(remaining);
    result = output;
    
    // Remove <a> tags but keep content (links are preserved as text)
    result = remove_tag_keep_content(&result, "a");
    
    // Remove <div>, <p>, <span> but keep content
    result = remove_tag_keep_content(&result, "div");
    result = remove_tag_keep_content(&result, "p");
    result = remove_tag_keep_content(&result, "span");
    result = remove_tag_keep_content(&result, "h1");
    result = remove_tag_keep_content(&result, "h2");
    result = remove_tag_keep_content(&result, "h3");
    result = remove_tag_keep_content(&result, "br");
    result = remove_tag_keep_content(&result, "hr");
    
    // Remove HTML comments <!-- ... -->
    while let Some(start) = result.find("<!--") {
        if let Some(end) = result[start..].find("-->") {
            result = format!("{}{}", &result[..start], &result[start + end + 3..]);
        } else {
            break;
        }
    }
    
    // Remove Markdown image syntax ![alt](url) since we can't display images
    let mut output = String::new();
    let mut remaining = result.as_str();
    while let Some(start) = remaining.find("![") {
        output.push_str(&remaining[..start]);
        // Find the closing ]
        if let Some(bracket_end) = remaining[start..].find("](") {
            // Find the closing )
            if let Some(paren_end) = remaining[start + bracket_end..].find(')') {
                remaining = &remaining[start + bracket_end + paren_end + 1..];
                continue;
            }
        }
        // Not a valid image syntax, keep it
        output.push_str(&remaining[start..start + 2]);
        remaining = &remaining[start + 2..];
    }
    output.push_str(remaining);
    result = output;
    
    // Clean up excessive whitespace
    let lines: Vec<&str> = result.lines().collect();
    let mut cleaned_lines = Vec::new();
    let mut prev_empty = false;
    
    for line in lines {
        let trimmed = line.trim();
        let is_empty = trimmed.is_empty();
        if is_empty {
            if !prev_empty {
                cleaned_lines.push("");
            }
            prev_empty = true;
        } else {
            cleaned_lines.push(trimmed);
            prev_empty = false;
        }
    }
    
    cleaned_lines.join("\n")
}

/// Extract an attribute value from an HTML tag
fn extract_attr(tag: &str, attr_name: &str) -> Option<String> {
    let search = format!("{}=\"", attr_name);
    if let Some(start) = tag.find(&search) {
        let value_start = start + search.len();
        if let Some(end_offset) = tag[value_start..].find('"') {
            return Some(tag[value_start..value_start + end_offset].to_string());
        }
    }
    // Try single quotes
    let search_single = format!("{}='", attr_name);
    if let Some(start) = tag.find(&search_single) {
        let value_start = start + search_single.len();
        if let Some(end_offset) = tag[value_start..].find('\'') {
            return Some(tag[value_start..value_start + end_offset].to_string());
        }
    }
    None
}

/// Remove HTML tags but keep the content inside
fn remove_tag_keep_content(content: &str, tag_name: &str) -> String {
    let mut result = content.to_string();
    
    // Remove opening tags like <tag ...>
    let open_pattern = format!("<{}", tag_name);
    while let Some(start) = result.to_lowercase().find(&open_pattern) {
        if let Some(end_offset) = result[start..].find('>') {
            result = format!("{}{}", &result[..start], &result[start + end_offset + 1..]);
        } else {
            break;
        }
    }
    
    // Remove closing tags like </tag>
    let close_pattern = format!("</{}>", tag_name);
    result = result.replace(&close_pattern, "");
    // Also handle uppercase
    result = result.replace(&close_pattern.to_uppercase(), "");
    
    result
}

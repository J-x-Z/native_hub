//! Search UI Component
//!
//! Provides search functionality for finding repositories on GitHub.

use eframe::egui::{self, Color32, RichText, ScrollArea, Sense, Stroke, Vec2};
use crate::app_event::{AppAction, SearchRepoItem};
use crate::i18n::I18n;
use tokio::sync::mpsc::Sender;

use super::style::colors;
use super::components::CyberButton;

/// Search panel state
pub struct SearchPanel {
    pub query: String,
    pub results: Vec<SearchRepoItem>,
    pub searching: bool,
    action_tx: Sender<AppAction>,
}

impl SearchPanel {
    pub fn new(action_tx: Sender<AppAction>) -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
            searching: false,
            action_tx,
        }
    }
    
    pub fn set_results(&mut self, results: Vec<SearchRepoItem>) {
        self.results = results;
        self.searching = false;
    }
    
    /// Show the search panel. Returns Some(full_name) if a repo was clicked.
    pub fn show(&mut self, ui: &mut egui::Ui, i18n: &I18n) -> Option<String> {
        let mut selected = None;
        
        ui.vertical(|ui| {
            // Search Header
            ui.horizontal(|ui| {
                ui.label(RichText::new("🔍 搜索仓库").size(18.0).color(colors::ACCENT).strong());
            });
            
            ui.add_space(10.0);
            
            // Search Input Row
            ui.horizontal(|ui| {
                // Input field with cyberpunk styling
                let input = egui::TextEdit::singleline(&mut self.query)
                    .hint_text("输入关键词搜索 (如: rust async)")
                    .desired_width(ui.available_width() - 100.0)
                    .font(egui::FontId::proportional(14.0));
                
                let response = ui.add(input);
                
                // Search on Enter key
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if !self.query.trim().is_empty() {
                        self.searching = true;
                        let _ = self.action_tx.try_send(AppAction::SearchRepos(self.query.clone()));
                    }
                }
                
                // Search button
                if self.searching {
                    ui.spinner();
                } else {
                    if CyberButton::new("搜索").min_size(Vec2::new(80.0, 30.0)).show(ui).clicked() {
                        if !self.query.trim().is_empty() {
                            self.searching = true;
                            let _ = self.action_tx.try_send(AppAction::SearchRepos(self.query.clone()));
                        }
                    }
                }
            });
            
            ui.separator();
            
            // Results count
            if !self.results.is_empty() {
                ui.label(RichText::new(format!("找到 {} 个结果", self.results.len()))
                    .size(12.0).color(colors::TEXT_MUTED));
                ui.add_space(5.0);
            }
            
            // Results list
            ScrollArea::vertical().id_salt("search_results").show(ui, |ui| {
                ui.set_width(ui.available_width());
                
                for repo in &self.results {
                    if let Some(full_name) = self.render_search_result(ui, repo) {
                        selected = Some(full_name);
                    }
                    ui.add_space(6.0);
                }
                
                if self.results.is_empty() && !self.searching && !self.query.is_empty() {
                    ui.colored_label(Color32::GRAY, "无搜索结果");
                }
            });
        });
        
        selected
    }
    
    fn render_search_result(&self, ui: &mut egui::Ui, repo: &SearchRepoItem) -> Option<String> {
        let h = 70.0;
        let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::click());
        
        let painter = ui.painter();
        let is_hovered = response.hovered();
        
        // Hover effect
        let bg_color = if is_hovered {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            Color32::from_rgba_unmultiplied(0, 50, 60, 180)
        } else {
            Color32::from_rgb(8, 12, 18)
        };
        
        let border_color = if is_hovered {
            colors::ACCENT
        } else {
            Color32::from_rgb(0, 60, 60)
        };
        
        // Background
        painter.rect_filled(rect, 4.0, bg_color);
        painter.rect_stroke(rect, 4.0, Stroke::new(1.0, border_color), egui::StrokeKind::Middle);
        
        // Status strip
        let strip_color = if repo.is_private {
            Color32::from_rgb(255, 140, 0)
        } else {
            colors::ACCENT
        };
        let strip_rect = egui::Rect::from_min_size(rect.min, Vec2::new(3.0, rect.height()));
        painter.rect_filled(strip_rect, 0.0, strip_color.gamma_multiply(if is_hovered { 1.0 } else { 0.6 }));
        
        // Content
        let content_rect = rect.shrink2(Vec2::new(12.0, 8.0));
        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    // Repo full name
                    ui.label(RichText::new(&repo.full_name).size(14.0).color(Color32::WHITE).strong());
                    
                    // Description (truncated)
                    if let Some(desc) = &repo.description {
                        let desc_text = if desc.len() > 80 {
                            format!("{}...", &desc[..80])
                        } else {
                            desc.clone()
                        };
                        ui.label(RichText::new(desc_text).size(11.0).color(Color32::GRAY));
                    }
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Stats
                    ui.label(RichText::new(format!("🍴 {}", repo.forks_count)).size(10.0).color(Color32::GRAY));
                    ui.add_space(10.0);
                    ui.label(RichText::new(format!("⭐ {}", repo.stargazers_count)).size(10.0).color(Color32::from_rgb(255, 215, 0)));
                    
                    // Language
                    if let Some(lang) = &repo.language {
                        ui.add_space(10.0);
                        ui.label(RichText::new(lang).size(10.0).color(colors::ACCENT_DIM));
                    }
                });
            });
        });
        
        if response.clicked() {
            let _ = self.action_tx.try_send(AppAction::SelectRepo(repo.full_name.clone()));
            return Some(repo.full_name.clone());
        }
        
        None
    }
}

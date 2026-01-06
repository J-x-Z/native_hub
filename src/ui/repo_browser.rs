use eframe::egui::{self, Color32, RichText, Sense, Stroke, Vec2};
use crate::app_event::{AppAction, RepoData};
use crate::i18n::I18n;
use tokio::sync::mpsc::Sender;

pub struct RepoBrowser {
    pub repos: Vec<RepoData>,
    pub loading: bool,
    action_tx: Sender<AppAction>,
}

impl RepoBrowser {
    pub fn new(action_tx: Sender<AppAction>) -> Self {
        Self {
            repos: Vec::new(),
            loading: false,
            action_tx,
        }
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }
    
    pub fn set_repos(&mut self, repos: Vec<RepoData>) {
        self.repos = repos;
        self.loading = false;
    }

    /// Returns Some(full_name) if a repo was clicked
    pub fn show(&mut self, ui: &mut egui::Ui, i18n: &I18n) -> Option<String> {
        let mut selected = None;
        
        ui.vertical(|ui| {
            self.render_header(ui, i18n);
            ui.add_space(10.0);
            selected = self.render_list(ui, i18n);
        });
        
        selected
    }

    fn render_header(&mut self, ui: &mut egui::Ui, i18n: &I18n) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(i18n.t("repos.title")).size(20.0).color(Color32::from_rgb(0, 240, 255)).strong());
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Refresh Button
                let refresh_btn = if self.loading {
                    ui.spinner();
                    ui.add_enabled(false, egui::Button::new(i18n.t("repos.loading")))
                } else {
                    ui.button(format!("♻️ {}", i18n.t("repos.refresh")))
                };
                
                if refresh_btn.clicked() {
                    self.loading = true;
                    // Trigger backend fetch
                     let _ = self.action_tx.try_send(AppAction::FetchRepos);
                }
            });
        });
        
        ui.separator();
    }

    fn render_list(&mut self, ui: &mut egui::Ui, i18n: &I18n) -> Option<String> {
        if self.loading && self.repos.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label(i18n.t("repos.loading"));
            });
            return None;
        }

        if self.repos.is_empty() {
             ui.centered_and_justified(|ui| {
                ui.label(i18n.t("repos.empty"));
            });
            return None;
        }
        
        let mut clicked_repo = None;
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(ui.available_width());
            
            for repo in &self.repos {
                if let Some(full_name) = self.render_repo_card(ui, repo) {
                    clicked_repo = Some(full_name);
                }
                ui.add_space(8.0);
            }
        });
        
        clicked_repo
    }

    fn render_repo_card(&self, ui: &mut egui::Ui, repo: &RepoData) -> Option<String> {
        let h = 80.0;
        let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::click());
        
        let painter = ui.painter();
        let is_hovered = response.hovered();
        
        // Hover Effect - Cyan glow background
        let bg_color = if is_hovered {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            Color32::from_rgba_unmultiplied(0, 40, 50, 180) // Faint cyan glow
        } else {
            Color32::from_rgb(5, 8, 12) // Dark background
        };
        
        let border_color = if is_hovered {
            Color32::from_rgb(0, 255, 255) // Bright Cyan
        } else {
            Color32::from_rgb(0, 80, 80) // Dim Cyan
        };
        
        // Background
        painter.rect_filled(rect, 4.0, bg_color);
        painter.rect_stroke(rect, 4.0, Stroke::new(1.0, border_color), eframe::egui::StrokeKind::Middle);
        
        // Status Strip (2px vertical line on left)
        let strip_color = if repo.is_private {
            Color32::from_rgb(255, 140, 0) // Orange for private
        } else {
            Color32::from_rgb(0, 240, 255) // Cyan for public
        };
        let strip_brightness = if is_hovered { 1.0 } else { 0.6 };
        let strip_rect = egui::Rect::from_min_size(
            rect.min,
            Vec2::new(3.0, rect.height())
        );
        painter.rect_filled(strip_rect, 0.0, strip_color.gamma_multiply(strip_brightness));
        
        // Content
        let content_rect = rect.shrink2(Vec2::new(12.0, 10.0));
        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
            ui.horizontal(|ui| {
                // Icon
                let icon = if repo.is_private { "🔒" } else { "🌐" };
                ui.label(RichText::new(icon).size(24.0));
                
                ui.vertical(|ui| {
                    // Repo name
                    ui.label(RichText::new(&repo.name).size(16.0).color(Color32::WHITE).strong());
                    
                    // Description (truncated)
                    let desc = if repo.description.len() > 60 {
                        format!("{}...", &repo.description[..60])
                    } else {
                        repo.description.clone()
                    };
                    ui.label(RichText::new(desc).size(11.0).color(Color32::GRAY));
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Updated time
                    ui.label(RichText::new(&repo.last_updated).size(10.0).color(Color32::from_rgb(0, 180, 200)).italics());
                    
                    ui.add_space(10.0);
                    
                    // Stars & Forks (if we have the data - use placeholder for now)
                    ui.label(RichText::new("⭐ --").size(10.0).color(Color32::from_rgb(255, 215, 0)));
                    ui.label(RichText::new("🍴 --").size(10.0).color(Color32::GRAY));
                });
            });
        });
        
        // Handle click - return the full_name for file browsing
        if response.clicked() {
            let _ = self.action_tx.try_send(AppAction::SelectRepo(repo.full_name.clone()));
            return Some(repo.full_name.clone());
        }
        
        None
    }
}

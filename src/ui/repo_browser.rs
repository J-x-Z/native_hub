use eframe::egui::{self, Color32, RichText, Sense, Stroke, Vec2};
use crate::app_event::{AppAction, RepoData};
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

    pub fn show(&mut self, ui: &mut egui::Ui) {
        // Defines the main layout
        ui.vertical(|ui| {
            self.render_header(ui);
            ui.add_space(10.0);
            self.render_list(ui);
        });
    }

    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("REPOSITORIES").size(20.0).color(Color32::from_rgb(0, 240, 255)).strong());
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Refresh Button
                let refresh_btn = if self.loading {
                    ui.spinner();
                    ui.add_enabled(false, egui::Button::new("Connecting..."))
                } else {
                    ui.button("♻️ REFRESH")
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

    fn render_list(&mut self, ui: &mut egui::Ui) {
        if self.loading && self.repos.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("Accessing GitHub Uplink...");
            });
            return;
        }

        if self.repos.is_empty() {
             ui.centered_and_justified(|ui| {
                ui.label("No Data Stream. Click Refresh.");
            });
            return;
        }
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(ui.available_width());
            
            for repo in &self.repos {
                self.render_repo_card(ui, repo);
                ui.add_space(8.0);
            }
        });
    }

    fn render_repo_card(&self, ui: &mut egui::Ui, repo: &RepoData) {
        let h = 80.0;
        let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::click());
        
        let painter = ui.painter();
        
        // Hover Effect
        let border_color = if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
             Color32::from_rgb(0, 255, 255) // Cyan
        } else {
             Color32::from_rgb(0, 100, 100) // Dim Cyan
        };
        
        // Background
        painter.rect_filled(rect, 4.0, Color32::from_rgb(5, 8, 12));
        painter.rect_stroke(rect, 4.0, Stroke::new(1.0, border_color), eframe::egui::StrokeKind::Middle);
        
        // Content
        let content_rect = rect.shrink(10.0);
        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
            ui.horizontal(|ui| {
                // Icon
                let icon = if repo.is_private { "🔒" } else { "🌐" };
                ui.label(RichText::new(icon).size(24.0));
                
                ui.vertical(|ui| {
                    ui.label(RichText::new(&repo.name).size(16.0).color(Color32::WHITE).strong());
                    ui.label(RichText::new(&repo.description).size(12.0).color(Color32::GRAY));
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(&repo.last_updated).size(10.0).color(Color32::from_rgb(0, 240, 255)).italics());
                });
            });
        });
        
        // Handle click - open repo in browser
        if response.clicked() {
            let _ = self.action_tx.try_send(AppAction::SelectRepo(repo.name.clone()));
        }
    }
}

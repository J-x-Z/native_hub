//! Pull Requests UI Component
//!
//! Displays pull requests list and allows merge/close actions.

use eframe::egui::{self, Color32, RichText, ScrollArea, Sense, Stroke, Vec2};
use crate::app_event::{AppAction, PullRequest, MergeResult};
use crate::i18n::I18n;
use tokio::sync::mpsc::Sender;

use super::style::colors;
use super::components::CyberButton;

/// Pull Requests panel
pub struct PullRequestsPanel {
    pub pull_requests: Vec<PullRequest>,
    pub loading: bool,
    pub current_repo: String,
    pub filter_state: String, // "open", "closed", "all"
    
    // Detail view
    pub selected_pr: Option<PullRequest>,
    
    action_tx: Sender<AppAction>,
}

impl PullRequestsPanel {
    pub fn new(action_tx: Sender<AppAction>) -> Self {
        Self {
            pull_requests: Vec::new(),
            loading: false,
            current_repo: String::new(),
            filter_state: "open".to_string(),
            selected_pr: None,
            action_tx,
        }
    }
    
    pub fn set_repo(&mut self, repo: String) {
        if self.current_repo != repo {
            self.current_repo = repo.clone();
            self.pull_requests.clear();
            self.selected_pr = None;
            self.loading = true;
            let _ = self.action_tx.try_send(AppAction::FetchPullRequests(repo, self.filter_state.clone()));
        }
    }
    
    pub fn set_pull_requests(&mut self, prs: Vec<PullRequest>) {
        self.pull_requests = prs;
        self.loading = false;
    }
    
    pub fn on_pr_merged(&mut self, _result: MergeResult) {
        // Refresh the list after merge
        self.loading = true;
        let _ = self.action_tx.try_send(AppAction::FetchPullRequests(
            self.current_repo.clone(),
            self.filter_state.clone()
        ));
        self.selected_pr = None;
    }
    
    pub fn on_pr_closed(&mut self, pr: PullRequest) {
        // Update in list
        if let Some(pos) = self.pull_requests.iter().position(|p| p.number == pr.number) {
            self.pull_requests[pos] = pr.clone();
        }
        self.selected_pr = None;
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, _i18n: &I18n) {
        if self.selected_pr.is_some() {
            self.show_detail(ui);
        } else {
            self.show_list(ui);
        }
    }
    
    fn show_list(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Header
            ui.horizontal(|ui| {
                ui.label(RichText::new("🔀 Pull Requests").size(18.0).color(colors::ACCENT).strong());
                
                ui.add_space(20.0);
                
                // Filter buttons
                for (label, state) in [("Open", "open"), ("Closed", "closed"), ("All", "all")] {
                    let is_selected = self.filter_state == state;
                    let text_color = if is_selected { colors::ACCENT } else { Color32::GRAY };
                    
                    if ui.add(egui::Button::new(RichText::new(label).color(text_color))
                        .fill(if is_selected { Color32::from_rgba_unmultiplied(0, 60, 80, 100) } else { Color32::TRANSPARENT })
                    ).clicked() {
                        self.filter_state = state.to_string();
                        self.loading = true;
                        let _ = self.action_tx.try_send(AppAction::FetchPullRequests(
                            self.current_repo.clone(),
                            state.to_string()
                        ));
                    }
                }
                
                if self.loading {
                    ui.spinner();
                }
            });
            
            ui.separator();
            
            // PR list
            ScrollArea::vertical().id_salt("pr_list").show(ui, |ui| {
                ui.set_width(ui.available_width());
                
                if self.pull_requests.is_empty() && !self.loading {
                    ui.colored_label(Color32::GRAY, "暂无 Pull Requests");
                }
                
                for pr in &self.pull_requests {
                    if self.render_pr_card(ui, pr) {
                        self.selected_pr = Some(pr.clone());
                    }
                    ui.add_space(4.0);
                }
            });
        });
    }
    
    fn render_pr_card(&self, ui: &mut egui::Ui, pr: &PullRequest) -> bool {
        let h = 65.0;
        let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::click());
        
        let painter = ui.painter();
        let is_hovered = response.hovered();
        
        let bg_color = if is_hovered {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            Color32::from_rgba_unmultiplied(50, 30, 60, 180)
        } else {
            Color32::from_rgb(12, 8, 18)
        };
        
        // Background
        painter.rect_filled(rect, 4.0, bg_color);
        
        // Status strip - magenta for PRs
        let strip_color = if pr.merged {
            Color32::from_rgb(150, 80, 200) // Purple for merged
        } else if pr.state == "open" {
            Color32::from_rgb(0, 200, 100) // Green for open
        } else {
            Color32::from_rgb(200, 80, 80) // Red for closed
        };
        let strip_rect = egui::Rect::from_min_size(rect.min, Vec2::new(3.0, rect.height()));
        painter.rect_filled(strip_rect, 0.0, strip_color);
        
        // Border
        painter.rect_stroke(rect, 4.0, Stroke::new(1.0, if is_hovered { Color32::from_rgb(200, 100, 200) } else { Color32::from_rgb(60, 40, 60) }), egui::StrokeKind::Middle);
        
        // Content
        let content_rect = rect.shrink2(Vec2::new(12.0, 6.0));
        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    // Title
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("#{}", pr.number)).size(12.0).color(Color32::GRAY));
                        ui.label(RichText::new(&pr.title).size(13.0).color(Color32::WHITE).strong());
                    });
                    
                    // Branch info
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{} ← {}", pr.base.ref_name, pr.head.ref_name))
                            .size(10.0).color(Color32::from_rgb(150, 100, 200)));
                        
                        // Stats
                        ui.label(RichText::new(format!("+{} -{}", pr.additions, pr.deletions))
                            .size(10.0).color(Color32::GRAY));
                    });
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Status badge
                    let (status_text, status_color) = if pr.merged {
                        ("MERGED", Color32::from_rgb(150, 80, 200))
                    } else if pr.state == "open" {
                        ("OPEN", Color32::from_rgb(0, 200, 100))
                    } else {
                        ("CLOSED", Color32::from_rgb(200, 80, 80))
                    };
                    ui.label(RichText::new(status_text).size(10.0).color(status_color).strong());
                });
            });
        });
        
        response.clicked()
    }
    
    fn show_detail(&mut self, ui: &mut egui::Ui) {
        let pr = self.selected_pr.clone().unwrap();
        
        ui.vertical(|ui| {
            // Back button + title
            ui.horizontal(|ui| {
                if CyberButton::new("← 返回").min_size(Vec2::new(80.0, 30.0)).show(ui).clicked() {
                    self.selected_pr = None;
                }
                
                ui.add_space(10.0);
                ui.label(RichText::new(format!("PR #{} {}", pr.number, pr.title))
                    .size(16.0).color(Color32::from_rgb(200, 100, 200)).strong());
            });
            
            ui.separator();
            
            ScrollArea::vertical().id_salt("pr_detail").show(ui, |ui| {
                ui.set_width(ui.available_width());
                
                // Branch info
                ui.group(|ui| {
                    ui.label(RichText::new("分支信息").size(14.0).color(colors::ACCENT_DIM));
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Base:").color(Color32::GRAY));
                        ui.label(RichText::new(&pr.base.ref_name).color(Color32::WHITE));
                        ui.add_space(20.0);
                        ui.label(RichText::new("Head:").color(Color32::GRAY));
                        ui.label(RichText::new(&pr.head.ref_name).color(Color32::WHITE));
                    });
                });
                
                ui.add_space(10.0);
                
                // Stats
                ui.group(|ui| {
                    ui.label(RichText::new("统计").size(14.0).color(colors::ACCENT_DIM));
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("📝 {} commits", pr.commits)).size(12.0));
                        ui.add_space(20.0);
                        ui.label(RichText::new(format!("📁 {} files changed", pr.changed_files)).size(12.0));
                        ui.add_space(20.0);
                        ui.label(RichText::new(format!("+{}", pr.additions)).size(12.0).color(Color32::from_rgb(100, 200, 100)));
                        ui.label(RichText::new(format!("-{}", pr.deletions)).size(12.0).color(Color32::from_rgb(200, 100, 100)));
                    });
                });
                
                ui.add_space(10.0);
                
                // Body
                ui.group(|ui| {
                    ui.label(RichText::new("描述").size(14.0).color(colors::ACCENT_DIM));
                    ui.separator();
                    if let Some(body) = &pr.body {
                        ui.style_mut().wrap = Some(true);
                        ui.label(body);
                    } else {
                        ui.colored_label(Color32::GRAY, "(无描述)");
                    }
                });
                
                ui.add_space(20.0);
                
                // Actions
                if pr.state == "open" && !pr.merged {
                    ui.horizontal(|ui| {
                        if CyberButton::new("🔀 Merge (merge)").min_size(Vec2::new(120.0, 35.0)).show(ui).clicked() {
                            let _ = self.action_tx.try_send(AppAction::MergePullRequest(
                                self.current_repo.clone(),
                                pr.number,
                                "merge".to_string()
                            ));
                        }
                        
                        ui.add_space(10.0);
                        
                        if CyberButton::new("🔀 Squash").min_size(Vec2::new(100.0, 35.0)).show(ui).clicked() {
                            let _ = self.action_tx.try_send(AppAction::MergePullRequest(
                                self.current_repo.clone(),
                                pr.number,
                                "squash".to_string()
                            ));
                        }
                        
                        ui.add_space(10.0);
                        
                        if CyberButton::new("🔀 Rebase").min_size(Vec2::new(100.0, 35.0)).show(ui).clicked() {
                            let _ = self.action_tx.try_send(AppAction::MergePullRequest(
                                self.current_repo.clone(),
                                pr.number,
                                "rebase".to_string()
                            ));
                        }
                        
                        ui.add_space(30.0);
                        
                        if CyberButton::new("❌ 关闭 PR").min_size(Vec2::new(100.0, 35.0)).show(ui).clicked() {
                            let _ = self.action_tx.try_send(AppAction::ClosePullRequest(
                                self.current_repo.clone(),
                                pr.number
                            ));
                        }
                    });
                } else {
                    let status = if pr.merged { "已合并" } else { "已关闭" };
                    ui.label(RichText::new(format!("此 PR {}", status)).size(14.0).color(Color32::GRAY));
                }
            });
        });
    }
}

//! Issues UI Component
//!
//! Displays issues list, issue details, comments, and allows actions.

use eframe::egui::{self, Color32, RichText, ScrollArea, Sense, Stroke, TextEdit, Vec2};
use crate::app_event::{AppAction, Issue, IssueComment, IssueLabel};
use crate::i18n::I18n;
use tokio::sync::mpsc::Sender;

use super::style::colors;
use super::components::CyberButton;

/// Issues panel - displays issues for a repository
pub struct IssuesPanel {
    pub issues: Vec<Issue>,
    pub loading: bool,
    pub current_repo: String,
    pub filter_state: String, // "open", "closed", "all"
    
    // Detail view
    pub selected_issue: Option<Issue>,
    pub comments: Vec<IssueComment>,
    pub loading_comments: bool,
    pub new_comment: String,
    
    action_tx: Sender<AppAction>,
}

impl IssuesPanel {
    pub fn new(action_tx: Sender<AppAction>) -> Self {
        Self {
            issues: Vec::new(),
            loading: false,
            current_repo: String::new(),
            filter_state: "open".to_string(),
            selected_issue: None,
            comments: Vec::new(),
            loading_comments: false,
            new_comment: String::new(),
            action_tx,
        }
    }
    
    pub fn set_repo(&mut self, repo: String) {
        if self.current_repo != repo {
            self.current_repo = repo.clone();
            self.issues.clear();
            self.selected_issue = None;
            self.comments.clear();
            self.loading = true;
            let _ = self.action_tx.try_send(AppAction::FetchIssues(repo, self.filter_state.clone()));
        }
    }
    
    pub fn set_issues(&mut self, issues: Vec<Issue>) {
        self.issues = issues;
        self.loading = false;
    }
    
    pub fn set_comments(&mut self, issue_number: u32, comments: Vec<IssueComment>) {
        if let Some(ref issue) = self.selected_issue {
            if issue.number == issue_number {
                self.comments = comments;
                self.loading_comments = false;
            }
        }
    }
    
    pub fn add_comment(&mut self, comment: IssueComment) {
        self.comments.push(comment);
        self.new_comment.clear();
    }
    
    pub fn update_issue(&mut self, updated: Issue) {
        // Update in list
        if let Some(pos) = self.issues.iter().position(|i| i.number == updated.number) {
            self.issues[pos] = updated.clone();
        }
        // Update selected
        if let Some(ref mut selected) = self.selected_issue {
            if selected.number == updated.number {
                *selected = updated;
            }
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, i18n: &I18n) {
        if self.selected_issue.is_some() {
            self.show_detail(ui, i18n);
        } else {
            self.show_list(ui, i18n);
        }
    }
    
    fn show_list(&mut self, ui: &mut egui::Ui, _i18n: &I18n) {
        ui.vertical(|ui| {
            // Header
            ui.horizontal(|ui| {
                ui.label(RichText::new("📋 Issues").size(18.0).color(colors::ACCENT).strong());
                
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
                        let _ = self.action_tx.try_send(AppAction::FetchIssues(
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
            
            // Issues list
            ScrollArea::vertical().id_salt("issues_list").show(ui, |ui| {
                ui.set_width(ui.available_width());
                
                if self.issues.is_empty() && !self.loading {
                    ui.colored_label(Color32::GRAY, "暂无 Issues");
                }
                
                for issue in &self.issues {
                    if self.render_issue_card(ui, issue) {
                        self.selected_issue = Some(issue.clone());
                        self.comments.clear();
                        self.loading_comments = true;
                        let _ = self.action_tx.try_send(AppAction::FetchIssueComments(
                            self.current_repo.clone(),
                            issue.number
                        ));
                    }
                    ui.add_space(4.0);
                }
            });
        });
    }
    
    fn render_issue_card(&self, ui: &mut egui::Ui, issue: &Issue) -> bool {
        let h = 60.0;
        let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::click());
        
        let painter = ui.painter();
        let is_hovered = response.hovered();
        
        let bg_color = if is_hovered {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            Color32::from_rgba_unmultiplied(0, 50, 60, 180)
        } else {
            Color32::from_rgb(8, 12, 18)
        };
        
        // Background
        painter.rect_filled(rect, 4.0, bg_color);
        
        // Status strip
        let strip_color = if issue.state == "open" {
            Color32::from_rgb(0, 200, 100) // Green for open
        } else {
            Color32::from_rgb(150, 80, 150) // Purple for closed
        };
        let strip_rect = egui::Rect::from_min_size(rect.min, Vec2::new(3.0, rect.height()));
        painter.rect_filled(strip_rect, 0.0, strip_color);
        
        // Border
        painter.rect_stroke(rect, 4.0, Stroke::new(1.0, if is_hovered { colors::ACCENT } else { Color32::from_rgb(0, 60, 60) }), egui::StrokeKind::Middle);
        
        // Content
        let content_rect = rect.shrink2(Vec2::new(12.0, 6.0));
        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    // Title
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("#{}", issue.number)).size(12.0).color(Color32::GRAY));
                        ui.label(RichText::new(&issue.title).size(13.0).color(Color32::WHITE).strong());
                    });
                    
                    // Labels
                    ui.horizontal_wrapped(|ui| {
                        for label in &issue.labels {
                            let color = parse_label_color(&label.color);
                            ui.label(RichText::new(&label.name).size(10.0).color(color)
                                .background_color(color.gamma_multiply(0.2)));
                        }
                        
                        // Comment count
                        if issue.comments > 0 {
                            ui.label(RichText::new(format!("💬 {}", issue.comments)).size(10.0).color(Color32::GRAY));
                        }
                    });
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(&issue.user.login).size(10.0).color(Color32::DARK_GRAY));
                });
            });
        });
        
        response.clicked()
    }
    
    fn show_detail(&mut self, ui: &mut egui::Ui, _i18n: &I18n) {
        let issue = self.selected_issue.clone().unwrap();
        
        ui.vertical(|ui| {
            // Back button + title
            ui.horizontal(|ui| {
                if CyberButton::new("← 返回").min_size(Vec2::new(80.0, 30.0)).show(ui).clicked() {
                    self.selected_issue = None;
                    self.comments.clear();
                }
                
                ui.add_space(10.0);
                ui.label(RichText::new(format!("#{} {}", issue.number, issue.title))
                    .size(16.0).color(colors::ACCENT).strong());
            });
            
            ui.separator();
            
            // Issue body + comments
            ScrollArea::vertical().id_salt("issue_detail").show(ui, |ui| {
                ui.set_width(ui.available_width());
                
                // Issue body
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&issue.user.login).size(12.0).color(colors::ACCENT_DIM));
                        ui.label(RichText::new(&issue.created_at[..10]).size(10.0).color(Color32::DARK_GRAY));
                    });
                    ui.separator();
                    if let Some(body) = &issue.body {
                        ui.style_mut().wrap = Some(true);
                        ui.label(body);
                    } else {
                        ui.colored_label(Color32::GRAY, "(无描述)");
                    }
                });
                
                ui.add_space(10.0);
                
                // Comments
                ui.label(RichText::new(format!("💬 评论 ({})", self.comments.len())).size(14.0).color(colors::TEXT_MUTED));
                ui.separator();
                
                if self.loading_comments {
                    ui.spinner();
                }
                
                for comment in &self.comments {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&comment.user.login).size(12.0).color(colors::ACCENT_DIM));
                            ui.label(RichText::new(&comment.created_at[..10]).size(10.0).color(Color32::DARK_GRAY));
                        });
                        ui.separator();
                        ui.style_mut().wrap = Some(true);
                        ui.label(&comment.body);
                    });
                    ui.add_space(5.0);
                }
                
                ui.add_space(20.0);
                
                // New comment input
                ui.label(RichText::new("添加评论:").size(12.0).color(colors::TEXT_MUTED));
                let input = TextEdit::multiline(&mut self.new_comment)
                    .desired_width(ui.available_width())
                    .desired_rows(3)
                    .hint_text("输入评论内容...");
                ui.add(input);
                
                ui.horizontal(|ui| {
                    if CyberButton::new("发表评论").min_size(Vec2::new(100.0, 30.0)).show(ui).clicked() {
                        if !self.new_comment.trim().is_empty() {
                            let _ = self.action_tx.try_send(AppAction::CreateComment(
                                self.current_repo.clone(),
                                issue.number,
                                self.new_comment.clone()
                            ));
                        }
                    }
                    
                    ui.add_space(20.0);
                    
                    // Close/Reopen button
                    let (btn_text, new_state) = if issue.state == "open" {
                        ("关闭 Issue", "closed")
                    } else {
                        ("重新打开", "open")
                    };
                    
                    if CyberButton::new(btn_text).min_size(Vec2::new(100.0, 30.0)).show(ui).clicked() {
                        let _ = self.action_tx.try_send(AppAction::UpdateIssueState(
                            self.current_repo.clone(),
                            issue.number,
                            new_state.to_string()
                        ));
                    }
                });
            });
        });
    }
}

fn parse_label_color(hex: &str) -> Color32 {
    if hex.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return Color32::from_rgb(r, g, b);
        }
    }
    Color32::GRAY
}

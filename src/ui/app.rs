use eframe::egui;
use tokio::sync::mpsc::Sender; // UI -> Backend
use std::sync::mpsc::Receiver; // Backend -> UI

use crate::context::AppContext;
use crate::modules::auth::DeviceCodeResponse;
use crate::app_event::{AppAction, AppEvent};
use crate::i18n::{I18n, Lang};
use super::sidebar::Sidebar;
use super::log_viewer::LogViewer;
use super::repo_browser::RepoBrowser;
use super::particles::{ParticleSystem, ClickRipple};

pub enum AppState {
    Login,
    RequestingCode,
    DeviceAuth {
        response: DeviceCodeResponse,
    },
    Main,
}

pub struct NativeHubApp {
    ctx: AppContext,
    state: AppState,
    
    // Internationalization
    pub i18n: I18n,
    
    // UI Components
    sidebar: Sidebar,
    log_viewer: LogViewer,
    repo_browser: RepoBrowser,
    
    // FX
    particles: ParticleSystem,
    click_ripples: Vec<ClickRipple>,
    
    // Async Bridge
    action_tx: Sender<AppAction>,
    event_rx: Receiver<AppEvent>,
    
    auth_error: Option<String>,
}

impl NativeHubApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        action_tx: Sender<AppAction>,
        event_rx: Receiver<AppEvent>,
        ctx: AppContext
    ) -> Self {
        super::configure_style(&cc.egui_ctx);
        
        Self {
            ctx,
            state: AppState::Login,
            i18n: I18n::default(), // Chinese by default
            sidebar: Sidebar::new(),
            log_viewer: LogViewer::new(),
            repo_browser: RepoBrowser::new(action_tx.clone()),
            particles: ParticleSystem::new(100), // Max 100 particles
            click_ripples: Vec::new(),
            action_tx,
            event_rx,
            auth_error: None,
        }
    }

    fn process_events(&mut self) {
        // ... same as before
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                AppEvent::Log(msg) => {
                    self.log_viewer.add_log(msg);
                }
                AppEvent::DeviceCode(res) => {
                    self.state = AppState::DeviceAuth { response: res };
                }
                AppEvent::AuthSuccess(_token) => {
                    tracing::info!("Auth success, token received");
                    self.state = AppState::Main;
                    self.auth_error = None;
                    self.log_viewer.add_log("SYSTEM: Secure Connection Established.".to_string());
                    
                    // Auto-fetch repos immediately after login
                    self.repo_browser.set_loading(true);
                    let _ = self.action_tx.try_send(AppAction::FetchRepos);
                }
                AppEvent::Error(err) => {
                    self.auth_error = Some(err.clone());
                    self.log_viewer.add_log(format!("ERROR: {}", err));
                    
                    if matches!(self.state, AppState::RequestingCode) {
                        self.state = AppState::Login;
                    }
                    self.repo_browser.set_loading(false);
                }
                AppEvent::RepoList(repos) => {
                    self.log_viewer.add_log(format!("SYSTEM: Received {} repositories.", repos.len()));
                    self.repo_browser.set_repos(repos);
                }
            }
        }
    }
}

impl eframe::App for NativeHubApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_events();
        
        // 0. Handle Click FX Input (Global)
        if ctx.input(|i| i.pointer.any_click()) {
            if let Some(pos) = ctx.pointer_interact_pos() {
                self.click_ripples.push(ClickRipple::new(pos));
            }
        }

        let screen_rect = ctx.screen_rect();
        
        // TEMPORARILY DISABLED: Custom background was blocking UI
        // TODO: Fix layer ordering issue
        // let bg_painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("global_bg")));
        // bg_painter.rect_filled(screen_rect, 0.0, egui::Color32::from_rgba_unmultiplied(5, 8, 15, 200));
        // let time = ctx.input(|i| i.time);
        // super::effects::draw_retro_grid(&bg_painter, screen_rect, time);
        let _ = screen_rect; // Suppress warning
        
        // DISABLED FOR CLARITY: Particles
        // let dt = ctx.input(|i| i.stable_dt).min(0.1);
        // self.particles.update(dt, screen_rect);
        // self.particles.draw(&bg_painter);

        // DISABLED FOR CLARITY: Click effects
        // super::particles::draw_click_effects(&bg_painter, &mut self.click_ripples, dt);

        // 3. UI Layers - These should now be visible with their default dark backgrounds
        match &self.state {
            AppState::Login => {
                 egui::CentralPanel::default().show(ctx, |ui| {
                    self.render_login(ui);
                 });
            }
            AppState::RequestingCode => {
                 egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.spinner();
                        ui.label(egui::RichText::new("ESTABLISHING UPLINK...").color(egui::Color32::from_rgb(0, 240, 255)));
                    });
                });
            }
            AppState::DeviceAuth { response } => {
                let response = response.clone();
                egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
                     self.render_device_auth(ctx, ui, &response);
                });
            }
            AppState::Main => {
                self.render_main(ctx);
            }
        }
        
        // DISABLED FOR CLARITY: CRT overlay makes text blurry
        // let overlay_painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("crt_overlay")));
        // super::effects::draw_crt_overlay(&overlay_painter, screen_rect);
        
        // Force constant repaint for animations
        ctx.request_repaint();
    }
}

impl NativeHubApp {
    fn render_login(&mut self, ui: &mut egui::Ui) {
        use super::login_view::{render_login, LoginAction};
        
        if let LoginAction::Initiate = render_login(ui, &self.auth_error, &mut self.i18n) {
            self.initiate_login();
        }
    }

    fn initiate_login(&mut self) {
        self.state = AppState::RequestingCode;
        // Non-blocking send
        let _ = self.action_tx.try_send(AppAction::Login);
    }

    fn render_device_auth(&mut self, ctx: &egui::Context, _parent_ui: &mut egui::Ui, res: &DeviceCodeResponse) {
        use super::retro_modal::RetroModal;
        
        RetroModal::show(ctx, "🔐 SECURITY CHECKPOINT", |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new("AUTHENTICATION REQUIRED").color(egui::Color32::from_rgb(255, 0, 128))); // Magenta
                ui.separator();
                ui.add_space(20.0);
                
                ui.label("1. COPY ONE-TIME CODE:");
                ui.add_space(5.0);
                
                let code_text = egui::RichText::new(&res.user_code)
                    .font(egui::FontId::monospace(32.0))
                    .color(egui::Color32::from_rgb(0, 240, 255)) // Cyan
                    .strong();
                    
                if ui.add(egui::Button::new(code_text).frame(true)).clicked() {
                     ctx.copy_text(res.user_code.clone());
                }
                ui.label(egui::RichText::new("(CLICK TO COPY)").size(10.0).color(egui::Color32::GRAY));
                
                ui.add_space(20.0);
                ui.label("2. VERIFY AT EXTERNAL TERMINAL:");
                ui.hyperlink(&res.verification_uri);
                
                ui.add_space(30.0);
                ui.horizontal_centered(|ui| {
                    ui.spinner();
                    ui.label(" Awaiting Authorization Signal...");
                });
                
                ui.add_space(30.0);
                // Cancel Button
                if ui.add(egui::Button::new("ABORT SEQUENCE").min_size(egui::Vec2::new(150.0, 30.0))).clicked() {
                     let _ = self.action_tx.try_send(AppAction::Cancel);
                     self.state = AppState::Login;
                }
            });
        });
    }

    fn render_main(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("sidebar_panel")
            .width_range(200.0..=400.0)
            .resizable(true)
            .show(ctx, |ui| {
                self.sidebar.show(ui);
            });
        
        egui::TopBottomPanel::bottom("terminal_panel")
            .min_height(150.0)
            .resizable(true)
            .show(ctx, |ui| {
                 self.log_viewer.show(ui);
            });

        // The Central Panel must be added last
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE) // Transparent to show grid
            .show(ctx, |ui| {
                 ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    // ui.heading(egui::RichText::new("COMMAND DECK ONLINE").color(egui::Color32::LIGHT_BLUE)); // Removed header
                    
                    self.repo_browser.show(ui, &self.i18n);
                });
            });
    }
}

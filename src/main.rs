// Hide console window on Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod context;
mod modules;
mod app_event;
mod backend;
mod engine;
pub mod i18n;

use eframe::egui;
use ui::NativeHubApp;
use tokio::runtime::Runtime;

// Shared initialization logic returning the app creation closure
fn make_app_creator() -> Box<dyn FnOnce(&eframe::CreationContext<'_>) -> eframe::Result<Box<dyn eframe::App>>> {
    // 1. Create Channels for Async Bridge
    let (action_tx, action_rx) = tokio::sync::mpsc::channel(100);
    let (event_tx, event_rx) = std::sync::mpsc::channel();
    
    // 2. Initialize Global Context
    let ctx = context::AppContext::new();

    // 3. Spawn Backend Logic on a separate OS thread
    let ctx_bg = ctx.clone();
    std::thread::spawn(move || {
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        tracing::info!("Backend Runtime Started");
        rt.block_on(backend::run_backend(action_rx, event_tx, ctx_bg));
    });

    // 4. Return closure
    Box::new(move |cc| Ok(Box::new(NativeHubApp::new(cc, action_tx, event_rx, ctx))))
}

#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let app_creator = make_app_creator();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("NativeHub // TERMINAL"),
        ..Default::default()
    };

    eframe::run_native(
        "NativeHub",
        options,
        app_creator,
    )
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: eframe::egui::winit::platform::android::activity::AndroidApp) {
    use eframe::egui::winit::platform::android::EventLoopBuilderExtAndroid;

    std::env::set_var("RUST_BACKTRACE", "1");
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let app_creator = make_app_creator();

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    eframe::run_native(
        "NativeHub",
        options,
        app_creator,
    ).unwrap();
}

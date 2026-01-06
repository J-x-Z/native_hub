mod ui;
mod context;
mod modules;
mod app_event;
mod backend;
mod engine;

use eframe::egui;
use ui::NativeHubApp;
use tokio::runtime::Runtime;

fn main() -> eframe::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // 1. Create Channels for Async Bridge
    // UI -> Backend (Async)
    let (action_tx, action_rx) = tokio::sync::mpsc::channel(100);
    // Backend -> UI (Sync)
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

    // 4. Configure native options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("NativeHub // TERMINAL"),
        ..Default::default()
    };

    // 5. Launch the application
    eframe::run_native(
        "NativeHub",
        options,
        Box::new(|cc| Ok(Box::new(NativeHubApp::new(cc, action_tx, event_rx, ctx)))),
    )
}

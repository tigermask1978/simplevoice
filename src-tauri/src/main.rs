// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;
mod hotkey;
mod inject;
mod transcribe;
mod tray;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;

pub struct AppState {
    pub config: Arc<Mutex<config::Config>>,
    pub recorder: Arc<Mutex<audio::Recorder>>,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let config = config::load(app.handle())?;
            let state = AppState {
                config: Arc::new(Mutex::new(config)),
                recorder: Arc::new(Mutex::new(audio::Recorder::new())),
            };
            app.manage(state);
            tray::setup(app)?;
            hotkey::register(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            config::get_config,
            config::save_config,
            audio::start_recording,
            audio::stop_recording,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

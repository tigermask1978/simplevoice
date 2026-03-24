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
use whisper_rs::WhisperContext;

pub struct AppState {
    pub config: Arc<Mutex<config::Config>>,
    pub recorder: Arc<Mutex<audio::Recorder>>,
    pub whisper_ctx: Arc<Mutex<Option<(String, WhisperContext)>>>,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let config = config::load(app.handle())?;
            let state = AppState {
                config: Arc::new(Mutex::new(config)),
                recorder: Arc::new(Mutex::new(audio::Recorder::new())),
                whisper_ctx: Arc::new(Mutex::new(None)),
            };
            app.manage(state);
            let _tray = tray::setup(app)?;
            // Keep tray alive for the app lifetime
            app.manage(_tray);
            hotkey::register(app)?;

            // Set window icon to tray.png
            if let Some(win) = app.get_webview_window("settings") {
                if let Some(icon) = crate::tray::load_icon("tray.png") {
                    win.set_icon(icon).ok();
                }
            }

            // Hide to tray on close or minimize
            if let Some(win) = app.get_webview_window("settings") {
                let win2 = win.clone();
                win.on_window_event(move |event| match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        win2.hide().ok();
                    }
                    tauri::WindowEvent::Resized(size) if size.width == 0 && size.height == 0 => {
                        win2.hide().ok();
                    }
                    _ => {}
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            config::get_config,
            config::save_config,
            audio::start_recording,
            audio::stop_recording,
            hotkey::register_hotkey,
            tray::update_tray_lang,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

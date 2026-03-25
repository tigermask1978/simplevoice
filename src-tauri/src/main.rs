// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;
mod hotkey;
mod inject;
mod transcribe;
mod tray;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;
use whisper_rs::WhisperContext;

pub struct AppState {
    pub config: Arc<Mutex<config::Config>>,
    pub recorder: Arc<Mutex<audio::Recorder>>,
    pub whisper_ctx: Arc<Mutex<Option<(String, WhisperContext)>>>,
}

fn send_notification(app: &tauri::AppHandle, body: &str) {
    app.notification()
        .builder()
        .title("SimpleVoice")
        .body(body)
        .show()
        .ok();
}

pub fn notify_here(app: &tauri::AppHandle, lang: &str, hotkey: &str) {
    let body = match lang {
        "en" => format!("I'm here. Hold {} to record, release to transcribe!", hotkey),
        "ja" => format!("ここにいます。{} を押し続けて録音、離すと文字起こし！", hotkey),
        "ko" => format!("여기 있어요. {}를 누르고 있으면 녹음, 떼면 전사 시작!", hotkey),
        _    => format!("我在这里，按住 {} 开始录音，松开开始转录！", hotkey),
    };
    send_notification(app, &body);
}

#[tauri::command]
async fn show_tray_notification(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<(), ()> {
    let cfg = state.config.lock().await;
    notify_here(&app, &cfg.language, &cfg.hotkey);
    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
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

            // Set window icon and show on startup
            if let Some(win) = app.get_webview_window("settings") {
                if let Some(icon) = crate::tray::load_icon("tray.png") {
                    win.set_icon(icon).ok();
                }
                win.show().ok();
            }

            // Hide to tray on close or minimize — ask frontend to validate first
            if let Some(win) = app.get_webview_window("settings") {
                let app2 = app.handle().clone();
                let minimizing = Arc::new(AtomicBool::new(false));
                win.on_window_event(move |event| match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        app2.emit("request-hide", ()).ok();
                    }
                    tauri::WindowEvent::Resized(size) if size.width == 0 && size.height == 0 => {
                        if !minimizing.swap(true, Ordering::SeqCst) {
                            app2.emit("request-hide", ()).ok();
                        }
                    }
                    tauri::WindowEvent::Resized(_) => {
                        minimizing.store(false, Ordering::SeqCst);
                    }
                    _ => {}
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            config::get_config,
            config::save_config,
            config::validate_model_path,
            audio::start_recording,
            audio::stop_recording,
            hotkey::register_hotkey,
            tray::update_tray_lang,
            show_tray_notification,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

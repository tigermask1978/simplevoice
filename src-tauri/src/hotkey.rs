use tauri::{App, AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

fn register_shortcut(handle: &AppHandle, hotkey: &str) -> Result<(), String> {
    let shortcut: Shortcut = hotkey.parse().map_err(|e| format!("无效热键格式: {e}"))?;
    handle.global_shortcut().unregister_all().ok();
    let h2 = handle.clone();
    handle
        .global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            let handle = h2.clone();
            tauri::async_runtime::spawn(async move {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default();
                let state = handle.state::<crate::AppState>();
                match event.state() {
                    ShortcutState::Pressed => {
                        tracing::info!("Hotkey triggered (Pressed) at {}ms", now.as_millis());
                        if let Err(e) = crate::audio::start_recording(state, handle.clone()).await {
                            tracing::error!("start_recording: {e}");
                        }
                    }
                    ShortcutState::Released => {
                        tracing::info!("Hotkey triggered (Released) at {}ms", now.as_millis());
                        if let Err(e) = crate::audio::stop_recording(state, handle.clone()).await {
                            tracing::error!("stop_recording: {e}");
                        }
                    }
                }
            });
        })
        .map_err(|e| format!("热键注册失败（可能与其他程序冲突）: {e}"))?;

    tracing::info!("Registered hotkey: {}", hotkey);
    Ok(())
}

pub fn register(app: &mut App) -> anyhow::Result<()> {
    let hotkey = app.state::<crate::AppState>().config.blocking_lock().hotkey.clone();
    register_shortcut(app.handle(), &hotkey).map_err(|e| anyhow::anyhow!("{e}"))
}

#[tauri::command]
pub async fn register_hotkey(
    hotkey: String,
    app: AppHandle,
) -> Result<(), String> {
    register_shortcut(&app, &hotkey)
}

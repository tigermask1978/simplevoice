use tauri::{App, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

pub fn register(app: &mut App) -> anyhow::Result<()> {
    let config = app.state::<crate::AppState>().config.blocking_lock().clone();
    let shortcut: Shortcut = config.hotkey.parse().map_err(|e| anyhow::anyhow!("{e}"))?;
    let handle = app.handle().clone();

    app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
        let handle = handle.clone();
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
    })?;

    tracing::info!("Registered hotkey: {}", config.hotkey);
    Ok(())
}

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub hotkey: String,
    pub model_path: String,
    pub language: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hotkey: "Ctrl+Shift+Space".into(),
            model_path: "models/ggml-small.bin".into(),
            language: "zh".into(),
        }
    }
}

fn config_path(app: &AppHandle) -> PathBuf {
    app.path().app_config_dir().unwrap().join("config.json")
}

pub fn load(app: &AppHandle) -> anyhow::Result<Config> {
    let path = config_path(app);
    if path.exists() {
        let data = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&data).unwrap_or_default())
    } else {
        Ok(Config::default())
    }
}

#[tauri::command]
pub async fn get_config(state: tauri::State<'_, crate::AppState>) -> Result<Config, String> {
    Ok(state.config.lock().await.clone())
}

#[tauri::command]
pub async fn save_config(
    config: Config,
    state: tauri::State<'_, crate::AppState>,
    app: AppHandle,
) -> Result<(), String> {
    let path = config_path(&app);
    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(&path, serde_json::to_string_pretty(&config).unwrap())
        .map_err(|e| e.to_string())?;
    *state.config.lock().await = config;
    Ok(())
}

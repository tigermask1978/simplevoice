use serde::{Deserialize, Serialize};
use std::io::Read;
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
            model_path: "".into(),
            language: "en".into(),
        }
    }
}

pub fn config_path(app: &AppHandle) -> PathBuf {
    app.path().app_config_dir().unwrap().join("config.json")
}

pub fn load(app: &AppHandle) -> anyhow::Result<Config> {
    let path = config_path(app);
    let mut config = if path.exists() {
        let data = std::fs::read_to_string(&path)?;
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Config::default()
    };

    // Resolve relative model_path against the project/resource root so it
    // works in both `tauri dev` (cwd = src-tauri/) and installed builds.
    if !config.model_path.is_empty() && !std::path::Path::new(&config.model_path).is_absolute() {
        // Walk up from cwd until we find a `models/` directory.
        let mut dir = std::env::current_dir().unwrap_or_default();
        loop {
            let candidate = dir.join(&config.model_path);
            if candidate.exists() {
                config.model_path = candidate.to_string_lossy().into_owned();
                break;
            }
            let parent = dir.join("..").canonicalize();
            match parent {
                Ok(p) if p != dir => dir = p,
                _ => break, // give up, keep original relative path
            }
        }
    }

    tracing::info!("Model path: {}", config.model_path);
    Ok(config)
}

#[tauri::command]
pub fn validate_model_path(path: String) -> bool {
    is_valid_whisper_model(&path)
}

#[tauri::command]
pub async fn get_config(state: tauri::State<'_, crate::AppState>) -> Result<Config, String> {
    Ok(state.config.lock().await.clone())
}

pub fn is_valid_whisper_model(path: &str) -> bool {
    let Ok(mut f) = std::fs::File::open(path) else { return false };
    let mut magic = [0u8; 4];
    let Ok(_) = f.read_exact(&mut magic) else { return false };
    magic == *b"GGUF" || magic == *b"lmgg"
}

#[tauri::command]
pub async fn save_config(
    config: Config,
    state: tauri::State<'_, crate::AppState>,
    app: AppHandle,
) -> Result<(), String> {
    if !is_valid_whisper_model(&config.model_path) {
        return Err("invalid_model".to_string());
    }
    let path = config_path(&app);
    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(&path, serde_json::to_string_pretty(&config).unwrap())
        .map_err(|e| e.to_string())?;
    *state.config.lock().await = config;
    Ok(())
}

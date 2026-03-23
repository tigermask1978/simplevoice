use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    App, Manager,
};

const ICONS_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn load_icon(name: &str) -> Option<tauri::image::Image<'static>> {
    let path = std::path::PathBuf::from(ICONS_DIR).join("icons").join(name);
    std::fs::read(&path)
        .ok()
        .and_then(|b| tauri::image::Image::from_bytes(&b).ok())
}

pub fn setup(app: &mut App) -> anyhow::Result<TrayIcon> {
    let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&settings_item, &quit_item])?;

    let icon = load_icon("tray.png")
        .unwrap_or_else(|| app.default_window_icon().unwrap().clone());

    let tray = TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "settings" => {
                if let Some(win) = app.get_webview_window("settings") {
                    win.show().ok();
                    win.set_focus().ok();
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(tray)
}

pub fn set_recording(app: &tauri::AppHandle, recording: bool) {
    let name = if recording { "tray-recording.png" } else { "tray.png" };
    if let Some(icon) = load_icon(name) {
        if let Some(tray) = app.tray_by_id("main") {
            tray.set_icon(Some(icon)).ok();
        }
    }
}

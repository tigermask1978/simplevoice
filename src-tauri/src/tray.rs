use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Manager,
};

pub fn setup(app: &mut App) -> anyhow::Result<()> {
    let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&settings_item, &quit_item])?;

    // In dev, resource_dir points to target/debug; use the manifest dir instead.
    let icon_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("icons/tray.png");
    tracing::info!("Tray icon path: {:?}, exists: {}", icon_path, icon_path.exists());
    let icon = std::fs::read(&icon_path)
        .ok()
        .and_then(|bytes| tauri::image::Image::from_bytes(&bytes).ok())
        .unwrap_or_else(|| app.default_window_icon().unwrap().clone());

    TrayIconBuilder::new()
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

    Ok(())
}

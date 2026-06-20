use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

/// Create the system tray icon with a context menu.
pub fn create_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

    let mut builder = TrayIconBuilder::new().tooltip("Core AI Pet").menu(&menu);

    // Try to load icon from common paths
    if let Some(icon) = load_tray_icon(app) {
        builder = builder.icon(icon);
    }

    builder.build(app)?;

    log::info!("System tray icon created");
    Ok(())
}

fn load_tray_icon(app: &AppHandle) -> Option<Image<'static>> {
    // Try resource dir (bundled app)
    if let Ok(resource_dir) = app.path().resource_dir() {
        for name in &["icons/icon.png", "icons/icon.ico", "icon.png"] {
            let path = resource_dir.join(name);
            if path.exists() {
                if let Ok(icon) = Image::from_path(&path) {
                    return Some(icon.to_owned());
                }
            }
        }
    }

    // Try current working directory (dev mode)
    for name in &["icons/icon.png", "icons/icon.ico", "src-tauri/icons/icon.png"] {
        let path = std::path::Path::new(name);
        if path.exists() {
            if let Ok(icon) = Image::from_path(path) {
                return Some(icon.to_owned());
            }
        }
    }

    log::warn!("No tray icon found, tray will have no icon");
    None
}

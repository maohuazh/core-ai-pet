use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

/// Create the system tray icon with a context menu and event handlers.
pub fn create_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

    let mut builder = TrayIconBuilder::new()
        .tooltip("Core AI Pet")
        .menu(&menu)
        .on_menu_event(move |app, event| {
            match event.id().as_ref() {
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        log::info!("Tray: show window");
                    }
                }
                "quit" => {
                    log::info!("Tray: quit application");
                    app.exit(0);
                }
                _ => {}
            }
        });

    // Load logo icon for the tray
    if let Some(icon) = load_tray_icon(app) {
        builder = builder.icon(icon);
    }

    builder.build(app)?;

    log::info!("System tray icon created with logo");
    Ok(())
}

fn load_tray_icon(app: &AppHandle) -> Option<Image<'static>> {
    // Try resource dir (bundled app) - look for logo first
    if let Ok(resource_dir) = app.path().resource_dir() {
        for name in &[
            "icons/logo.png",
            "icons/logo_256x256.png",
            "icons/logo_128x128.png",
            "icons/logo_32x32.png",
            "icons/icon.png",
            "icons/icon.ico",
        ] {
            let path = resource_dir.join(name);
            if path.exists() {
                if let Ok(icon) = Image::from_path(&path) {
                    log::info!("Loaded tray icon from: {}", path.display());
                    return Some(icon.to_owned());
                }
            }
        }
    }

    // Try current working directory (dev mode)
    for name in &[
        "icons/logo.png",
        "icons/logo_256x256.png",
        "icons/logo_128x128.png",
        "icons/logo_32x32.png",
        "icons/icon.png",
        "icons/icon.ico",
        "src-tauri/icons/logo.png",
    ] {
        let path = std::path::Path::new(name);
        if path.exists() {
            if let Ok(icon) = Image::from_path(path) {
                log::info!("Loaded tray icon from: {}", path.display());
                return Some(icon.to_owned());
            }
        }
    }

    log::warn!("No tray icon found, tray will have no icon");
    None
}

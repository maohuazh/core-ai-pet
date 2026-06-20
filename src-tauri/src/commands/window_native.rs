use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetAncestor, GetWindowLongPtrW, GetWindowRect, SetWindowLongPtrW, GA_ROOT, GWL_EXSTYLE,
    WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TRANSPARENT,
};

/// Get the HWND (*mut c_void) from a Tauri WebviewWindow
fn get_hwnd(window: &WebviewWindow) -> Result<*mut core::ffi::c_void, String> {
    let handle = window
        .window_handle()
        .map_err(|e| format!("Failed to get window handle: {}", e))?;
    match handle.as_raw() {
        RawWindowHandle::Win32(h) => Ok(h.hwnd.get() as *mut core::ffi::c_void),
        _ => Err("Not a Windows window".into()),
    }
}

/// Get the root (top-level) HWND from a Tauri WebviewWindow.
/// Tauri 2 creates a parent-child HWND hierarchy:
///   HWND_A (top-level Tauri window)
///     └── HWND_B (WebView2 control) ← raw-window-handle returns this
/// WS_EX_TRANSPARENT must be set on HWND_A to affect OS hit-testing.
fn get_root_hwnd(window: &WebviewWindow) -> Result<*mut core::ffi::c_void, String> {
    let hwnd = get_hwnd(window)?;
    unsafe {
        let root = GetAncestor(hwnd as _, GA_ROOT);
        if root.is_null() {
            return Err("Failed to get root window".into());
        }
        Ok(root as *mut core::ffi::c_void)
    }
}

/// Set or remove click-through mode (WS_EX_TRANSPARENT).
/// When enabled, the entire window passes mouse events to windows below.
#[tauri::command]
pub async fn set_click_through(window: WebviewWindow, enabled: bool) -> Result<(), String> {
    let hwnd = get_root_hwnd(&window)?;
    unsafe {
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        let new_style = if enabled {
            ex_style | (WS_EX_TRANSPARENT as isize)
        } else {
            ex_style & !(WS_EX_TRANSPARENT as isize)
        };
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style);
    }
    log::info!("Click-through: {}", enabled);
    Ok(())
}

/// Set WS_EX_NOACTIVATE so the window never steals focus from other applications.
pub fn apply_no_activate(window: &WebviewWindow) -> Result<(), String> {
    let hwnd = get_root_hwnd(window)?;
    unsafe {
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | (WS_EX_NOACTIVATE as isize));
    }
    log::info!("WS_EX_NOACTIVATE applied");
    Ok(())
}

/// Set WS_EX_LAYERED to enable per-pixel alpha transparency.
pub fn apply_layered(window: &WebviewWindow) -> Result<(), String> {
    let hwnd = get_root_hwnd(window)?;
    unsafe {
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | (WS_EX_LAYERED as isize));
    }
    log::info!("WS_EX_LAYERED applied");
    Ok(())
}

/// Start a background task that polls cursor position and toggles click-through.
/// When cursor enters window bounds → disable click-through, emit "pet-hover-start".
/// When cursor leaves window bounds → enable click-through, emit "pet-hover-end".
pub fn start_cursor_monitor(app: AppHandle) {
    std::thread::spawn(move || {
        use windows_sys::Win32::Foundation::{POINT, RECT};
        use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

        let mut is_hovering = false;

        loop {
            let window = app.get_webview_window("main");
            if let Some(window) = window {
                if let Ok(hwnd) = get_root_hwnd(&window) {
                    let mut cursor = POINT { x: 0, y: 0 };
                    let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };

                    unsafe {
                        let cursor_ok = GetCursorPos(&mut cursor);
                        let rect_ok = GetWindowRect(hwnd, &mut rect);

                        if cursor_ok != 0 && rect_ok != 0 {
                            // Use centered 160x160 hit area instead of full window rect
                            let cx = (rect.left + rect.right) / 2;
                            let cy = (rect.top + rect.bottom) / 2;
                            let half = 80; // 160 / 2
                            let in_bounds = cursor.x >= cx - half
                                && cursor.x <= cx + half
                                && cursor.y >= cy - half
                                && cursor.y <= cy + half;

                            if in_bounds && !is_hovering {
                                is_hovering = true;
                                let _ = window_set_click_through_pub(&window, false);
                                let _ = app.emit("pet-hover-start", ());
                                log::debug!("Cursor entered 160x160 hit area");
                            } else if !in_bounds && is_hovering {
                                is_hovering = false;
                                let _ = window_set_click_through_pub(&window, true);
                                let _ = app.emit("pet-hover-end", ());
                                log::debug!("Cursor left 160x160 hit area");
                            }
                        }
                    }
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });
}

/// Helper to set click-through without async (for use in setup and monitor thread)
pub fn window_set_click_through_pub(window: &WebviewWindow, enabled: bool) -> Result<(), String> {
    let hwnd = get_root_hwnd(window)?;
    unsafe {
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        let new_style = if enabled {
            ex_style | (WS_EX_TRANSPARENT as isize)
        } else {
            ex_style & !(WS_EX_TRANSPARENT as isize)
        };
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style);
    }
    Ok(())
}

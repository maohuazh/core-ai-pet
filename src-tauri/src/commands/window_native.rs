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
/// Menu visibility logic:
///   - Show menu when cursor is in model area (75x100) OR on any button (radius 80px circle, ~20px button hit area)
///   - Hide menu when cursor is outside both model area and all buttons for 400ms (debounce)
///   - The 400ms delay allows cursor to pass through the gap between model and buttons
pub fn start_cursor_monitor(app: AppHandle) {
    std::thread::spawn(move || {
        use std::time::{Duration, Instant};
        use windows_sys::Win32::Foundation::{POINT, RECT};
        use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

        let mut is_hovering = false;
        let mut leave_time: Option<Instant> = None;
        let hide_delay = Duration::from_millis(400); // 400ms debounce

        // Button positions: 6 menu buttons at radius 80px + 1 close button
        // Menu buttons: starting from top (-90°), 60° apart
        // Close button: top-right corner (top: 8px, right: 8px, size: 28x28)
        // Coordinates relative to window center (0,0), Y-axis points down
        let button_positions: [(i32, i32); 7] = [
            (0, -80),     // menu 0: top
            (69, -40),    // menu 1: top-right
            (69, 40),     // menu 2: bottom-right
            (0, 80),      // menu 3: bottom
            (-69, 40),    // menu 4: bottom-left
            (-69, -40),   // menu 5: top-left
            (78, -78),    // close button: top-right corner (178-100, 22-100)
        ];
        let button_hit_radius: i32 = 30; // pixels (increased for better detection)

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
                            let cx = (rect.left + rect.right) / 2;
                            let cy = (rect.top + rect.bottom) / 2;

                            // Check if cursor is in model area (75x100)
                            let model_half_w = 37; // 75 / 2
                            let model_half_h = 50; // 100 / 2
                            let in_model = cursor.x >= cx - model_half_w
                                && cursor.x <= cx + model_half_w
                                && cursor.y >= cy - model_half_h
                                && cursor.y <= cy + model_half_h;

                            // Check if cursor is on any button
                            let dx = cursor.x - cx;
                            let dy = cursor.y - cy;
                            let on_button = button_positions.iter().any(|&(bx, by)| {
                                let dist_sq = (dx - bx).pow(2) + (dy - by).pow(2);
                                dist_sq <= button_hit_radius.pow(2)
                            });

                            let should_show = in_model || on_button;

                            if should_show {
                                // Cursor is in interaction area
                                leave_time = None; // Reset debounce timer
                                if !is_hovering {
                                    is_hovering = true;
                                    let _ = window_set_click_through_pub(&window, false);
                                    let _ = app.emit("pet-hover-start", ());
                                    log::debug!("Cursor in model or on button (model={}, button={})", in_model, on_button);
                                }
                            } else if is_hovering {
                                // Cursor left interaction area, start or check debounce
                                if leave_time.is_none() {
                                    leave_time = Some(Instant::now());
                                    log::debug!("Cursor left interaction area, starting debounce");
                                } else if let Some(lt) = leave_time {
                                    if lt.elapsed() >= hide_delay {
                                        // Debounce period elapsed, hide menu
                                        is_hovering = false;
                                        leave_time = None;
                                        let _ = window_set_click_through_pub(&window, true);
                                        let _ = app.emit("pet-hover-end", ());
                                        log::debug!("Debounce elapsed, hiding menu");
                                    }
                                }
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

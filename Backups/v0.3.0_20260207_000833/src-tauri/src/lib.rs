//! Floating MD - Main Library
//! 
//! A floating utility bar for Windows that provides:
//! - Copy/Paste without focus stealing
//! - Clipboard history with search
//! - Markdown editing and preview
//! - Smart scrubbing of AI responses

// Modules
pub mod window;
pub mod input;
pub mod db;
pub mod ai;

use std::sync::Mutex;
use rusqlite::Connection;
use tauri::{Manager, State};
#[cfg(windows)]
use std::os::windows::process::CommandExt;

use crate::input::{ClipboardEntry, ClipboardContentType};
use crate::window::{WindowPosition, Monitor};

/// Application state containing the database connection
pub struct AppState {
    pub db: Mutex<Connection>,
}

// ============ Window Commands ============

/// Apply the noactivate window style to prevent focus stealing
#[tauri::command]
async fn apply_noactivate(window: tauri::Window) -> Result<(), String> {
    #[cfg(windows)]
    {
        let hwnd = window.hwnd().map_err(|e| e.to_string())?;
        window::apply_noactivate_style(hwnd.0 as isize)?;
        window::subclass_window_for_noactivate(hwnd.0 as isize)?;
    }
    Ok(())
}

/// Get all monitors
#[tauri::command]
fn get_monitors() -> Vec<Monitor> {
    window::get_all_monitors()
}

/// Save foreground window before action
#[tauri::command]
fn save_foreground() {
    window::save_foreground_window();
}

/// Restore foreground window after action
#[tauri::command]
fn restore_foreground() -> Result<(), String> {
    window::restore_foreground_window()
}

/// Set window position
#[tauri::command]
fn set_position(window: tauri::Window, position: WindowPosition) -> Result<(), String> {
    #[cfg(windows)]
    {
        let hwnd = window.hwnd().map_err(|e| e.to_string())?;
        window::set_window_position(hwnd.0 as isize, &position)
    }
    #[cfg(not(windows))]
    {
        let _ = (window, position);
        Err("Window positioning is only available on Windows".to_string())
    }
}

/// Resize window dynamically (for toolbar <-> editor transformation)
#[tauri::command]
async fn resize_window(window: tauri::Window, width: u32, height: u32) -> Result<(), String> {
    use tauri::LogicalSize;
    window.set_size(LogicalSize::new(width, height))
        .map_err(|e| format!("Failed to resize window: {}", e))?;
    Ok(())
}

/// Saved toolbar position and size before opening editor: (x, y, width, height)
static SAVED_TOOLBAR_POSITION: std::sync::Mutex<Option<(i32, i32, u32, u32)>> = std::sync::Mutex::new(None);

/// Set editor mode (changes window properties for editor view)
#[tauri::command]
async fn set_editor_mode(window: tauri::Window, is_editor: bool) -> Result<(), String> {
    use tauri::{LogicalSize, LogicalPosition};
    
    if is_editor {
        // Save current position AND size before opening editor
        let mut current_x = 0;
        let mut current_y = 0;
        
        if let (Ok(pos), Ok(size)) = (window.outer_position(), window.outer_size()) {
            current_x = pos.x;
            current_y = pos.y;
            let mut saved = SAVED_TOOLBAR_POSITION.lock().unwrap();
            *saved = Some((pos.x, pos.y, size.width, size.height));
            log::info!("Saved toolbar geometry: pos({}, {}) size({}x{})", pos.x, pos.y, size.width, size.height);
        }
        
        // Editor mode: larger window, resizable
        let editor_width = 800;
        let editor_height = 600;
        
        window.set_size(LogicalSize::new(editor_width as u32, editor_height as u32))
            .map_err(|e| format!("Failed to resize: {}", e))?;
        window.set_resizable(true)
            .map_err(|e| format!("Failed to set resizable: {}", e))?;
            
        // Smart Position: Ensure window is fully on screen
        if let Some(monitor) = window.current_monitor().ok().flatten() {
            let screen_size = monitor.size();
            let screen_pos = monitor.position();
            
            // Calculate relative position within monitor
            let rel_x = current_x - screen_pos.x;
            
            let mut new_x = current_x;
            let new_y = current_y;
            
            // Clamp X (Right Edge)
            if rel_x + editor_width > screen_size.width as i32 {
                new_x = screen_pos.x + (screen_size.width as i32 - editor_width);
            }
            // Clamp X (Left Edge)
            if rel_x < 0 {
                new_x = screen_pos.x;
            }

            // Apply new position if changed
            if new_x != current_x {
                 window.set_position(LogicalPosition::new(new_x, new_y))
                    .map_err(|e| format!("Failed to adjust position: {}", e))?;
            }
        }
    } else {
        // Restore saved position FIRST (before resizing)
        let saved_geometry = {
            let saved = SAVED_TOOLBAR_POSITION.lock().unwrap();
            *saved
        };

        if let Some((x, y, width, height)) = saved_geometry {
            log::info!("Restoring toolbar geometry: pos({}, {}) size({}x{})", x, y, width, height);
            
            // Restore Position
            window.set_position(LogicalPosition::new(x, y))
                .map_err(|e| format!("Failed to restore position: {}", e))?;
                
            // Restore Size
            window.set_size(LogicalSize::new(width, height))
                .map_err(|e| format!("Failed to restore size: {}", e))?;
        } else {
            // Fallback default if no saved state
            window.set_size(LogicalSize::new(80u32, 600u32))
                .map_err(|e| format!("Failed to resize: {}", e))?;
        }

        window.set_resizable(false)
            .map_err(|e| format!("Failed to set resizable: {}", e))?;
    }
    
    log::info!("Set editor mode: {}", is_editor);
    Ok(())
}
// ... (imports)

/// Check and dock window to nearest edge
#[tauri::command]
fn check_and_dock(window: tauri::Window) -> Result<String, String> {
    #[cfg(windows)]
    {
        // Get current position and size
        let current_pos = window.outer_position().map_err(|e| e.to_string())?;
        let current_size = window.outer_size().map_err(|e| e.to_string())?;
        
        // Get primary monitor (MVP: assuming primary)
        let monitor = window::get_primary_monitor().ok_or("Monitor not found")?;
        
        // Detect which edge to snap to (always returns an edge - magnetic behavior)
        let dock = window::detect_edge_snap(
            current_pos.x, 
            current_pos.y, 
            current_size.width as i32,
            current_size.height as i32,
            &monitor, 
            50 // Ignored but kept for API compatibility
        );
        
        // Get target dimensions based on dock position
        let (w, h) = window::get_docked_dimensions(dock);
        
        // Calculate exact position with sliding (keeps the free axis)
        let (new_x, new_y) = window::calculate_docked_position(
            &monitor, 
            dock, 
            current_pos.x,  // Pass current X for sliding
            current_pos.y,  // Pass current Y for sliding
            w, 
            h
        );
        
        // Apply new position and size using our low-level helper to avoid focus stealing
        let pos_struct = crate::window::WindowPosition {
            monitor_id: monitor.id,
            dock_position: dock,
            x: new_x,
            y: new_y,
            width: w,
            height: h,
        };
        
        let hwnd = window.hwnd().map_err(|e| e.to_string())?.0 as isize;
        window::set_window_position(hwnd, &pos_struct)?;
        
        // Return new orientation for frontend
        match dock {
            crate::window::DockPosition::Top | crate::window::DockPosition::Bottom => Ok("horizontal".to_string()),
            _ => Ok("vertical".to_string()),
        }
    }
    #[cfg(not(windows))]
    Ok("none".to_string())
}

/// Toggle editor mode
/// Instead of opening a separate window, this emits an event to the frontend
/// The frontend handles the transformation from toolbar to editor mode
#[tauri::command]
async fn toggle_editor(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Emitter;
    
    // Emit event to frontend to toggle editor mode
    app.emit("toggle-editor-mode", ())
        .map_err(|e| format!("Failed to emit toggle-editor-mode event: {}", e))?;
    
    log::info!("Emitted toggle-editor-mode event");
    Ok(())
}

/// Smart scrub clipboard content
#[tauri::command]
async fn smart_scrub() -> Result<String, String> {
    // 1. Get clipboard content
    let text = input::get_clipboard_text()?;
    
    // 2. Scrub it
    let clean_text = ai::scrub_text(&text);
    
    // 3. If changed, apply back
    if clean_text != text {
        input::set_clipboard_text(&clean_text)?;
        Ok("cleaned".to_string())
    } else {
        Ok("no_change".to_string())
    }
}

// ============ Clipboard Commands ============

/// Send Ctrl+C to the foreground window
#[tauri::command]
async fn send_copy() -> Result<(), String> {
    window::save_foreground_window();
    std::thread::sleep(std::time::Duration::from_millis(50));
    input::send_copy()?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    Ok(())
}

/// Send Ctrl+V to the foreground window
#[tauri::command]
async fn send_paste() -> Result<(), String> {
    window::save_foreground_window();
    std::thread::sleep(std::time::Duration::from_millis(50));
    input::send_paste()?;
    Ok(())
}

/// Send Ctrl+X to the foreground window
#[tauri::command]
async fn send_cut() -> Result<(), String> {
    window::save_foreground_window();
    std::thread::sleep(std::time::Duration::from_millis(50));
    input::send_cut()?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    Ok(())
}

/// Send Ctrl+A to the foreground window
#[tauri::command]
async fn send_select_all() -> Result<(), String> {
    window::save_foreground_window();
    std::thread::sleep(std::time::Duration::from_millis(50));
    input::send_select_all()?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    Ok(())
}

/// Send Delete to the foreground window
#[tauri::command]
async fn send_delete() -> Result<(), String> {
    window::save_foreground_window();
    std::thread::sleep(std::time::Duration::from_millis(50));
    input::send_delete()?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    Ok(())
}

/// Get text from clipboard
#[tauri::command]
fn get_clipboard() -> Result<String, String> {
    input::get_clipboard_text()
}

/// Set text to clipboard
#[tauri::command]
fn set_clipboard(text: String) -> Result<(), String> {
    input::set_clipboard_text(&text)
}

/// Type text character by character
#[tauri::command]
async fn type_text(text: String) -> Result<(), String> {
    window::save_foreground_window();
    std::thread::sleep(std::time::Duration::from_millis(50));
    input::type_string(&text)
}

// ============ Database Commands ============

/// Get recent clipboard entries
#[tauri::command]
fn get_recent_entries(
    state: State<'_, AppState>,
    limit: Option<u32>,
) -> Result<Vec<ClipboardEntry>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::get_recent_entries(&conn, limit.unwrap_or(50))
        .map_err(|e| e.to_string())
}

/// Search clipboard entries
#[tauri::command]
fn search_clipboard(
    state: State<'_, AppState>,
    query: String,
    limit: Option<u32>,
) -> Result<Vec<ClipboardEntry>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::search_entries(&conn, &query, limit.unwrap_or(50))
        .map_err(|e| e.to_string())
}

/// Add a clipboard entry
#[tauri::command]
fn add_clipboard_entry(
    state: State<'_, AppState>,
    content: String,
    content_type: Option<String>,
) -> Result<Option<i64>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    let ct = match content_type.as_deref() {
        Some("code") => ClipboardContentType::Code,
        Some("markdown") => ClipboardContentType::Markdown,
        _ => input::detect_content_type(&content),
    };
    
    let entry = ClipboardEntry::new(content).with_type(ct);
    db::insert_entry(&conn, &entry).map_err(|e| e.to_string())
}

/// Pin or unpin an entry
#[tauri::command]
fn pin_entry(
    state: State<'_, AppState>,
    id: i64,
    pinned: bool,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::set_entry_pinned(&conn, id, pinned)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Delete an entry
#[tauri::command]
fn delete_entry(
    state: State<'_, AppState>,
    id: i64,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::delete_entry(&conn, id)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Update entry content (for scrubbing)
#[tauri::command]
fn update_entry(
    state: State<'_, AppState>,
    id: i64,
    content: String,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::update_entry_content(&conn, id, &content)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Save window position to database
#[tauri::command]
fn save_position(
    state: State<'_, AppState>,
    position: WindowPosition,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::save_window_position(&conn, &position)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Load window position from database
#[tauri::command]
fn load_position(
    state: State<'_, AppState>,
) -> Result<Option<WindowPosition>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::load_window_position(&conn).map_err(|e| e.to_string())
}

// ============ Application Entry Point ============

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Kill other instances (Highlander Mode: There can be only one)
    let current_pid = std::process::id();
    #[cfg(windows)]
    {
        // We use taskkill to silently kill other processes with the same name but different PID
        // This ensures a fresh start every time the user launches the app
        let _ = std::process::Command::new("taskkill")
            .args(&["/F", "/FI", &format!("PID ne {}", current_pid), "/IM", "floating-md.exe"])
            // Suppress window creation
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .output();
    }

    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data directory");
            
            // Create directory if it doesn't exist
            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");
            
            // Initialize database
            let db_path = db::get_database_path(&app_data_dir);
            let conn = db::init_database(&db_path)
                .expect("Failed to initialize database");
            
            // Store database connection in app state
            app.manage(AppState {
                db: Mutex::new(conn),
            });
            
            log::info!("Floating MD initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Window commands
            apply_noactivate,
            get_monitors,
            save_foreground,
            restore_foreground,
            set_position,
            resize_window,
            set_editor_mode,
            // Clipboard commands
            send_copy,
            send_paste,
            send_cut,
            get_clipboard,
            set_clipboard,
            type_text,
            send_select_all,
            send_delete,
            // Database commands
            get_recent_entries,
            search_clipboard,
            add_clipboard_entry,
            pin_entry,
            delete_entry,
            update_entry,
            save_position,
            load_position,
            check_and_dock,
            toggle_editor,
            smart_scrub,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

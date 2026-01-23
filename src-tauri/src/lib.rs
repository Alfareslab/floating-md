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

use std::sync::Mutex;
use rusqlite::Connection;
use tauri::{AppHandle, Manager, State};

use crate::input::{ClipboardEntry, ClipboardContentType};
use crate::window::{WindowPosition, DockPosition, Monitor};

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
        Err("Window positioning is only available on Windows".to_string())
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
            // Clipboard commands
            send_copy,
            send_paste,
            send_cut,
            get_clipboard,
            set_clipboard,
            type_text,
            // Database commands
            get_recent_entries,
            search_clipboard,
            add_clipboard_entry,
            pin_entry,
            delete_entry,
            update_entry,
            save_position,
            load_position,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

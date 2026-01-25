//! Docking Module
//! 
//! Handles window positioning, snapping to screen edges, and multi-monitor support.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[cfg(windows)]
use windows::{
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::UI::WindowsAndMessaging::*,
};

#[cfg(windows)]
use std::ffi::c_void;

/// Represents a screen/monitor in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    pub id: String,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_primary: bool,
}

/// Represents the docking position of the window
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DockPosition {
    Left,
    Right,
    Top,
    Bottom,
    Float,
}

/// Window position configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPosition {
    pub monitor_id: String,
    pub dock_position: DockPosition,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Default for WindowPosition {
    fn default() -> Self {
        Self {
            monitor_id: "primary".to_string(),
            dock_position: DockPosition::Right,
            x: 0,
            y: 0,
            width: 80,
            height: 600,
        }
    }
}

/// Get all monitors in the system
#[cfg(windows)]
pub fn get_all_monitors() -> Vec<Monitor> {
    let mut monitors = Vec::new();
    
    unsafe {
        unsafe extern "system" fn monitor_enum_proc(
            hmonitor: HMONITOR,
            _hdc: HDC,
            _lprect: *mut RECT,
            lparam: LPARAM,
        ) -> BOOL {
            let monitors = &mut *(lparam.0 as *mut Vec<Monitor>);
            
            let mut info = MONITORINFOEXW::default();
            info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
            
            if GetMonitorInfoW(hmonitor, &mut info.monitorInfo as *mut _ as *mut MONITORINFO).as_bool() {
                let rect = info.monitorInfo.rcMonitor;
                let is_primary = (info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY) != 0;
                
                let name: String = info.szDevice
                    .iter()
                    .take_while(|&&c| c != 0)
                    .map(|&c| char::from_u32(c as u32).unwrap_or('?'))
                    .collect();
                
                monitors.push(Monitor {
                    id: format!("{:?}", hmonitor.0),
                    name,
                    x: rect.left,
                    y: rect.top,
                    width: rect.right - rect.left,
                    height: rect.bottom - rect.top,
                    is_primary,
                });
            }
            
            BOOL(1)
        }
        
        let _ = EnumDisplayMonitors(
            HDC::default(),
            None,
            Some(monitor_enum_proc),
            LPARAM(&mut monitors as *mut _ as isize),
        );
    }
    
    monitors
}

/// Get the primary monitor
#[cfg(windows)]
pub fn get_primary_monitor() -> Option<Monitor> {
    get_all_monitors().into_iter().find(|m| m.is_primary)
}

/// Get monitor by ID
#[cfg(windows)]
pub fn get_monitor_by_id(id: &str) -> Option<Monitor> {
    get_all_monitors().into_iter().find(|m| m.id == id)
}

/// Calculate docked position based on monitor and dock side
/// Supports "sliding" - keeping the free axis position while snapping the constrained axis.
/// Applies clamping to ensure window stays fully visible on screen.
pub fn calculate_docked_position(
    monitor: &Monitor,
    dock_position: DockPosition,
    current_x: i32,
    current_y: i32,
    window_width: i32,
    window_height: i32,
) -> (i32, i32) {
    let padding = 0; // No padding from edge
    
    match dock_position {
        DockPosition::Left => {
            // Snap X to left edge, keep Y (clamped to screen bounds)
            let x = monitor.x + padding;
            let y = current_y.clamp(monitor.y, monitor.y + monitor.height - window_height);
            (x, y)
        },
        DockPosition::Right => {
            // Snap X to right edge, keep Y (clamped)
            let x = monitor.x + monitor.width - window_width - padding;
            let y = current_y.clamp(monitor.y, monitor.y + monitor.height - window_height);
            (x, y)
        },
        DockPosition::Top => {
            // Snap Y to top edge, keep X (clamped)
            let x = current_x.clamp(monitor.x, monitor.x + monitor.width - window_width);
            let y = monitor.y + padding;
            (x, y)
        },
        DockPosition::Bottom => {
            // Bottom is disabled, treat as Right
            let x = monitor.x + monitor.width - window_width - padding;
            let y = current_y.clamp(monitor.y, monitor.y + monitor.height - window_height);
            (x, y)
        },
        DockPosition::Float => {
            // Float is disabled, snap to Right
            let x = monitor.x + monitor.width - window_width - padding;
            let y = current_y.clamp(monitor.y, monitor.y + monitor.height - window_height);
            (x, y)
        },
    }
}

/// Get recommended dimensions for a dock position
pub fn get_docked_dimensions(dock_position: DockPosition) -> (i32, i32) {
    match dock_position {
        DockPosition::Left | DockPosition::Right => (80, 600), // Vertical toolbar
        DockPosition::Top | DockPosition::Bottom => (600, 80), // Horizontal toolbar
        DockPosition::Float => (80, 600), // Default vertical
    }
}

/// Check if a point is near a screen edge and determine which edge to snap to
/// This function ALWAYS returns an edge (magnetic behavior) - the toolbar cannot float freely.
/// Only Left, Right, and Top are allowed (Bottom is reserved for taskbar).
pub fn detect_edge_snap(
    x: i32,
    y: i32,
    window_width: i32,
    _window_height: i32,
    monitor: &Monitor,
    _snap_distance: i32, // Ignored - we always snap
) -> DockPosition {
    // Calculate distances to each allowed edge
    // Left: distance from window's left edge to monitor's left edge
    let left_dist = (x - monitor.x).abs();
    
    // Right: distance from window's right edge to monitor's right edge
    let right_dist = ((monitor.x + monitor.width) - (x + window_width)).abs();
    
    // Top: distance from window's top edge to monitor's top edge
    let top_dist = (y - monitor.y).abs();
    
    // Find minimum distance
    let min_dist = left_dist.min(right_dist).min(top_dist);
    
    // Return the closest edge (prioritize horizontal edges over vertical for ties)
    if min_dist == top_dist {
        DockPosition::Top
    } else if min_dist == left_dist {
        DockPosition::Left
    } else {
        DockPosition::Right
    }
}

/// Move and size window to a specific position
#[cfg(windows)]
pub fn set_window_position(hwnd: isize, position: &WindowPosition) -> std::result::Result<(), String> {
    unsafe {
        let hwnd = HWND(hwnd as *mut c_void);
        
        match SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            position.x,
            position.y,
            position.width,
            position.height,
            SWP_SHOWWINDOW | SWP_NOACTIVATE,
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to set window position: {:?}", e)),
        }
    }
}

// Stub implementations for non-Windows
#[cfg(not(windows))]
pub fn get_all_monitors() -> Vec<Monitor> { vec![] }

#[cfg(not(windows))]
pub fn get_primary_monitor() -> Option<Monitor> { None }

#[cfg(not(windows))]
pub fn get_monitor_by_id(_id: &str) -> Option<Monitor> { None }

#[cfg(not(windows))]
pub fn set_window_position(_hwnd: isize, _position: &WindowPosition) -> std::result::Result<(), String> {
    Err("Window positioning is only available on Windows".to_string())
}

/// Calculate position for the editor window relative to the toolbar
pub fn calculate_editor_position(
    toolbar_pos: &WindowPosition,
    editor_width: i32,
    editor_height: i32,
) -> WindowPosition {
    let gap = 10;
    let mut x = toolbar_pos.x;
    let mut y = toolbar_pos.y;

    match toolbar_pos.dock_position {
        DockPosition::Left => {
            x = toolbar_pos.x + toolbar_pos.width + gap;
        },
        DockPosition::Right => {
            x = toolbar_pos.x - editor_width - gap;
        },
        DockPosition::Top => {
            y = toolbar_pos.y + toolbar_pos.height + gap;
        },
        DockPosition::Bottom => {
             y = toolbar_pos.y - editor_height - gap;
        },
        DockPosition::Float => {
             x = toolbar_pos.x + toolbar_pos.width + gap;
        }
    }
    
    WindowPosition {
        monitor_id: toolbar_pos.monitor_id.clone(),
        dock_position: DockPosition::Float,
        x,
        y,
        width: editor_width,
        height: editor_height
    }
}

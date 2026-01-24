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
pub fn calculate_docked_position(
    monitor: &Monitor,
    dock_position: DockPosition,
    window_width: i32,
    window_height: i32,
) -> (i32, i32) {
    match dock_position {
        DockPosition::Left => (
            monitor.x,
            monitor.y + (monitor.height - window_height) / 2,
        ),
        DockPosition::Right => (
            monitor.x + monitor.width - window_width,
            monitor.y + (monitor.height - window_height) / 2,
        ),
        DockPosition::Top => (
            monitor.x + (monitor.width - window_width) / 2,
            monitor.y,
        ),
        DockPosition::Bottom => (
            monitor.x + (monitor.width - window_width) / 2,
            monitor.y + monitor.height - window_height,
        ),
        DockPosition::Float => (
            monitor.x + (monitor.width - window_width) / 2,
            monitor.y + (monitor.height - window_height) / 2,
        ),
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

/// Check if a point is near a screen edge
pub fn detect_edge_snap(
    x: i32,
    y: i32,
    monitor: &Monitor,
    snap_distance: i32,
) -> Option<DockPosition> {
    let left_dist = (x - monitor.x).abs();
    let right_dist = (x - (monitor.x + monitor.width)).abs();
    let top_dist = (y - monitor.y).abs();
    let bottom_dist = (y - (monitor.y + monitor.height)).abs();
    
    let min_dist = left_dist.min(right_dist).min(top_dist).min(bottom_dist);
    
    if min_dist > snap_distance {
        return None;
    }
    
    if min_dist == left_dist {
        Some(DockPosition::Left)
    } else if min_dist == right_dist {
        Some(DockPosition::Right)
    } else if min_dist == top_dist {
        Some(DockPosition::Top)
    } else {
        Some(DockPosition::Bottom)
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

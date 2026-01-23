//! Docking Module
//! 
//! Handles window positioning, snapping to screen edges, and multi-monitor support.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[cfg(windows)]
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::UI::WindowsAndMessaging::*,
};

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
    Float, // Not docked to any edge
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
        // Callback function for EnumDisplayMonitors
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
                
                // Convert device name to string
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
            
            BOOL(1) // Continue enumeration
        }
        
        EnumDisplayMonitors(
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

/// Check if a point is near a screen edge (within snap distance)
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
pub fn set_window_position(hwnd: isize, position: &WindowPosition) -> Result<(), String> {
    unsafe {
        let hwnd = HWND(hwnd as *mut std::ffi::c_void);
        
        let result = SetWindowPos(
            hwnd,
            HWND_TOPMOST, // Always on top
            position.x,
            position.y,
            position.width,
            position.height,
            SWP_SHOWWINDOW | SWP_NOACTIVATE,
        );
        
        if result.as_bool() {
            Ok(())
        } else {
            Err(format!("Failed to set window position: {:?}", GetLastError()))
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
pub fn set_window_position(_hwnd: isize, _position: &WindowPosition) -> Result<(), String> {
    Err("Window positioning is only available on Windows".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_docked_position_right() {
        let monitor = Monitor {
            id: "test".to_string(),
            name: "Test".to_string(),
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
            is_primary: true,
        };
        
        let (x, y) = calculate_docked_position(&monitor, DockPosition::Right, 80, 600);
        assert_eq!(x, 1840); // 1920 - 80
        assert_eq!(y, 240);  // (1080 - 600) / 2
    }
    
    #[test]
    fn test_detect_edge_snap() {
        let monitor = Monitor {
            id: "test".to_string(),
            name: "Test".to_string(),
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
            is_primary: true,
        };
        
        // Near right edge
        let snap = detect_edge_snap(1910, 500, &monitor, 20);
        assert_eq!(snap, Some(DockPosition::Right));
        
        // Not near any edge
        let snap = detect_edge_snap(960, 540, &monitor, 20);
        assert_eq!(snap, None);
    }
}

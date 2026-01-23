//! WS_EX_NOACTIVATE Implementation
//! 
//! This module provides functionality to prevent window activation (focus stealing)
//! when the floating bar is clicked. This is crucial for a utility bar that needs
//! to send keystrokes to other applications without stealing their focus.
//!
//! # Strategy
//! 1. Primary: Use WS_EX_NOACTIVATE extended window style
//! 2. Fallback: Subclass window procedure to intercept WM_MOUSEACTIVATE
//! 3. Emergency Fallback: Save/restore foreground window

#![allow(non_snake_case)]

use std::sync::atomic::{AtomicPtr, Ordering};
use std::ffi::c_void;

#[cfg(windows)]
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

/// Store the original window procedure for subclassing
static ORIGINAL_WNDPROC: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

/// Store the last active window handle for fallback restoration
static LAST_FOREGROUND_HWND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

/// Apply WS_EX_NOACTIVATE style to a window handle
/// 
/// # Arguments
/// * `hwnd` - The window handle (as raw pointer from Tauri)
/// 
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` with error description if failed
#[cfg(windows)]
pub fn apply_noactivate_style(hwnd: isize) -> Result<(), String> {
    unsafe {
        let hwnd = HWND(hwnd as *mut c_void);
        
        // Get current extended style
        let current_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        if current_style == 0 {
            return Err(format!("Failed to get window style: {:?}", GetLastError()));
        }
        
        // Add WS_EX_NOACTIVATE and WS_EX_TOOLWINDOW (also helps with taskbar)
        let new_style = current_style 
            | WS_EX_NOACTIVATE.0 as isize 
            | WS_EX_TOOLWINDOW.0 as isize;
        
        let result = SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style);
        if result == 0 && GetLastError() != WIN32_ERROR(0) {
            return Err(format!("Failed to set window style: {:?}", GetLastError()));
        }
        
        log::info!("Applied WS_EX_NOACTIVATE to window {:?}", hwnd);
        Ok(())
    }
}

/// Subclass the window procedure to intercept WM_MOUSEACTIVATE
/// This is a fallback for systems where WS_EX_NOACTIVATE doesn't work reliably
#[cfg(windows)]
pub fn subclass_window_for_noactivate(hwnd: isize) -> Result<(), String> {
    unsafe {
        let hwnd = HWND(hwnd as *mut c_void);
        
        // Get the original window procedure
        let original_proc = GetWindowLongPtrW(hwnd, GWLP_WNDPROC);
        if original_proc == 0 {
            return Err(format!("Failed to get window procedure: {:?}", GetLastError()));
        }
        
        // Store the original procedure
        ORIGINAL_WNDPROC.store(original_proc as *mut c_void, Ordering::SeqCst);
        
        // Set our custom window procedure
        let result = SetWindowLongPtrW(
            hwnd, 
            GWLP_WNDPROC, 
            noactivate_wndproc as isize
        );
        
        if result == 0 && GetLastError() != WIN32_ERROR(0) {
            return Err(format!("Failed to subclass window: {:?}", GetLastError()));
        }
        
        log::info!("Subclassed window {:?} for WM_MOUSEACTIVATE interception", hwnd);
        Ok(())
    }
}

/// Custom window procedure that intercepts WM_MOUSEACTIVATE
#[cfg(windows)]
unsafe extern "system" fn noactivate_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_MOUSEACTIVATE => {
            // Return MA_NOACTIVATE to prevent activation
            log::trace!("Intercepted WM_MOUSEACTIVATE, returning MA_NOACTIVATE");
            return LRESULT(MA_NOACTIVATE as isize);
        }
        WM_ACTIVATE => {
            // If we're being activated, try to restore the previous window
            let activation_state = (wparam.0 & 0xFFFF) as u32;
            if activation_state != WA_INACTIVE {
                let last_hwnd = LAST_FOREGROUND_HWND.load(Ordering::SeqCst);
                if !last_hwnd.is_null() {
                    log::trace!("Restoring focus to previous window");
                    let _ = SetForegroundWindow(HWND(last_hwnd));
                }
            }
        }
        _ => {}
    }
    
    // Call the original window procedure for all other messages
    let original_proc = ORIGINAL_WNDPROC.load(Ordering::SeqCst);
    if !original_proc.is_null() {
        CallWindowProcW(
            Some(std::mem::transmute(original_proc)),
            hwnd,
            msg,
            wparam,
            lparam,
        )
    } else {
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

/// Save the current foreground window before our window might steal focus
/// Call this before any click handler
#[cfg(windows)]
pub fn save_foreground_window() {
    unsafe {
        let fg_hwnd = GetForegroundWindow();
        if fg_hwnd.0 != std::ptr::null_mut() {
            LAST_FOREGROUND_HWND.store(fg_hwnd.0, Ordering::SeqCst);
            log::trace!("Saved foreground window: {:?}", fg_hwnd);
        }
    }
}

/// Restore focus to the previously saved foreground window
#[cfg(windows)]
pub fn restore_foreground_window() -> Result<(), String> {
    unsafe {
        let last_hwnd = LAST_FOREGROUND_HWND.load(Ordering::SeqCst);
        if last_hwnd.is_null() {
            return Err("No foreground window saved".to_string());
        }
        
        let hwnd = HWND(last_hwnd);
        if SetForegroundWindow(hwnd).as_bool() {
            log::trace!("Restored foreground window: {:?}", hwnd);
            Ok(())
        } else {
            Err(format!("Failed to restore foreground window: {:?}", GetLastError()))
        }
    }
}

/// Get the current foreground window handle
#[cfg(windows)]
pub fn get_foreground_window() -> Option<isize> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            None
        } else {
            Some(hwnd.0 as isize)
        }
    }
}

// Stub implementations for non-Windows platforms
#[cfg(not(windows))]
pub fn apply_noactivate_style(_hwnd: isize) -> Result<(), String> {
    Err("WS_EX_NOACTIVATE is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn subclass_window_for_noactivate(_hwnd: isize) -> Result<(), String> {
    Err("Window subclassing is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn save_foreground_window() {}

#[cfg(not(windows))]
pub fn restore_foreground_window() -> Result<(), String> {
    Err("Foreground window management is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn get_foreground_window() -> Option<isize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[cfg(windows)]
    fn test_get_foreground_window() {
        // Should return Some value as there's always a foreground window
        let hwnd = get_foreground_window();
        assert!(hwnd.is_some());
    }
}

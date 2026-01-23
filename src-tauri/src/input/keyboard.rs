//! Keyboard Module
//! 
//! Handles sending keystrokes to other applications using the Windows SendInput API.
//! This is used to trigger Ctrl+C (copy) and Ctrl+V (paste) in the foreground window.

#![allow(non_snake_case)]

#[cfg(windows)]
use windows::{
    core::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

/// Virtual key codes for common keys
#[cfg(windows)]
pub mod vk {
    pub const VK_CONTROL: u16 = 0x11;
    pub const VK_SHIFT: u16 = 0x10;
    pub const VK_ALT: u16 = 0x12;
    pub const VK_C: u16 = 0x43;
    pub const VK_V: u16 = 0x56;
    pub const VK_X: u16 = 0x58;
    pub const VK_Z: u16 = 0x5A;
    pub const VK_A: u16 = 0x41;
    pub const VK_BACK: u16 = 0x08;
    pub const VK_DELETE: u16 = 0x2E;
    pub const VK_RETURN: u16 = 0x0D;
    pub const VK_TAB: u16 = 0x09;
    pub const VK_ESCAPE: u16 = 0x1B;
}

/// Send Ctrl+C to the foreground window (Copy)
#[cfg(windows)]
pub fn send_copy() -> Result<(), String> {
    send_key_combo(&[vk::VK_CONTROL], vk::VK_C)
}

/// Send Ctrl+V to the foreground window (Paste)
#[cfg(windows)]
pub fn send_paste() -> Result<(), String> {
    send_key_combo(&[vk::VK_CONTROL], vk::VK_V)
}

/// Send Ctrl+X to the foreground window (Cut)
#[cfg(windows)]
pub fn send_cut() -> Result<(), String> {
    send_key_combo(&[vk::VK_CONTROL], vk::VK_X)
}

/// Send Ctrl+Z to the foreground window (Undo)
#[cfg(windows)]
pub fn send_undo() -> Result<(), String> {
    send_key_combo(&[vk::VK_CONTROL], vk::VK_Z)
}

/// Send Ctrl+A to the foreground window (Select All)
#[cfg(windows)]
pub fn send_select_all() -> Result<(), String> {
    send_key_combo(&[vk::VK_CONTROL], vk::VK_A)
}

/// Send a key combination (modifiers + key)
/// 
/// # Arguments
/// * `modifiers` - Modifier keys to hold (Ctrl, Shift, Alt)
/// * `key` - The main key to press
#[cfg(windows)]
pub fn send_key_combo(modifiers: &[u16], key: u16) -> Result<(), String> {
    // Calculate number of inputs needed:
    // - Down events for each modifier
    // - Down event for key
    // - Up event for key
    // - Up events for each modifier (in reverse order)
    let num_inputs = modifiers.len() * 2 + 2;
    let mut inputs: Vec<INPUT> = Vec::with_capacity(num_inputs);
    
    // Press modifiers
    for &modifier in modifiers {
        inputs.push(create_keyboard_input(modifier, false));
    }
    
    // Press and release the main key
    inputs.push(create_keyboard_input(key, false));
    inputs.push(create_keyboard_input(key, true));
    
    // Release modifiers (in reverse order)
    for &modifier in modifiers.iter().rev() {
        inputs.push(create_keyboard_input(modifier, true));
    }
    
    // Send all inputs
    unsafe {
        let sent = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        if sent as usize != inputs.len() {
            return Err(format!(
                "SendInput failed: sent {} of {} inputs",
                sent,
                inputs.len()
            ));
        }
    }
    
    log::trace!("Sent key combo: modifiers={:?}, key=0x{:02X}", modifiers, key);
    Ok(())
}

/// Send a single key press
#[cfg(windows)]
pub fn send_key(key: u16) -> Result<(), String> {
    let inputs = [
        create_keyboard_input(key, false),
        create_keyboard_input(key, true),
    ];
    
    unsafe {
        let sent = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        if sent != 2 {
            return Err(format!("SendInput failed: sent {} of 2 inputs", sent));
        }
    }
    
    log::trace!("Sent key: 0x{:02X}", key);
    Ok(())
}

/// Type a string character by character
/// Note: This is slower but works for any Unicode character
#[cfg(windows)]
pub fn type_string(text: &str) -> Result<(), String> {
    for ch in text.encode_utf16() {
        let inputs = [
            create_unicode_input(ch, false),
            create_unicode_input(ch, true),
        ];
        
        unsafe {
            let sent = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
            if sent != 2 {
                return Err(format!("SendInput failed at char {}", ch));
            }
        }
    }
    
    log::trace!("Typed string: {} chars", text.len());
    Ok(())
}

/// Create a keyboard INPUT structure for a virtual key
#[cfg(windows)]
fn create_keyboard_input(vk: u16, key_up: bool) -> INPUT {
    let mut flags = KEYBD_EVENT_FLAGS(0);
    if key_up {
        flags |= KEYEVENTF_KEYUP;
    }
    
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk),
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// Create a keyboard INPUT structure for a Unicode character
#[cfg(windows)]
fn create_unicode_input(ch: u16, key_up: bool) -> INPUT {
    let mut flags = KEYEVENTF_UNICODE;
    if key_up {
        flags |= KEYEVENTF_KEYUP;
    }
    
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: ch,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

// Stub implementations for non-Windows
#[cfg(not(windows))]
pub mod vk {
    pub const VK_CONTROL: u16 = 0x11;
    pub const VK_SHIFT: u16 = 0x10;
    pub const VK_ALT: u16 = 0x12;
    pub const VK_C: u16 = 0x43;
    pub const VK_V: u16 = 0x56;
    pub const VK_X: u16 = 0x58;
    pub const VK_Z: u16 = 0x5A;
    pub const VK_A: u16 = 0x41;
    pub const VK_BACK: u16 = 0x08;
    pub const VK_DELETE: u16 = 0x2E;
    pub const VK_RETURN: u16 = 0x0D;
    pub const VK_TAB: u16 = 0x09;
    pub const VK_ESCAPE: u16 = 0x1B;
}

#[cfg(not(windows))]
pub fn send_copy() -> Result<(), String> {
    Err("Keyboard injection is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn send_paste() -> Result<(), String> {
    Err("Keyboard injection is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn send_cut() -> Result<(), String> {
    Err("Keyboard injection is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn send_undo() -> Result<(), String> {
    Err("Keyboard injection is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn send_select_all() -> Result<(), String> {
    Err("Keyboard injection is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn send_key_combo(_modifiers: &[u16], _key: u16) -> Result<(), String> {
    Err("Keyboard injection is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn send_key(_key: u16) -> Result<(), String> {
    Err("Keyboard injection is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn type_string(_text: &str) -> Result<(), String> {
    Err("Keyboard injection is only available on Windows".to_string())
}

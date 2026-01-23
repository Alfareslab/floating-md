//! Input module - Clipboard monitoring and keyboard injection
//! 
//! This module handles:
//! - Monitoring clipboard changes
//! - Sending Ctrl+C/V keystrokes to other applications
//! - Reading clipboard content

mod clipboard;
mod keyboard;

pub use clipboard::*;
pub use keyboard::*;

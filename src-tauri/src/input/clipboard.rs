//! Clipboard Module
//! 
//! Handles clipboard operations including:
//! - Reading text from clipboard
//! - Writing text to clipboard
//! - Monitoring clipboard changes
//! - Clipboard history management (via database)

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[cfg(windows)]
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::DataExchange::*,
    Win32::System::Memory::*,
    Win32::UI::WindowsAndMessaging::*,
};

#[cfg(windows)]
use std::ffi::c_void;

/// Represents a clipboard entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    pub id: Option<i64>,
    pub content: String,
    pub content_type: ClipboardContentType,
    pub created_at: i64,
    pub is_pinned: bool,
    pub is_scrubbed: bool,
}

/// Type of clipboard content
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ClipboardContentType {
    Text,
    Code,
    Markdown,
}

impl Default for ClipboardContentType {
    fn default() -> Self {
        Self::Text
    }
}

impl ClipboardEntry {
    pub fn new(content: String) -> Self {
        Self {
            id: None,
            content,
            content_type: ClipboardContentType::Text,
            created_at: chrono::Utc::now().timestamp(),
            is_pinned: false,
            is_scrubbed: false,
        }
    }
    
    pub fn with_type(mut self, content_type: ClipboardContentType) -> Self {
        self.content_type = content_type;
        self
    }
}

/// Read text content from the clipboard
#[cfg(windows)]
pub fn get_clipboard_text() -> Result<String, String> {
    unsafe {
        // Open clipboard
        if !OpenClipboard(HWND::default()).as_bool() {
            return Err(format!("Failed to open clipboard: {:?}", GetLastError()));
        }
        
        // Ensure we close clipboard on exit
        struct ClipboardGuard;
        impl Drop for ClipboardGuard {
            fn drop(&mut self) {
                unsafe { let _ = CloseClipboard(); }
            }
        }
        let _guard = ClipboardGuard;
        
        // Get clipboard data as Unicode text
        let handle = GetClipboardData(CF_UNICODETEXT.0 as u32);
        if handle.is_err() {
            return Err("No text content in clipboard".to_string());
        }
        
        let handle = handle.unwrap();
        let ptr = GlobalLock(handle.0 as *mut c_void);
        if ptr.is_null() {
            return Err(format!("Failed to lock clipboard data: {:?}", GetLastError()));
        }
        
        struct GlobalLockGuard(HGLOBAL);
        impl Drop for GlobalLockGuard {
            fn drop(&mut self) {
                unsafe { let _ = GlobalUnlock(self.0); }
            }
        }
        let _lock_guard = GlobalLockGuard(HGLOBAL(handle.0 as *mut c_void));
        
        // Convert wide string to Rust String
        let wide_ptr = ptr as *const u16;
        let mut len = 0;
        while *wide_ptr.add(len) != 0 {
            len += 1;
        }
        
        let wide_slice = std::slice::from_raw_parts(wide_ptr, len);
        String::from_utf16(wide_slice)
            .map_err(|e| format!("Failed to convert clipboard text: {}", e))
    }
}

/// Write text content to the clipboard
#[cfg(windows)]
pub fn set_clipboard_text(text: &str) -> Result<(), String> {
    unsafe {
        // Convert to wide string
        let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let size = wide.len() * 2;
        
        // Allocate global memory
        let hmem = GlobalAlloc(GMEM_MOVEABLE, size);
        if hmem.is_err() {
            return Err(format!("Failed to allocate memory: {:?}", GetLastError()));
        }
        let hmem = hmem.unwrap();
        
        // Lock and copy data
        let ptr = GlobalLock(hmem.0);
        if ptr.is_null() {
            let _ = GlobalFree(hmem);
            return Err(format!("Failed to lock memory: {:?}", GetLastError()));
        }
        
        std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr as *mut u16, wide.len());
        let _ = GlobalUnlock(hmem);
        
        // Open clipboard
        if !OpenClipboard(HWND::default()).as_bool() {
            let _ = GlobalFree(hmem);
            return Err(format!("Failed to open clipboard: {:?}", GetLastError()));
        }
        
        // Ensure we close clipboard on exit
        struct ClipboardGuard;
        impl Drop for ClipboardGuard {
            fn drop(&mut self) {
                unsafe { let _ = CloseClipboard(); }
            }
        }
        let _guard = ClipboardGuard;
        
        // Empty clipboard and set new data
        if !EmptyClipboard().as_bool() {
            let _ = GlobalFree(hmem);
            return Err(format!("Failed to empty clipboard: {:?}", GetLastError()));
        }
        
        let result = SetClipboardData(CF_UNICODETEXT.0 as u32, HANDLE(hmem.0));
        if result.is_err() {
            let _ = GlobalFree(hmem);
            return Err(format!("Failed to set clipboard data: {:?}", GetLastError()));
        }
        
        log::info!("Set clipboard text ({} chars)", text.len());
        Ok(())
    }
}

/// Detect content type based on content analysis
pub fn detect_content_type(content: &str) -> ClipboardContentType {
    // Check for code patterns
    let code_patterns = [
        "fn ", "pub ", "let ", "const ", "impl ", // Rust
        "function ", "const ", "var ", "=>", "===", // JavaScript
        "def ", "class ", "import ", "from ", // Python
        "public ", "private ", "protected ", "void ", // Java/C#/C++
        "func ", "var ", "let ", "guard ", // Swift
        "#include", "#define", "int main", // C/C++
    ];
    
    let has_code_patterns = code_patterns.iter().any(|p| content.contains(p));
    let has_braces = content.contains('{') && content.contains('}');
    let has_semicolons = content.matches(';').count() > 2;
    
    if has_code_patterns || (has_braces && has_semicolons) {
        return ClipboardContentType::Code;
    }
    
    // Check for markdown patterns
    let md_patterns = [
        "# ", "## ", "### ", // Headers
        "```", "---", "***", // Code blocks, dividers
        "- ", "* ", "1. ", // Lists
        "[", "](", // Links
        "**", "__", // Bold
        "*", "_", // Italic
    ];
    
    let md_count = md_patterns.iter().filter(|p| content.contains(*p)).count();
    if md_count >= 2 {
        return ClipboardContentType::Markdown;
    }
    
    ClipboardContentType::Text
}

// Stub implementations for non-Windows
#[cfg(not(windows))]
pub fn get_clipboard_text() -> Result<String, String> {
    Err("Clipboard access is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn set_clipboard_text(_text: &str) -> Result<(), String> {
    Err("Clipboard access is only available on Windows".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_content_type_code() {
        let rust_code = "fn main() {\n    println!(\"Hello\");\n}";
        assert_eq!(detect_content_type(rust_code), ClipboardContentType::Code);
    }
    
    #[test]
    fn test_detect_content_type_markdown() {
        let markdown = "# Title\n\nSome **bold** text and a [link](url)";
        assert_eq!(detect_content_type(markdown), ClipboardContentType::Markdown);
    }
    
    #[test]
    fn test_detect_content_type_text() {
        let text = "This is just plain text without any special formatting.";
        assert_eq!(detect_content_type(text), ClipboardContentType::Text);
    }
}

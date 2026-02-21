# 🚶 Floating MD: Audit & Fixes Walkthrough (v0.3.0)

> **Date:** 2026-02-06  
> **Phase:** Audit & Stabilization Complete  
> **Status:** Stable Core

---

## 🎯 Overview
This walkthrough documents the journey of stabilizing "Floating MD" from a non-functional state to a stable, usable application with core features active.

## 🛠️ Critical Fixes (P1) - The "Unusable" to "Stable" Journey

### 1. The "Dead Buttons" Issue 💀
- **Symptom:** No toolbar buttons (Copy, Paste, etc.) were responding to clicks.
- **Root Cause:** A Rust-side stack overflow in the Focus Guardian logic was interacting badly with the frontend events.
- **Fix:** Implemented a re-entrancy guard in `noactivate.rs`.

### 2. The Focus Thief 🦹
- **Symptom:** Clicking the app stole focus from other apps (e.g., Notepad), breaking the core "Floating" promise.
- **Root Cause:** Standard window behavior.
- **Fix:** Implementation of `WS_EX_NOACTIVATE` style and `WM_MOUSEACTIVATE` interception in Rust (Focus Guardian).

### 3. The Stationary Window ⚓
- **Symptom:** The window was glued to the screen, dragging didn't work.
- **Root Cause:** The `data-tauri-drag-region` div was commented out during debugging.
- **Fix:** Re-enabled the drag region with `z-index: -10` to sit behind buttons.

### 4. The Uncloseable App 🧟
- **Symptom:** Quit button did nothing.
- **Root Cause:** Missing `core:window:allow-close` permission in Tauri v2 capabilities.
- **Fix:** Added the permission to `default.json`.

---

## ⚡ Feature Activation (P2)

### 1. Markdown Preview
- **Before:** Showed raw markdown text.
- **Now:** Renders beautiful HTML using `unified` + `remark` + `rehype`.

### 2. Clipboard Integration
- **Load from Clipboard:** One-click import.
- **Copy HTML:** One-click export with toast feedback.

---

## 📸 State of the App

| Feature | Status | Notes |
|---------|--------|-------|
| **Focus Guardian** | ✅ Active | Tested with Notepad |
| **Toolbar** | ✅ Active | All buttons responsive |
| **Window** | ✅ Active | Draggable & Dockable |
| **Markdown** | ✅ Active | Edit/Preview modes working |
| **Stability** | ✅ Stable | No more crashes/stack overflows |

---

## 🔮 What's Next? (Roadmap)

We are now ready to move to **v0.4.0 Polish Phase**:
1. **Clipboard History:** Fix popup positioning.
2. **Smoothness:** Improve dragging physics.
3. **Advanced Editor:** Enhance Markdown capabilities & AI Button.

# 🏗️ Floating MD - Architecture Reference

> **Tech Stack:** Tauri 2.0 (Rust) + React 18 (TypeScript) + SQLite

This document outlines the high-level architecture of Floating MD, designed for performance, low memory usage, and unobtrusive UX.

## 🔄 Data Flow

```mermaid
graph TD
    User[User / Global Input] -->|Ctrl+C / Ctrl+V| OS_Clipboard
    OS_Clipboard -->|Polling/Hooks| Rust_Backend[Rust Backend (Tauri)]
    
    subgraph Rust Core
        Rust_Backend -->|Sqlite| DB[(Clipboard History DB)]
        Rust_Backend -->|Regex| AI_Scrubber[AI Smart Scrubber]
        Rust_Backend -->|WinAPI| Window_Manager[Window Positioning & Focus]
    end
    
    subgraph Frontend (React)
        Rust_Backend -->|IPC Events| React_UI[React UI (Toolbar)]
        React_UI -->|Invoke Commands| Rust_Backend
        React_UI -->|State| Editor[Markdown Editor]
    end
```

## 🧩 Key Components

### 1. The Floating Toolbar (`src-tauri/src/lib.rs`, `Toolbar.tsx`)
- **Focus Guardian:** Uses `WS_EX_NOACTIVATE` (Windows API) to ensure clicking buttons *never* steals focus from the user's active application (e.g., VS Code, Word).
- **Magnetic Docking:** 
  - Logic in `src-tauri/src/window/docking.rs`.
  - Enforces snapping to Screen Left, Right, or Top.
  - Supports "Sliding" along the snapped edge.

### 2. Clipboard Manager (`src-tauri/src/input/`)
- Monitors system clipboard.
- Stores history in SQLite (`clipboard_history.db`).
- **Input Simulation:** Uses `enigo` & `rdev` to simulate `Ctrl+V` keystrokes for pasting into external apps.

### 3. Smart Scrub (`src-tauri/src/ai/`)
- A regex-based engine to detect and remove LLM conversational fluff ("Here is the code", "Sure!", etc).
- Works offline, zero-latency.

### 4. Markdown Editor
- Hidden `WebviewWindow` that slides out on command.
- Built with `react-markdown` and `remark-gfm`.
- Supports RTL (Arabic) natively via CSS Grid/Flexbox tweaks.

## 📂 Project Structure

```
floating-md/
├── src/                  # React Frontend
│   ├── components/       # Toolbar, Editor, ClipboardHistory
│   ├── hooks/            # useDocking, useAutoHide
│   └── stores/           # Zustand state (if needed)
├── src-tauri/            # Rust Backend
│   ├── src/
│   │   ├── ai/           # scrubbing logic
│   │   ├── db/           # sqlite schemas & queries
│   │   ├── input/        # keyboard/clipboard handling
│   │   ├── window/       # docking & monitor math
│   │   └── lib.rs        # Main entry point & commands
│   └── tauri.conf.json   # Process configuration
└── plans/                # Development history
```

## 🚀 Build Process

1. **Frontend Build:** Vite compiles React to static HTML/JS/CSS (`dist/`).
2. **Backend Build:** Cargo compiles Rust code + embeds `dist/` into the binary.
3. **Bundling:** Tauri bundles the `.exe` and WebView2 runtime into an `.msi` installer.

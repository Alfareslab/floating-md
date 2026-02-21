# 🐛 Known Bugs & Final Polish (Before Release v1.0)

## ✅ Critical Issues (Fixed)
- [x] **Multiple Instances**: The app allows multiple copies to run simultaneously. (Fixed: Implemented 'Highlander Mode' in `lib.rs` startup to kill other instances).
- [x] **Dragging Stability**: Auto-docking logic sometimes fights with user dragging. (Fixed: Implemented Magnetic Snapping in `docking.rs` + CSS drag regions).
- [x] **Editor Window Visibility**: Editor window was opening as a blank white window at startup (Fixed via `visible: false` and Label Routing).

## 🛠️ Minor Issues / Polish
- [ ] **Drag Handle**: The separate drag handle was too small (Fixed: Made entire toolbar draggable).
- [x] **Startup Position**: App could start off-screen if last position was invalid (Fixed: Hardcoded safe start at 100,100).
- [ ] **Window Focus**: Ensure `check_and_dock` doesn't steal focus while typing in another app.

## 📝 Action Plan
1. Implement **Single Instance Lock** in Rust main.rs.
2. Refine **Docking Logic** to be smoother or manual-only.
3. Perform full clean & rebuild.

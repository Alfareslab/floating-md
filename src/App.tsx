import { useState } from 'react';
import { Toolbar } from './components/Toolbar/Toolbar';
import { MarkdownEditor } from './components/Editor/MarkdownEditor';
import { useDocking } from './hooks/useDocking';
import { invoke } from '@tauri-apps/api/core';

/**
 * Main App Component
 * 
 * The app is a SINGLE WINDOW that transforms between:
 * - Toolbar Mode: Slim floating bar with action buttons
 * - Editor Mode: Full markdown editor with toolbar as header
 * 
 * See docs/PRODUCT_VISION.md for full specification.
 */
function App() {
  const [orientation, setOrientation] = useState<'horizontal' | 'vertical'>('vertical');
  const [dockSide, setDockSide] = useState<'top' | 'bottom' | 'left' | 'right'>('left');
  const [mode, setMode] = useState<'toolbar' | 'editor'>('toolbar');

  // Store previous state for restoration when closing editor
  const [previousState, setPreviousState] = useState<{
    orientation: 'horizontal' | 'vertical';
    dockSide: 'top' | 'bottom' | 'left' | 'right';
  } | null>(null);

  // ... (Focus Guardian useEffect remains same)

  // Enable auto-docking logic (only in toolbar mode)
  useDocking((newOrientation, newSide) => {
    setOrientation(newOrientation);
    setDockSide(newSide);
  }, mode === 'toolbar');

  // ... (Toggle logic)

  // Toggle function for internal use
  const handleToggleEditor = async () => {
    if (mode === 'toolbar') {
      setPreviousState({ orientation, dockSide });
      setOrientation('horizontal');
      setMode('editor');
      // Resize window to editor size
      await invoke('set_editor_mode', { isEditor: true }).catch(console.error);
    } else {
      if (previousState) {
        setOrientation(previousState.orientation);
        setDockSide(previousState.dockSide);
      }
      setMode('toolbar');
      // Resize window back to toolbar size
      await invoke('set_editor_mode', { isEditor: false }).catch(console.error);
    }
  };

  // ========== TOOLBAR MODE ==========
  if (mode === 'toolbar') {
    return (
      <div className="w-full h-full bg-transparent overflow-hidden">
        <Toolbar
          orientation={orientation}
          dockSide={dockSide}
          onToggleEditor={handleToggleEditor}
        />
      </div>
    );
  }

  // ========== EDITOR MODE ==========
  return (
    <div className="w-screen h-screen flex flex-col overflow-hidden bg-base rounded-3xl border border-white/5 shadow-2xl">
      {/* Toolbar as Header */}
      <Toolbar
        orientation="horizontal"
        dockSide="top" // Editor is always top-oriented
        onToggleEditor={handleToggleEditor}
        mode="editor"
      />

      {/* Editor Content */}
      <div className="flex-1 overflow-hidden bg-base/50">
        <MarkdownEditor onClose={handleToggleEditor} />
      </div>
    </div>
  );
}

export default App;

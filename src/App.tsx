import { useState } from 'react';
import { Toolbar } from './components/Toolbar/Toolbar';
import { MarkdownEditor } from './components/Editor/MarkdownEditor';
import { useDocking } from './hooks/useDocking';

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
  const [mode, setMode] = useState<'toolbar' | 'editor'>('toolbar');

  // Store previous state for restoration when closing editor
  const [previousState, setPreviousState] = useState<{
    orientation: 'horizontal' | 'vertical';
  } | null>(null);

  // Enable auto-docking logic (only in toolbar mode)
  // When in editor mode, docking is disabled to prevent resizing
  useDocking(setOrientation, mode === 'toolbar');

  // NOTE: We removed the toggle-editor-mode event listener because
  // we now call onToggleEditor directly from Toolbar.tsx
  // This prevents the double-toggle issue.

  // Toggle function for internal use
  const handleToggleEditor = async () => {
    const { invoke } = await import('@tauri-apps/api/core');

    if (mode === 'toolbar') {
      setPreviousState({ orientation });
      setOrientation('horizontal');
      setMode('editor');
      // Resize window to editor size
      await invoke('set_editor_mode', { isEditor: true }).catch(console.error);
    } else {
      if (previousState) {
        setOrientation(previousState.orientation);
      }
      setMode('toolbar');
      // Resize window back to toolbar size
      await invoke('set_editor_mode', { isEditor: false }).catch(console.error);
    }
  };

  // ========== TOOLBAR MODE ==========
  console.log("Current mode:", mode);
  if (mode === 'toolbar') {
    console.log("Rendering TOOLBAR MODE");
    return (
      <div className="w-screen h-screen flex items-center justify-center bg-transparent overflow-hidden">
        <Toolbar
          orientation={orientation}
          onToggleEditor={handleToggleEditor}
        />
      </div>
    );
  }

  // ========== EDITOR MODE ==========
  // ========== EDITOR MODE ==========
  // The toolbar becomes the header of the editor panel
  console.log("Rendering EDITOR MODE");
  return (
    // Added rounded-3xl to mimic modern OS windows (smooth corners)
    // border-white/5 for subtle definition
    <div className="w-screen h-screen flex flex-col overflow-hidden bg-base rounded-3xl border border-white/5 shadow-2xl">
      {/* Toolbar as Header */}
      <Toolbar
        orientation="horizontal"
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

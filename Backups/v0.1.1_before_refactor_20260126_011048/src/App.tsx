import React, { useEffect, useState } from 'react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Toolbar } from './components/Toolbar/Toolbar';
import { MarkdownEditor } from './components/Editor/MarkdownEditor';
import { useDocking } from './hooks/useDocking';

function App() {
  const [windowLabel, setWindowLabel] = useState<string>('');
  const [orientation, setOrientation] = useState<'horizontal' | 'vertical'>('vertical');

  // Enable auto-docking logic (only relevant for main toolbar)
  useDocking(setOrientation);

  useEffect(() => {
    // Determine which window we are running in
    const label = getCurrentWindow().label;
    console.log("Current Window Label:", label);
    setWindowLabel(label);
  }, []);

  // Loading state
  if (!windowLabel) return null;

  // Editor Window
  if (windowLabel === 'editor') {
    return (
      <div className="w-screen h-screen overflow-hidden bg-base">
        <MarkdownEditor onClose={() => getCurrentWindow().hide()} />
      </div>
    );
  }

  // Main Toolbar Window (default)
  return (
    <div className="w-screen h-screen flex items-center justify-center bg-transparent overflow-hidden">
      <Toolbar orientation={orientation} />
    </div>
  );
}

export default App;

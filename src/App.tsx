import React, { useEffect, useState } from 'react';
import { Toolbar } from './components/Toolbar/Toolbar';
import { MarkdownEditor } from './components/Editor/MarkdownEditor';
import { useDocking } from './hooks/useDocking';

function App() {
  const [path, setPath] = useState(window.location.pathname);
  const [orientation, setOrientation] = useState<'horizontal' | 'vertical'>('vertical');

  // Enable auto-docking logic
  useDocking(setOrientation);

  useEffect(() => {
    console.log("Current path:", window.location.pathname);
    setPath(window.location.pathname);
  }, []);

  // Editor Window
  if (path === '/editor') {
    return (
      <div className="w-screen h-screen overflow-hidden bg-base">
        <MarkdownEditor />
      </div>
    );
  }

  // Main Toolbar Window
  return (
    <div className="w-screen h-screen flex items-center justify-center bg-transparent overflow-hidden">
      <Toolbar orientation={orientation} />
    </div>
  );
}

export default App;

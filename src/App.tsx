import React from 'react';
import { Toolbar } from './components/Toolbar/Toolbar';
import { useClipboardStore } from './stores/clipboardStore';

function App() {
  const [orientation, setOrientation] = React.useState<'horizontal' | 'vertical'>('vertical');

  return (
    <div className="w-screen h-screen flex items-center justify-center bg-transparent">
      <Toolbar orientation={orientation} />

      {/* Debug Toggle (will be removed in production) */}
      <button
        onClick={() => setOrientation(o => o === 'horizontal' ? 'vertical' : 'horizontal')}
        className="fixed bottom-4 right-4 px-4 py-2 bg-accent rounded-lg text-sm"
      >
        Toggle: {orientation}
      </button>
    </div>
  );
}

export default App;

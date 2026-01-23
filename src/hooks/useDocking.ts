import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * Hook to handle window docking and snap-to-edge logic
 */
export const useDocking = (
    onOrientationChange: (orientation: 'horizontal' | 'vertical') => void
) => {
    useEffect(() => {
        // Poll for docking status every 500ms
        // This is a simple way to detect if the user moved the window near an edge
        const intervalId = setInterval(async () => {
            try {
                // 'check_and_dock' will:
                // 1. Check if window is near an edge
                // 2. If yes, snap it and resize it
                // 3. Return the new orientation ('horizontal' or 'vertical')
                // 4. If no, return 'none'
                const orientation = await invoke<string>('check_and_dock');

                if (orientation !== 'none') {
                    onOrientationChange(orientation as 'horizontal' | 'vertical');
                }
            } catch (error) {
                console.error("Docking check failed:", error);
            }
        }, 500);

        return () => clearInterval(intervalId);
    }, [onOrientationChange]);
};

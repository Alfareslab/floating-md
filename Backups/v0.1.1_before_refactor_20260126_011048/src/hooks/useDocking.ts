import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * Hook to handle window docking and snap-to-edge logic
 * Polls the backend every 1500ms to check if window should snap to an edge.
 * The backend now uses "magnetic" behavior - always snapping to nearest edge.
 */
export const useDocking = (
    onOrientationChange: (orientation: 'horizontal' | 'vertical') => void
) => {
    useEffect(() => {
        // Poll for docking status every 1500ms
        // Gives user time to drag before snapping kicks in
        const intervalId = setInterval(async () => {
            try {
                const orientation = await invoke<string>('check_and_dock');
                // Backend always returns an orientation now (no 'none')
                onOrientationChange(orientation as 'horizontal' | 'vertical');
            } catch (error) {
                console.error("Docking check failed:", error);
            }
        }, 1500);

        return () => clearInterval(intervalId);
    }, [onOrientationChange]);
};

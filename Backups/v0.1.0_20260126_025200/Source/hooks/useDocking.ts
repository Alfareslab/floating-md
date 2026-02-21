import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * Hook to handle window docking and snap-to-edge logic
 * Polls the backend every 1500ms to check if window should snap to an edge.
 * The backend now uses "magnetic" behavior - always snapping to nearest edge.
 * 
 * @param onOrientationChange - Callback when orientation changes
 * @param enabled - Whether docking should be active (false in editor mode)
 */
export const useDocking = (
    onOrientationChange: (orientation: 'horizontal' | 'vertical') => void,
    enabled: boolean = true
) => {
    useEffect(() => {
        // Don't run docking logic if disabled (e.g., in editor mode)
        if (!enabled) {
            console.log("useDocking: DISABLED (editor mode)");
            return;
        }

        console.log("useDocking: ENABLED (toolbar mode)");

        // Poll for docking status every 1500ms
        const intervalId = setInterval(async () => {
            try {
                const orientation = await invoke<string>('check_and_dock');
                onOrientationChange(orientation as 'horizontal' | 'vertical');
            } catch (error) {
                console.error("Docking check failed:", error);
            }
        }, 1500);

        return () => clearInterval(intervalId);
    }, [onOrientationChange, enabled]);
};

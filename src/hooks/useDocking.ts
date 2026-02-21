import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

export type DockSide = 'top' | 'bottom' | 'left' | 'right';

export const useDocking = (
    onDockChange: (orientation: 'horizontal' | 'vertical', side: DockSide) => void,
    enabled: boolean = true
) => {
    useEffect(() => {
        if (!enabled) return;

        // Listen for snap updates from backend (triggered after window move)
        const unlistenPromise = listen<string>('snap-update', (event) => {
            const side = event.payload as DockSide;
            const orientation = (side === 'top' || side === 'bottom') ? 'horizontal' : 'vertical';
            onDockChange(orientation, side);
        });

        return () => {
            unlistenPromise.then(unlisten => unlisten());
        };
    }, [onDockChange, enabled]);
};

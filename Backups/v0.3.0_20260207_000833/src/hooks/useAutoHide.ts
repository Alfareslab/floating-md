import { useState, useEffect, useCallback, useRef } from 'react';

export const useAutoHide = (enabled: boolean = true, delay: number = 3000) => {
    const [isVisible, setIsVisible] = useState(true);
    const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    const show = useCallback(() => {
        setIsVisible(true);
        if (timerRef.current) {
            clearTimeout(timerRef.current);
            timerRef.current = null;
        }
    }, []);

    const startHideTimer = useCallback(() => {
        if (enabled) {
            if (timerRef.current) clearTimeout(timerRef.current);
            timerRef.current = setTimeout(() => {
                setIsVisible(false);
            }, delay);
        }
    }, [enabled, delay]);

    // Initial timer
    useEffect(() => {
        if (enabled) {
            startHideTimer();
        }
        return () => {
            if (timerRef.current) clearTimeout(timerRef.current);
        };
    }, [enabled, startHideTimer]);

    return { isVisible, show, startHideTimer };
};

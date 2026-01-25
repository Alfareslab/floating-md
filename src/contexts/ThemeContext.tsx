import React, { createContext, useContext, useEffect, useState } from 'react';

export type Theme = 'dark' | 'light' | 'warm';

interface ThemeContextType {
    theme: Theme;
    setTheme: (theme: Theme) => void;
    nextTheme: () => void; // Cycle through themes
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export const useTheme = () => {
    const context = useContext(ThemeContext);
    if (!context) {
        throw new Error('useTheme must be used within a ThemeProvider');
    }
    return context;
};

interface ThemeColors {
    background: string;
    foreground: string;
    toolbarBg: string; // For the glass toolbar
    accent: string;
}

const THEME_COLORS: Record<Theme, ThemeColors> = {
    dark: {
        background: '#1C1D21',
        foreground: '#EAEAEA',
        toolbarBg: 'rgba(30, 30, 35, 0.8)',
        accent: '#F59E0B', // Amber-400
    },
    light: {
        background: '#F9FAFB', // Gray-50
        foreground: '#1F2937', // Gray-800
        toolbarBg: 'rgba(255, 255, 255, 0.8)',
        accent: '#F59E0B',
    },
    warm: {
        background: '#FDF6E3', // Solarized Base3
        foreground: '#657B83', // Solarized Base00
        toolbarBg: 'rgba(253, 246, 227, 0.9)',
        accent: '#D33682', // Magenta
    },
};

export const ThemeProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    // Load saved theme or default to dark
    const [theme, setTheme] = useState<Theme>(() => {
        const saved = localStorage.getItem('floating-md-theme');
        return (saved as Theme) || 'dark';
    });

    useEffect(() => {
        localStorage.setItem('floating-md-theme', theme);
        applyTheme(theme);
    }, [theme]);

    const nextTheme = () => {
        const order: Theme[] = ['dark', 'light', 'warm'];
        const nextIndex = (order.indexOf(theme) + 1) % order.length;
        setTheme(order[nextIndex]);
    };

    const applyTheme = (t: Theme) => {
        const root = document.documentElement;
        const colors = THEME_COLORS[t];

        root.style.setProperty('--bg-base', colors.background);
        root.style.setProperty('--fg-base', colors.foreground);
        root.style.setProperty('--toolbar-bg', colors.toolbarBg);
        root.style.setProperty('--accent', colors.accent);

        // Update body background immediately to prevent flashes
        document.body.style.backgroundColor = colors.background;
        document.body.style.color = colors.foreground;
    };

    return (
        <ThemeContext.Provider value={{ theme, setTheme, nextTheme }}>
            {children}
        </ThemeContext.Provider>
    );
};

import React from 'react';
import { toolbarRegistry, ToolbarItem } from './ToolbarConfig';
import { useAutoHide } from '../../hooks/useAutoHide';
import { useTheme } from '../../contexts/ThemeContext';
import { Eye, EyeOff, Palette } from 'lucide-react';
import { ClipboardHistory } from '../Clipboard/ClipboardHistory';

interface ToolbarProps {
    orientation: 'horizontal' | 'vertical';
    className?: string;
    onToggleEditor?: () => void;
    mode?: 'toolbar' | 'editor';
}

export const Toolbar: React.FC<ToolbarProps> = ({
    orientation,
    className = '',
    onToggleEditor,
    mode = 'toolbar'
}) => {
    const [items, setItems] = React.useState<ToolbarItem[]>([]);
    const [pinned, setPinned] = React.useState(false);
    const [historyOpen, setHistoryOpen] = React.useState(false);
    const { isVisible, show, startHideTimer } = useAutoHide(!pinned && mode === 'toolbar', 3000); // Disable autohide in editor mode
    const { theme, nextTheme } = useTheme();

    React.useEffect(() => {
        setItems(toolbarRegistry.getAll());

        const handlePinToggle = () => {
            setPinned(p => !p);
            show();
        };

        const handleHistoryToggle = () => {
            setHistoryOpen(p => !p);
            show();
        };

        window.addEventListener('toggle-pin', handlePinToggle);
        window.addEventListener('toggle-history', handleHistoryToggle);
        return () => {
            window.removeEventListener('toggle-pin', handlePinToggle);
            window.removeEventListener('toggle-history', handleHistoryToggle);
        };
    }, [show]);

    const handleItemClick = async (item: ToolbarItem) => {
        if (item.type === 'separator') return;

        // Special handling for markdown button (doesn't need focus preservation)
        if (item.id === 'markdown' && onToggleEditor) {
            onToggleEditor();
            return;
        }

        try {
            // 🛡️ Focus Guardian: Save the current foreground window before action
            const { invoke } = await import('@tauri-apps/api/core');
            await invoke('save_foreground');

            // Execute the action
            await item.action();
        } catch (error) {
            console.error(`Error executing ${item.id}:`, error);
        }
    };

    const handleHistorySelect = async (_content: string) => {
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('send_paste');
    };

    // Determine styles based on mode
    const isEditor = mode === 'editor';

    const containerClasses = isEditor
        // Editor Mode: Transparent! Let the App's bg-base show through.
        ? "bg-transparent w-full rounded-none"
        // Toolbar Mode: Full pill shape with glass effect
        : `glass transition-all duration-500 ease-in-out ${orientation === 'horizontal' ? 'toolbar-horizontal' : 'toolbar-vertical'} rounded-full`;

    const visibilityClasses = isEditor
        ? "opacity-100 scale-100" // Always visible in editor
        : (isVisible ? 'opacity-100 scale-100' : 'opacity-20 scale-90 blur-[1px]');

    return (
        <div className={`relative ${isEditor ? 'w-full' : ''}`}>
            {/* Clipboard History Popup */}
            <ClipboardHistory
                isOpen={historyOpen}
                onClose={() => setHistoryOpen(false)}
                onSelect={handleHistorySelect}
            />

            {/* Main Toolbar */}
            <div
                onMouseEnter={show}
                onMouseLeave={startHideTimer}
                className={`
                    relative
                    flex items-center gap-3 ${isEditor ? 'p-2' : 'p-3'}
                    ${containerClasses}
                    ${visibilityClasses}
                    ${className}
                `}
            >
                {/* Drag Region - Behind buttons (z-index: -10) */}
                <div
                    data-tauri-drag-region
                    className="absolute inset-0 -z-10"
                />


                {/* Toolbar Items - Foreground layer for button clicks */}
                <div
                    className={`
                        relative z-10
                        flex items-center gap-2 w-full h-full
                        ${orientation === 'horizontal' ? 'flex-row' : 'flex-col'}
                    `}
                >
                    {items.map((item) => {
                        if (item.type === 'separator') {
                            return (
                                <div
                                    key={item.id}
                                    className={`
                      bg-white/10
                      ${orientation === 'horizontal' ? 'w-px h-6' : 'h-px w-6'}
                    `}
                                />
                            );
                        }

                        // Special rendering/logic for specific items
                        let icon = item.icon;
                        if (item.id === 'pin') {
                            icon = pinned ? <Eye size={20} className="text-cyan-400" /> : <EyeOff size={20} />;
                        }

                        // Quit button positioning
                        const isQuit = item.id === 'quit';
                        const positionClass = isQuit
                            ? (orientation === 'horizontal' ? 'ml-auto' : 'mt-auto')
                            : '';

                        // Inject Theme Toggle Button before Quit button
                        const themeButton = isQuit ? (
                            <button
                                key="theme-toggle"
                                onClick={nextTheme}
                                title={`Current Theme: ${theme}`}
                                className={`
                                    group w-10 h-10 flex items-center justify-center rounded-full text-text-secondary
                                    transition-all duration-300 hover:bg-white/10 active:scale-95 hover:text-cyan-400
                                `}
                            >
                                <span className="transition-transform duration-300 group-hover:rotate-180">
                                    <Palette size={20} />
                                </span>
                            </button>
                        ) : null;

                        return (
                            <React.Fragment key={item.id}>
                                {themeButton}
                                <button
                                    onClick={() => handleItemClick(item)}
                                    title={item.tooltip}
                                    className={`
                                        group
                                        w-10 h-10
                                        flex items-center justify-center
                                        rounded-full
                                        text-text-secondary
                                        transition-all duration-300
                                        hover:bg-white/10
                                        active:scale-95
                                        hover:text-cyan-400
                                        hover:shadow-[0_0_15px_rgba(34,211,238,0.4)]
                                        ${positionClass}
                                    `}
                                >
                                    <span className="transition-transform duration-300 group-hover:scale-110">
                                        {icon}
                                    </span>
                                </button>
                            </React.Fragment>
                        );
                    })}
                </div>
            </div>
        </div>
    );
};

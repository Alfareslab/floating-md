import React from 'react';
import { toolbarRegistry, ToolbarItem } from './ToolbarConfig';
import { useTheme } from '../../contexts/ThemeContext';
import { Palette } from 'lucide-react';
import { ClipboardHistory } from '../Clipboard/ClipboardHistory';

interface EditorHeaderProps {
    onToggleEditor: () => void;
}

export const EditorHeader: React.FC<EditorHeaderProps> = ({ onToggleEditor }) => {
    const [items, setItems] = React.useState<ToolbarItem[]>([]);
    const [historyOpen, setHistoryOpen] = React.useState(false);
    const { theme, nextTheme } = useTheme();

    React.useEffect(() => {
        const loadedItems = toolbarRegistry.getAll();
        console.log("EditorHeader loaded items:", loadedItems.length);
        setItems(loadedItems);

        const handleHistoryToggle = () => {
            setHistoryOpen(p => !p);
        };
        // ... rest of useEffect

        window.addEventListener('toggle-history', handleHistoryToggle);
        return () => {
            window.removeEventListener('toggle-history', handleHistoryToggle);
        };
    }, []);

    const handleItemClick = async (item: ToolbarItem) => {
        if (item.type === 'separator') return;

        // Toggle back to floating mode
        if (item.id === 'markdown') {
            onToggleEditor();
            return;
        }

        try {
            await item.action();
        } catch (error) {
            console.error(`Error executing ${item.id}:`, error);
        }
    };

    const handleHistorySelect = async (_content: string) => {
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('send_paste');
    };

    if (items.length === 0) {
        return (
            <div className="w-full p-4 flex items-center justify-center text-red-400 bg-white/5">
                <span>⚠️ Toolbar Items Not Loaded</span>
            </div>
        );
    }

    return (
        <div className="w-full relative shrink-0 z-50">
            {/* Clipboard History Popup */}
            <ClipboardHistory
                isOpen={historyOpen}
                onClose={() => setHistoryOpen(false)}
                onSelect={handleHistorySelect}
            />

            {/* Header Content */}
            <div
                data-tauri-drag-region
                className="flex items-center gap-3 p-3 w-full border-b border-white/5 bg-transparent"
            >
                <div className="flex items-center gap-2 w-full flex-row">
                    {items.map((item) => {
                        // Skip Pin button in Editor Mode - not needed
                        if (item.id === 'pin') return null;

                        if (item.type === 'separator') {
                            return (
                                <div key={item.id} className="bg-white/10 w-px h-6" />
                            );
                        }

                        // Quit button positioning - push to end
                        const isQuit = item.id === 'quit';
                        const positionClass = isQuit ? 'ml-auto' : '';

                        // Theme Toggle Button (before Quit)
                        const themeButton = isQuit ? (
                            <button
                                key="theme-toggle"
                                onClick={nextTheme}
                                title={`Current Theme: ${theme}`}
                                className="group w-10 h-10 flex items-center justify-center rounded-full text-text-secondary transition-all duration-300 hover:bg-white/10 active:scale-95 hover:text-cyan-400"
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
                                        group w-10 h-10 flex items-center justify-center rounded-full text-text-secondary
                                        transition-all duration-300 hover:bg-white/10 active:scale-95 hover:text-cyan-400
                                        hover:shadow-[0_0_15px_rgba(34,211,238,0.4)]
                                        ${positionClass}
                                    `}
                                >
                                    <span className="transition-transform duration-300 group-hover:scale-110">
                                        {item.icon}
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

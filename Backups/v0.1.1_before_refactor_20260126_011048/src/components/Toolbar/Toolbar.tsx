import React from 'react';
import { toolbarRegistry, ToolbarItem } from './ToolbarConfig';
import { useAutoHide } from '../../hooks/useAutoHide';
import { Eye, EyeOff, GripHorizontal, GripVertical } from 'lucide-react';
import { ClipboardHistory } from '../Clipboard/ClipboardHistory';

interface ToolbarProps {
    orientation: 'horizontal' | 'vertical';
    className?: string;
}

export const Toolbar: React.FC<ToolbarProps> = ({ orientation, className = '' }) => {
    const [items, setItems] = React.useState<ToolbarItem[]>([]);
    const [pinned, setPinned] = React.useState(false);
    const [historyOpen, setHistoryOpen] = React.useState(false);
    const { isVisible, show, startHideTimer } = useAutoHide(!pinned, 3000);

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

        try {
            await item.action();
        } catch (error) {
            console.error(`Error executing ${item.id}:`, error);
        }
    };

    const handleHistorySelect = async (_content: string) => {
        // After selecting from history, paste it
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('send_paste');
    };

    return (
        <div className="relative">
            {/* Clipboard History Popup */}
            <ClipboardHistory
                isOpen={historyOpen}
                onClose={() => setHistoryOpen(false)}
                onSelect={handleHistorySelect}
            />

            {/* Main Toolbar */}
            <div
                data-tauri-drag-region
                onMouseEnter={show}
                onMouseLeave={startHideTimer}
                className={`
            glass
            flex items-center gap-3 p-3
            transition-all duration-500 ease-in-out
            ${orientation === 'horizontal' ? 'toolbar-horizontal' : 'toolbar-vertical'}
            ${isVisible ? 'opacity-100 scale-100' : 'opacity-20 scale-90 blur-[1px]'} 
            ${className}
          `}
                style={{
                    borderRadius: '9999px',
                }}
            >
                {/* Toolbar Items */}
                <div
                    className={`
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

                        // Special rendering for Pin button
                        let icon = item.icon;
                        if (item.id === 'pin') {
                            icon = pinned ? <Eye size={20} className="text-cyan-400" /> : <EyeOff size={20} />;
                        }

                        // Special positioning for Quit button
                        const isQuit = item.id === 'quit';
                        const positionClass = isQuit
                            ? (orientation === 'horizontal' ? 'ml-auto' : 'mt-auto')
                            : '';

                        return (
                            <button
                                key={item.id}
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
                        );
                    })}
                </div>
            </div>
        </div>
    );
};

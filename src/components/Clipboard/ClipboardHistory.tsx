/**
 * Clipboard History Component
 * Displays recent clipboard entries in a dropdown menu
 */

import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Clock, Copy, X } from 'lucide-react';

interface ClipboardEntry {
    id: number;
    content: string;
    content_type: string;
    created_at: string;
    is_pinned: boolean;
}

interface ClipboardHistoryProps {
    isOpen: boolean;
    onClose: () => void;
    onSelect: (content: string) => void;
    dockSide: 'top' | 'bottom' | 'left' | 'right';
}

export const ClipboardHistory: React.FC<ClipboardHistoryProps> = ({
    isOpen,
    onClose,
    onSelect,
    dockSide
}) => {
    const [entries, setEntries] = useState<ClipboardEntry[]>([]);
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        if (isOpen) {
            loadHistory();
        }
    }, [isOpen]);

    const loadHistory = async () => {
        setLoading(true);
        try {
            const result = await invoke<ClipboardEntry[]>('get_recent_entries', { limit: 10 });
            setEntries(result);
        } catch (error) {
            console.error('Failed to load clipboard history:', error);
        } finally {
            setLoading(false);
        }
    };

    const handleSelect = async (entry: ClipboardEntry) => {
        try {
            await invoke('set_clipboard', { text: entry.content });
            onSelect(entry.content);
            onClose();
        } catch (error) {
            console.error('Failed to set clipboard:', error);
        }
    };

    const handleDelete = async (id: number, e: React.MouseEvent) => {
        e.stopPropagation();
        try {
            await invoke('delete_entry', { id });
            setEntries(prev => prev.filter(entry => entry.id !== id));
        } catch (error) {
            console.error('Failed to delete entry:', error);
        }
    };

    const truncateText = (text: string, maxLength: number = 50) => {
        if (text.length <= maxLength) return text;
        return text.substring(0, maxLength) + '...';
    };

    if (!isOpen) return null;

    // Calculate Position based on Dock Side
    let positionClasses = '';

    switch (dockSide) {
        case 'left':
            positionClasses = 'top-0 left-[90px] h-[600px] w-80'; // To the right of toolbar
            break;
        case 'right':
            positionClasses = 'top-0 right-[90px] h-[600px] w-80'; // To the left of toolbar
            break;
        case 'top':
            positionClasses = 'top-[80px] left-0 w-[600px] h-80'; // Below toolbar, reduced gap
            break;
        case 'bottom':
            positionClasses = 'bottom-[80px] left-0 w-[600px] h-80'; // Above toolbar
            break;
        default:
            positionClasses = 'top-[80px] left-0 w-[600px] h-80'; // Fallback
    }

    return (
        <div className={`absolute ${positionClasses}
                        bg-black/80 backdrop-blur-xl rounded-xl border border-white/10
                        shadow-2xl z-50 flex flex-col`}>
            {/* Header */}
            <div className="flex items-center justify-between p-3 border-b border-white/10">
                <div className="flex items-center gap-2 text-white/80">
                    <Clock size={16} />
                    <span className="text-sm font-medium">Clipboard History</span>
                </div>
                <button
                    onClick={onClose}
                    className="p-1 rounded-full hover:bg-white/10 transition-colors"
                >
                    <X size={16} className="text-white/60" />
                </button>
            </div>

            {/* Content */}
            <div className="p-2">
                {loading ? (
                    <div className="text-center py-4 text-white/50">Loading...</div>
                ) : entries.length === 0 ? (
                    <div className="text-center py-4 text-white/50">No history yet</div>
                ) : (
                    <ul className="space-y-1">
                        {entries.map((entry) => (
                            <li
                                key={entry.id}
                                onClick={() => handleSelect(entry)}
                                className="group flex items-center justify-between p-2 rounded-lg
                                           hover:bg-white/10 cursor-pointer transition-colors"
                            >
                                <div className="flex items-center gap-2 min-w-0">
                                    <Copy size={14} className="text-cyan-400 flex-shrink-0" />
                                    <span className="text-sm text-white/80 truncate">
                                        {truncateText(entry.content)}
                                    </span>
                                </div>
                                <button
                                    onClick={(e) => handleDelete(entry.id, e)}
                                    className="p-1 rounded-full opacity-0 group-hover:opacity-100
                                               hover:bg-red-500/20 transition-all"
                                >
                                    <X size={12} className="text-red-400" />
                                </button>
                            </li>
                        ))}
                    </ul>
                )}
            </div>
        </div>
    );
};

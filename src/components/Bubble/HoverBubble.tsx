import React from 'react';
import { ClipboardEntry } from '../../stores/clipboardStore';
import { detectDirection } from '../../utils/bidi';

interface HoverBubbleProps {
    entry: ClipboardEntry;
    position: { x: number; y: number };
    onCopy: () => void;
    onEdit: () => void;
    onPin: () => void;
    onDelete: () => void;
    onClose: () => void;
}

export const HoverBubble: React.FC<HoverBubbleProps> = ({
    entry,
    position,
    onCopy,
    onEdit,
    onPin,
    onDelete,
    onClose,
}) => {
    const direction = detectDirection(entry.content);
    const truncatedContent = entry.content.slice(0, 200) + (entry.content.length > 200 ? '...' : '');

    return (
        <div
            className="fixed glass rounded-xl p-4 shadow-2xl z-50 max-w-md"
            style={{
                left: `${position.x}px`,
                top: `${position.y}px`,
            }}
            onMouseLeave={onClose}
        >
            {/* Content Preview */}
            <div
                className="mb-3 text-sm text-text-secondary overflow-hidden"
                dir={direction}
                style={{
                    maxHeight: '120px',
                    overflowY: 'auto',
                }}
            >
                {entry.contentType === 'code' ? (
                    <pre className="code-block text-xs p-2">
                        <code>{truncatedContent}</code>
                    </pre>
                ) : (
                    <p className="whitespace-pre-wrap">{truncatedContent}</p>
                )}
            </div>

            {/* Type Badge */}
            <div className="mb-3">
                <span
                    className={`
            inline-block px-2 py-1 rounded text-xs font-medium
            ${entry.contentType === 'code' ? 'bg-accent/20 text-accent' : ''}
            ${entry.contentType === 'markdown' ? 'bg-purple-500/20 text-purple-400' : ''}
            ${entry.contentType === 'text' ? 'bg-text-muted/20 text-text-secondary' : ''}
          `}
                >
                    {entry.contentType === 'code' && '💻 Code'}
                    {entry.contentType === 'markdown' && '📝 Markdown'}
                    {entry.contentType === 'text' && '📄 Text'}
                </span>
            </div>

            {/* Action Buttons */}
            <div className="flex gap-2">
                <button
                    onClick={onCopy}
                    className="flex-1 px-3 py-2 bg-accent/10 hover:bg-accent/20 rounded-lg text-sm transition-colors"
                    title="Copy"
                >
                    📋 Copy
                </button>
                <button
                    onClick={onEdit}
                    className="flex-1 px-3 py-2 bg-surface hover:bg-hover rounded-lg text-sm transition-colors"
                    title="Edit"
                >
                    ✏️ Edit
                </button>
                <button
                    onClick={onPin}
                    className="px-3 py-2 bg-surface hover:bg-hover rounded-lg text-sm transition-colors"
                    title={entry.isPinned ? 'Unpin' : 'Pin'}
                >
                    {entry.isPinned ? '📌' : '📍'}
                </button>
                <button
                    onClick={onDelete}
                    className="px-3 py-2 bg-red-500/10 hover:bg-red-500/20 rounded-lg text-sm transition-colors"
                    title="Delete"
                >
                    🗑️
                </button>
            </div>
        </div>
    );
};

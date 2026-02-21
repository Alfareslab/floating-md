import React, { useState } from 'react';
import {
    Bold, Italic, Heading1, Heading2, List, Code,
    Wand2, ChevronDown
} from 'lucide-react';
import { AIAction } from '../../services/ai/aiService';

interface EditorToolbarProps {
    onInsert: (prefix: string, suffix: string) => void;
    onAIAction: (action: AIAction) => void;
    isProcessingAI: boolean;
}

export const EditorToolbar: React.FC<EditorToolbarProps> = ({
    onInsert,
    onAIAction,
    isProcessingAI
}) => {
    const [showAIMenu, setShowAIMenu] = useState(false);

    const handleAI = (action: AIAction) => {
        onAIAction(action);
        setShowAIMenu(false);
    };

    return (
        <div className="flex items-center gap-1 p-2 bg-surface border-b border-border overflow-x-auto">
            {/* Formatting Group */}
            <div className="flex items-center gap-0.5 border-r border-white/10 pr-2 mr-2">
                <ToolbarButton
                    icon={<Bold size={16} />}
                    tooltip="Bold (Ctrl+B)"
                    onClick={() => onInsert('**', '**')}
                />
                <ToolbarButton
                    icon={<Italic size={16} />}
                    tooltip="Italic (Ctrl+I)"
                    onClick={() => onInsert('*', '*')}
                />
            </div>

            {/* Headings Group */}
            <div className="flex items-center gap-0.5 border-r border-white/10 pr-2 mr-2">
                <ToolbarButton
                    icon={<Heading1 size={16} />}
                    tooltip="Heading 1"
                    onClick={() => onInsert('# ', '')}
                />
                <ToolbarButton
                    icon={<Heading2 size={16} />}
                    tooltip="Heading 2"
                    onClick={() => onInsert('## ', '')}
                />
            </div>

            {/* Lists & Code */}
            <div className="flex items-center gap-0.5 border-r border-white/10 pr-2 mr-2">
                <ToolbarButton
                    icon={<List size={16} />}
                    tooltip="Bullet List"
                    onClick={() => onInsert('- ', '')}
                />
                <ToolbarButton
                    icon={<Code size={16} />}
                    tooltip="Code Block"
                    onClick={() => onInsert('```\n', '\n```')}
                />
            </div>

            {/* AI Magic Button */}
            <div className="relative ml-auto">
                <button
                    onClick={() => setShowAIMenu(!showAIMenu)}
                    disabled={isProcessingAI}
                    className={`
                        flex items-center gap-2 px-3 py-1.5 rounded-lg
                        transition-all duration-300
                        ${isProcessingAI
                            ? 'bg-purple-500/20 text-purple-300 cursor-wait'
                            : 'bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 text-white shadow-lg shadow-purple-500/20'
                        }
                    `}
                >
                    <Wand2 size={14} className={isProcessingAI ? 'animate-pulse' : ''} />
                    <span className="text-sm font-medium">AI Magic</span>
                    <ChevronDown size={12} className={`transition-transform ${showAIMenu ? 'rotate-180' : ''}`} />
                </button>

                {/* AI Dropdown */}
                {showAIMenu && (
                    <>
                        <div
                            className="fixed inset-0 z-40"
                            onClick={() => setShowAIMenu(false)}
                        />
                        <div className="absolute right-0 top-full mt-2 w-48 bg-gray-900 border border-purple-500/30 rounded-xl shadow-2xl z-50 overflow-hidden animate-in fade-in slide-in-from-top-2">
                            <div className="p-1">
                                <AIMenuItem
                                    label="📝 Summarize"
                                    onClick={() => handleAI('summarize')}
                                />
                                <AIMenuItem
                                    label="✨ Improve Writing"
                                    onClick={() => handleAI('improve')}
                                />
                                <AIMenuItem
                                    label="🔧 Fix Grammar"
                                    onClick={() => handleAI('fix_grammar')}
                                />
                                <AIMenuItem
                                    label="🌐 Translate to Arabic"
                                    onClick={() => handleAI('translate_ar')}
                                />
                            </div>
                        </div>
                    </>
                )}
            </div>
        </div>
    );
};

interface ToolbarButtonProps {
    icon: React.ReactNode;
    tooltip: string;
    onClick: () => void;
}

const ToolbarButton: React.FC<ToolbarButtonProps> = ({ icon, tooltip, onClick }) => (
    <button
        onClick={onClick}
        title={tooltip}
        className="p-2 rounded-lg text-text-secondary hover:bg-white/10 hover:text-cyan-400 transition-colors"
    >
        {icon}
    </button>
);

interface AIMenuItemProps {
    label: string;
    onClick: () => void;
}

const AIMenuItem: React.FC<AIMenuItemProps> = ({ label, onClick }) => (
    <button
        onClick={onClick}
        className="w-full text-left px-3 py-2 rounded-lg text-sm text-gray-300 hover:bg-purple-500/20 hover:text-purple-300 transition-colors"
    >
        {label}
    </button>
);

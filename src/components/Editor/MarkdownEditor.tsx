import React from 'react';
import { detectDirection } from '../../utils/bidi';
import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkGfm from 'remark-gfm';
import remarkRehype from 'remark-rehype';
import rehypeStringify from 'rehype-stringify';
import { EditorToolbar } from './EditorToolbar';
import { aiService, AIAction } from '../../services/ai/aiService';

interface MarkdownEditorProps {
    initialContent?: string;
    onClose?: () => void;
}

export const MarkdownEditor: React.FC<MarkdownEditorProps> = ({
    initialContent = '',
    onClose,
}) => {
    const [content, setContent] = React.useState(initialContent);
    const [mode, setMode] = React.useState<'edit' | 'preview'>('edit');
    const [copyStatus, setCopyStatus] = React.useState<string | null>(null);
    const [isProcessingAI, setIsProcessingAI] = React.useState(false);
    const textareaRef = React.useRef<HTMLTextAreaElement>(null);
    const direction = detectDirection(content);

    // Convert markdown to HTML for preview
    const htmlContent = React.useMemo(() => {
        if (mode !== 'preview') return '';

        try {
            const result = unified()
                .use(remarkParse)
                .use(remarkGfm)
                .use(remarkRehype)
                .use(rehypeStringify)
                .processSync(content);

            return String(result);
        } catch (error) {
            console.error('Failed to render markdown:', error);
            return `<pre>Error rendering markdown: ${error}</pre>`;
        }
    }, [content, mode]);

    // Insert text at cursor position (for Toolbar)
    const handleInsert = (prefix: string, suffix: string) => {
        if (!textareaRef.current) return;

        const start = textareaRef.current.selectionStart;
        const end = textareaRef.current.selectionEnd;
        const text = textareaRef.current.value;

        const before = text.substring(0, start);
        const selection = text.substring(start, end);
        const after = text.substring(end);

        const newContent = before + prefix + selection + suffix + after;
        setContent(newContent);

        // Restore focus and cursor
        setTimeout(() => {
            if (textareaRef.current) {
                textareaRef.current.focus();
                textareaRef.current.setSelectionRange(
                    start + prefix.length,
                    end + prefix.length
                );
            }
        }, 0);
    };

    // Handle AI Actions
    const handleAIAction = async (action: AIAction) => {
        if (!textareaRef.current) return;

        const start = textareaRef.current.selectionStart;
        const end = textareaRef.current.selectionEnd;
        const selectedText = content.substring(start, end);

        // Use selected text if available, otherwise use all content
        const textToProcess = selectedText || content;

        if (!textToProcess.trim()) return;

        setIsProcessingAI(true);
        try {
            const response = await aiService.processText(textToProcess, action);

            if (selectedText) {
                // Replace selection
                const before = content.substring(0, start);
                const after = content.substring(end);
                setContent(before + response.result + after);
            } else {
                // Append result if processing full text (unless it's improve/fix)
                if (action === 'summarize' || action === 'translate_ar') {
                    setContent(content + '\n\n' + response.result);
                } else {
                    setContent(response.result);
                }
            }
        } catch (error) {
            console.error('AI Action failed:', error);
        } finally {
            setIsProcessingAI(false);
        }
    };

    const handleLoadFromClipboard = async () => {
        try {
            const text = await navigator.clipboard.readText();
            setContent(text);
        } catch (error) {
            console.error('Failed to read clipboard:', error);
        }
    };

    const handleCopyHTML = async () => {
        try {
            const result = await unified()
                .use(remarkParse)
                .use(remarkGfm)
                .use(remarkRehype)
                .use(rehypeStringify)
                .process(content);

            await navigator.clipboard.writeText(String(result));
            setCopyStatus('HTML Copied!');
            setTimeout(() => setCopyStatus(null), 2000);
        } catch (error) {
            console.error('Failed to copy HTML:', error);
            setCopyStatus('Failed to copy');
            setTimeout(() => setCopyStatus(null), 2000);
        }
    };

    return (

        <div className="w-full h-full flex flex-col bg-base text-text-primary">
            {/* Header */}
            <div className="flex items-center justify-between p-3 border-b border-border bg-surface/50 backdrop-blur-sm">
                <div className="flex items-center gap-2">
                    <h2 className="text-sm font-medium opacity-70">Markdown Editor</h2>
                </div>
                <div className="flex gap-2 items-center">
                    {copyStatus && (
                        <span className="text-xs text-green-400 bg-green-400/10 px-2 py-1 rounded animate-in fade-in slide-in-from-right-2">
                            {copyStatus}
                        </span>
                    )}
                    <button
                        onClick={handleLoadFromClipboard}
                        className="px-3 py-1.5 bg-accent/10 hover:bg-accent/20 rounded-md text-xs transition-colors"
                    >
                        📋 Load
                    </button>
                    <button
                        onClick={handleCopyHTML}
                        className="px-3 py-1.5 bg-surface hover:bg-hover rounded-md text-xs transition-colors"
                    >
                        📄 Copy HTML
                    </button>
                    <button
                        onClick={() => setMode(m => m === 'edit' ? 'preview' : 'edit')}
                        className={`px-3 py-1.5 rounded-md text-xs transition-colors ${mode === 'preview' ? 'bg-accent text-white' : 'bg-surface hover:bg-hover'}`}
                    >
                        {mode === 'edit' ? '👁️ Preview' : '✏️ Edit'}
                    </button>
                    {onClose && (
                        <button
                            onClick={onClose}
                            className="px-3 py-1.5 bg-red-500/10 hover:bg-red-500/20 text-red-400 rounded-md text-xs transition-colors"
                        >
                            ✕
                        </button>
                    )}
                </div>
            </div>

            {/* Toolbar (Only in Edit Mode) */}
            {mode === 'edit' && (
                <EditorToolbar
                    onInsert={handleInsert}
                    onAIAction={handleAIAction}
                    isProcessingAI={isProcessingAI}
                />
            )}

            {/* Content Area */}
            <div className="flex-1 flex overflow-hidden relative">
                {/* AI Loading Overlay */}
                {isProcessingAI && (
                    <div className="absolute inset-0 z-50 bg-black/40 backdrop-blur-[1px] flex items-center justify-center">
                        <div className="bg-surface border border-purple-500/30 p-4 rounded-xl shadow-2xl flex flex-col items-center gap-3 animate-in zoom-in-95">
                            <div className="w-6 h-6 border-2 border-purple-500 border-t-transparent rounded-full animate-spin"></div>
                            <span className="text-purple-300 text-sm font-medium">AI is thinking...</span>
                        </div>
                    </div>
                )}

                {/* Edit Panel */}
                {mode === 'edit' && (
                    <div className="flex-1 p-4 h-full">
                        <textarea
                            ref={textareaRef}
                            value={content}
                            onChange={(e) => setContent(e.target.value)}
                            dir={direction}
                            className="w-full h-full bg-surface/50 rounded-lg p-4 font-mono text-sm resize-none focus:outline-none focus:ring-1 focus:ring-accent/50 transition-all"
                            placeholder="Enter your markdown here..."
                            style={{
                                lineHeight: '1.6',
                            }}
                        />
                    </div>
                )}

                {/* Preview Panel */}
                {mode === 'preview' && (
                    <div className="flex-1 p-6 overflow-auto h-full bg-base/50">
                        <div
                            dir={direction}
                            className="prose prose-invert prose-sm max-w-none"
                            dangerouslySetInnerHTML={{ __html: htmlContent }}
                        />
                    </div>
                )}
            </div>

            {/* Status Bar */}
            <div className="px-4 py-1 bg-surface border-t border-border flex justify-between text-[10px] text-text-secondary">
                <span>{content.length} chars</span>
                <span>{direction === 'rtl' ? 'RTL Mode' : 'LTR Mode'}</span>
            </div>
        </div>
    );

};

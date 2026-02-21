import React from 'react';
import { detectDirection } from '../../utils/bidi';
import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkGfm from 'remark-gfm';
import remarkRehype from 'remark-rehype';
import rehypeStringify from 'rehype-stringify';

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
            <div className="flex items-center justify-between p-4 border-b border-border bg-surface/50 backdrop-blur-sm">
                <h2 className="text-lg font-medium">Markdown Editor</h2>
                <div className="flex gap-2">
                    <button
                        onClick={handleLoadFromClipboard}
                        className="px-4 py-2 bg-accent/10 hover:bg-accent/20 rounded-lg text-sm transition-colors"
                    >
                        📋 Load from Clipboard
                    </button>
                    <button
                        onClick={handleCopyHTML}
                        className="px-4 py-2 bg-surface hover:bg-hover rounded-lg text-sm transition-colors"
                    >
                        📄 Copy HTML
                    </button>
                    <button
                        onClick={() => setMode(m => m === 'edit' ? 'preview' : 'edit')}
                        className="px-4 py-2 bg-surface hover:bg-hover rounded-lg text-sm transition-colors"
                    >
                        {mode === 'edit' ? '👁️ Preview' : '✏️ Edit'}
                    </button>
                    {onClose && (
                        <button
                            onClick={onClose}
                            className="px-4 py-2 bg-red-500/10 hover:bg-red-500/20 rounded-lg text-sm transition-colors"
                        >
                            ✕ Close
                        </button>
                    )}

                    {/* Copy Status Toast */}
                    {copyStatus && (
                        <div className="px-4 py-2 bg-accent/20 rounded-lg text-sm">
                            {copyStatus}
                        </div>
                    )}
                </div>
            </div>

            {/* Content Area */}
            <div className="flex-1 flex overflow-hidden">
                {/* Edit Panel */}
                {mode === 'edit' && (
                    <div className="flex-1 p-4 h-full">
                        <textarea
                            value={content}
                            onChange={(e) => setContent(e.target.value)}
                            dir={direction}
                            className="w-full h-full bg-surface/50 rounded-lg p-4 font-mono text-sm resize-none focus:outline-none focus:ring-2 focus:ring-accent"
                            placeholder="Enter your markdown here..."
                            style={{
                                lineHeight: '1.6',
                            }}
                        />
                    </div>
                )}

                {/* Preview Panel */}
                {mode === 'preview' && (
                    <div className="flex-1 p-4 overflow-auto h-full">
                        <div
                            dir={direction}
                            className="prose prose-invert max-w-none"
                            dangerouslySetInnerHTML={{ __html: htmlContent }}
                        />
                    </div>
                )}
            </div>
        </div>
    );

};

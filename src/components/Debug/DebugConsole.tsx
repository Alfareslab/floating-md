import React, { useState, useRef, useEffect } from 'react';
import { Copy, Trash2, ChevronDown, ChevronUp } from 'lucide-react';

interface LogEntry {
    timestamp: string;
    type: 'click' | 'action' | 'focus' | 'error' | 'info';
    message: string;
}

// Global log array that can be accessed from anywhere
const globalLogs: LogEntry[] = [];
const logListeners: ((logs: LogEntry[]) => void)[] = [];

/**
 * Add a log entry from anywhere in the app
 */
export const debugLog = (type: LogEntry['type'], message: string) => {
    const entry: LogEntry = {
        timestamp: new Date().toLocaleTimeString('en-US', { hour12: false, hour: '2-digit', minute: '2-digit', second: '2-digit' }),
        type,
        message
    };
    globalLogs.push(entry);
    // Keep only last 100 entries
    if (globalLogs.length > 100) globalLogs.shift();
    // Notify listeners
    logListeners.forEach(listener => listener([...globalLogs]));
    // Also log to browser console
    console.log(`[${entry.type.toUpperCase()}] ${entry.message}`);
};

/**
 * Debug Console Component
 */
export const DebugConsole: React.FC = () => {
    const [logs, setLogs] = useState<LogEntry[]>([]);
    const [isCollapsed, setIsCollapsed] = useState(false);
    const logContainerRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        // Subscribe to log updates
        const listener = (newLogs: LogEntry[]) => setLogs(newLogs);
        logListeners.push(listener);

        // Initial log
        debugLog('info', 'Debug Console initialized');

        return () => {
            const index = logListeners.indexOf(listener);
            if (index > -1) logListeners.splice(index, 1);
        };
    }, []);

    useEffect(() => {
        // Auto-scroll to bottom
        if (logContainerRef.current) {
            logContainerRef.current.scrollTop = logContainerRef.current.scrollHeight;
        }
    }, [logs]);

    const handleCopy = async () => {
        const text = logs.map(l => `[${l.timestamp}] [${l.type.toUpperCase()}] ${l.message}`).join('\n');
        try {
            await navigator.clipboard.writeText(text);
            debugLog('info', 'Logs copied to clipboard');
        } catch (e) {
            debugLog('error', `Failed to copy: ${e}`);
        }
    };

    const handleClear = () => {
        globalLogs.length = 0;
        setLogs([]);
        debugLog('info', 'Logs cleared');
    };

    const getTypeColor = (type: LogEntry['type']) => {
        switch (type) {
            case 'click': return 'text-blue-400';
            case 'action': return 'text-green-400';
            case 'focus': return 'text-yellow-400';
            case 'error': return 'text-red-400';
            default: return 'text-gray-400';
        }
    };

    return (
        <div className="fixed bottom-0 left-0 right-0 bg-black/90 border-t border-white/20 font-mono text-xs z-50">
            {/* Header */}
            <div className="flex items-center justify-between px-3 py-1 bg-white/5 border-b border-white/10">
                <button
                    onClick={() => { debugLog('click', 'Toggle clicked'); setIsCollapsed(!isCollapsed); }}
                    className="flex items-center gap-2 text-white/70 hover:text-white"
                >
                    {isCollapsed ? <ChevronUp size={14} /> : <ChevronDown size={14} />}
                    <span>🔍 Debug Console ({logs.length})</span>
                </button>
                <div className="flex gap-2">
                    <button
                        onClick={handleCopy}
                        className="p-1 hover:bg-white/10 rounded text-white/70 hover:text-white"
                        title="Copy Logs"
                    >
                        <Copy size={14} />
                    </button>
                    <button
                        onClick={handleClear}
                        className="p-1 hover:bg-white/10 rounded text-white/70 hover:text-white"
                        title="Clear Logs"
                    >
                        <Trash2 size={14} />
                    </button>
                </div>
            </div>

            {/* Log Content */}
            {!isCollapsed && (
                <div
                    ref={logContainerRef}
                    className="h-32 overflow-y-auto p-2 space-y-0.5"
                >
                    {logs.length === 0 ? (
                        <div className="text-white/30 italic">No logs yet. Click buttons to see events...</div>
                    ) : (
                        logs.map((log, i) => (
                            <div key={i} className="flex gap-2">
                                <span className="text-white/40">{log.timestamp}</span>
                                <span className={`${getTypeColor(log.type)} w-12`}>[{log.type.toUpperCase()}]</span>
                                <span className="text-white/90">{log.message}</span>
                            </div>
                        ))
                    )}
                </div>
            )}
        </div>
    );
};

import { create } from 'zustand';

export interface ClipboardEntry {
    id: number;
    content: string;
    contentType: 'text' | 'code' | 'markdown';
    createdAt: number;
    isPinned: boolean;
    isScrubbed: boolean;
}

interface ClipboardStore {
    entries: ClipboardEntry[];
    selectedEntry: ClipboardEntry | null;

    // Actions
    addEntry: (entry: Omit<ClipboardEntry, 'id' | 'createdAt'>) => void;
    removeEntry: (id: number) => void;
    pinEntry: (id: number) => void;
    unpinEntry: (id: number) => void;
    selectEntry: (entry: ClipboardEntry | null) => void;
    clearAll: () => void;
}

export const useClipboardStore = create<ClipboardStore>((set) => ({
    entries: [],
    selectedEntry: null,

    addEntry: (entry) =>
        set((state) => ({
            entries: [
                {
                    ...entry,
                    id: Date.now(),
                    createdAt: Date.now(),
                },
                ...state.entries,
            ].slice(0, 100), // Keep only last 100 entries
        })),

    removeEntry: (id) =>
        set((state) => ({
            entries: state.entries.filter((e) => e.id !== id),
            selectedEntry: state.selectedEntry?.id === id ? null : state.selectedEntry,
        })),

    pinEntry: (id) =>
        set((state) => ({
            entries: state.entries.map((e) =>
                e.id === id ? { ...e, isPinned: true } : e
            ),
        })),

    unpinEntry: (id) =>
        set((state) => ({
            entries: state.entries.map((e) =>
                e.id === id ? { ...e, isPinned: false } : e
            ),
        })),

    selectEntry: (entry) =>
        set({ selectedEntry: entry }),

    clearAll: () =>
        set({ entries: [], selectedEntry: null }),
}));

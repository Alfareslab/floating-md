/**
 * Toolbar Configuration
 * Extensible registry for toolbar actions
 */

import { invoke } from '@tauri-apps/api/core';
import {
    Copy,
    ClipboardPaste,
    Scissors,
    CheckSquare,
    Sparkles,
    Trash2
} from 'lucide-react';
import React from 'react';

export type ToolbarItemType = 'action' | 'toggle' | 'separator';

export interface ToolbarItem {
    id: string;
    label: string;
    icon: React.ReactNode;
    action: () => void | Promise<void>;
    type: ToolbarItemType;
    order: number;
    tooltip?: string;
}

const MarkdownIcon = ({ size = 20, className = "" }: { size?: number, className?: string }) => (
    <svg 
        xmlns= "http://www.w3.org/2000/svg"
width = { size }
height = { size }
viewBox = "0 0 24 24"
fill = "none"
stroke = "currentColor"
strokeWidth = "2"
strokeLinecap = "round"
strokeLinejoin = "round"
className = { className }
    >
    <rect width="18" height = "18" x = "3" y = "3" rx = "2" ry = "2" />
        <path d="M7 8v8" />
            <path d="M7 8l3 5 3-5v8" />
                <path d="M16 8v8" />
                    <path d="M16 8c1.5 0 2.5.5 2.5 4s-1 4-2.5 4H16" />
                        </svg>
);

/**
 * Default toolbar actions
 * These are the core actions that ship with the app
 */
export const defaultToolbarItems: ToolbarItem[] = [
    {
        id: 'copy',
        label: 'Copy',
        icon: React.createElement(Copy, { size: 20 }),
        action: async () => {
            await invoke('send_copy');
        },
        type: 'action',
        order: 1,
        tooltip: 'Copy (Ctrl+C)',
    },
    {
        id: 'paste',
        label: 'Paste',
        icon: React.createElement(ClipboardPaste, { size: 20 }),
        action: async () => {
            await invoke('send_paste');
        },
        type: 'action',
        order: 2,
        tooltip: 'Paste (Ctrl+V)',
    },
    {
        id: 'cut',
        label: 'Cut',
        icon: React.createElement(Scissors, { size: 20 }),
        action: async () => {
            await invoke('send_cut');
        },
        type: 'action',
        order: 3,
        tooltip: 'Cut (Ctrl+X)',
    },
    {
        id: 'select-all',
        label: 'Select All',
        icon: React.createElement(CheckSquare, { size: 20 }),
        action: async () => {
            await invoke('send_select_all');
        },
        type: 'action',
        order: 4,
        tooltip: 'Select All (Ctrl+A)',
    },
    {
        id: 'delete',
        label: 'Delete',
        icon: React.createElement(Trash2, { size: 20 }),
        action: async () => {
            await invoke('send_delete');
        },
        type: 'action',
        order: 5,
        tooltip: 'Delete (Del)',
    },
    {
        id: 'separator-1',
        label: '',
        icon: null,
        action: () => { },
        type: 'separator',
        order: 6,
    },
    {
        id: 'markdown',
        label: 'Markdown',
        icon: React.createElement(MarkdownIcon, { size: 20 }),
        action: () => {
            console.log('Toggle Markdown Editor');
            const event = new CustomEvent('toggle-markdown-editor');
            window.dispatchEvent(event);
        },
        type: 'toggle',
        order: 7,
        tooltip: 'Toggle Markdown Editor',
    },
    {
        id: 'sparkle',
        label: 'Smart Scrub',
        icon: React.createElement(Sparkles, { size: 20, className: "text-amber-400" }),
        action: () => {
            console.log('Smart Scrub triggered');
        },
        type: 'action',
        order: 8,
        tooltip: 'Clean AI text',
    },
];

/**
 * Toolbar Registry
 * Allows adding custom toolbar items at runtime
 */
class ToolbarRegistry {
    private items: Map<string, ToolbarItem> = new Map();

    constructor() {
        // Register default items
        defaultToolbarItems.forEach((item) => {
            this.items.set(item.id, item);
        });
    }

    register(item: ToolbarItem): void {
        this.items.set(item.id, item);
    }

    unregister(id: string): void {
        this.items.delete(id);
    }

    getAll(): ToolbarItem[] {
        return Array.from(this.items.values()).sort((a, b) => a.order - b.order);
    }

    get(id: string): ToolbarItem | undefined {
        return this.items.get(id);
    }
}


export const toolbarRegistry = new ToolbarRegistry();

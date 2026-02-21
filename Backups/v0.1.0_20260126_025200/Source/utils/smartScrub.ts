/**
 * Smart Scrub Utility
 * Cleans AI-generated text by removing common prefixes and fluff
 */

/**
 * Common AI response patterns to remove
 */
const AI_PATTERNS = [
    // ChatGPT patterns
    /^(Sure|Certainly|Of course|Absolutely)[,!]?\s*/i,
    /^Here('s| is) (the|a|an)\s+/i,
    /^I('ll| will| can| would)\s+/i,
    /^Let me\s+/i,
    /^To (answer|help|assist)\s+/i,

    // Claude patterns
    /^I('d| would) be (happy|glad) to\s+/i,
    /^I understand you('re| are)\s+/i,

    // Gemini patterns
    /^Based on (your|the)\s+/i,
    /^According to\s+/i,

    // Generic patterns
    /^As (a|an) AI\s+/i,
    /^In (this|the) (case|context|example)\s+/i,
];

/**
 * Detects if text looks like AI-generated content
 */
export function isAIGenerated(text: string): boolean {
    const firstLine = text.split('\n')[0];
    return AI_PATTERNS.some((pattern) => pattern.test(firstLine));
}

/**
 * Removes AI prefixes and returns cleaned text
 */
export function scrubAIText(text: string): string {
    let cleaned = text;

    // Remove AI patterns from the beginning
    for (const pattern of AI_PATTERNS) {
        cleaned = cleaned.replace(pattern, '');
    }

    // Remove leading/trailing whitespace
    cleaned = cleaned.trim();

    // If text starts with a code block, extract it
    const codeBlockMatch = cleaned.match(/^```(\w+)?\n([\s\S]*?)\n```/);
    if (codeBlockMatch) {
        return codeBlockMatch[2].trim();
    }

    return cleaned;
}

/**
 * Extracts code blocks from text
 */
export function extractCodeBlocks(text: string): string[] {
    const codeBlockRegex = /```(\w+)?\n([\s\S]*?)\n```/g;
    const blocks: string[] = [];
    let match;

    while ((match = codeBlockRegex.exec(text)) !== null) {
        blocks.push(match[2].trim());
    }

    return blocks;
}

/**
 * Smart scrub with undo history
 */
export interface ScrubResult {
    original: string;
    cleaned: string;
    wasModified: boolean;
}

export function smartScrub(text: string): ScrubResult {
    const cleaned = scrubAIText(text);

    return {
        original: text,
        cleaned,
        wasModified: cleaned !== text,
    };
}

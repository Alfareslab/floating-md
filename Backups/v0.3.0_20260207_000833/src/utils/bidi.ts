/**
 * BiDi (Bidirectional Text) Utilities
 * Handles RTL/LTR detection and text direction
 */

/**
 * Detects if text contains RTL characters (Arabic, Hebrew, etc.)
 */
export function detectDirection(text: string): 'ltr' | 'rtl' {
    // RTL Unicode ranges:
    // Arabic: U+0600 to U+06FF
    // Hebrew: U+0590 to U+05FF
    // Arabic Supplement: U+0750 to U+077F
    // Arabic Extended-A: U+08A0 to U+08FF
    const rtlRegex = /[\u0591-\u07FF\uFB1D-\uFDFD\uFE70-\uFEFC]/;

    return rtlRegex.test(text) ? 'rtl' : 'ltr';
}

/**
 * Detects if text is primarily code (contains common programming patterns)
 */
export function isCode(text: string): boolean {
    // Check for common code patterns
    const codePatterns = [
        /^```/m, // Markdown code block
        /function\s+\w+\s*\(/,
        /const\s+\w+\s*=/,
        /let\s+\w+\s*=/,
        /var\s+\w+\s*=/,
        /import\s+.*from/,
        /export\s+(default\s+)?/,
        /#include\s*</,
        /public\s+class\s+/,
        /def\s+\w+\s*\(/,
        /<\?php/,
    ];

    return codePatterns.some((pattern) => pattern.test(text));
}

/**
 * Detects if text is Markdown
 */
export function isMarkdown(text: string): boolean {
    const markdownPatterns = [
        /^#{1,6}\s+/m, // Headers
        /\*\*.*\*\*/,  // Bold
        /\*.*\*/,      // Italic
        /\[.*\]\(.*\)/, // Links
        /^[-*+]\s+/m,  // Lists
        /^>\s+/m,      // Blockquotes
        /^```/m,       // Code blocks
    ];

    return markdownPatterns.some((pattern) => pattern.test(text));
}

/**
 * Determines content type based on text analysis
 */
export function detectContentType(text: string): 'text' | 'code' | 'markdown' {
    if (isCode(text)) return 'code';
    if (isMarkdown(text)) return 'markdown';
    return 'text';
}

/**
 * Wraps inline code snippets in RTL text with LTR isolation
 */
export function isolateCodeInRTL(text: string): string {
    // Find code patterns and wrap them with LTR marks
    const codePattern = /`([^`]+)`/g;
    return text.replace(codePattern, (match, code) => {
        return `\u202A${match}\u202C`; // LTR embedding
    });
}

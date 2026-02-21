/**
 * Mock AI Service
 * Simulates AI text processing capabilities.
 * In a real implementation, this would call an LLM API.
 */

export type AIAction = 'summarize' | 'improve' | 'fix_grammar' | 'translate_ar';

interface AIResponse {
    original: string;
    result: string;
    action: AIAction;
}

export const aiService = {
    /**
     * Process text with a specific AI action
     */
    processText: async (text: string, action: AIAction): Promise<AIResponse> => {
        // Simulate network delay
        await new Promise(resolve => setTimeout(resolve, 1500));

        let result = text;

        switch (action) {
            case 'summarize':
                result = `📝 **Summary:**\n\n${text.split(' ').slice(0, 20).join(' ')}...\n\n(Key points extracted by AI)`;
                break;
            case 'improve':
                result = text.replace(/\bvery\b\s/g, 'extremely ')
                    .replace(/\bgood\b/g, 'excellent')
                    .replace(/\bbad\b/g, 'suboptimal');
                break;
            case 'fix_grammar':
                // Mock grammar fix
                result = text.charAt(0).toUpperCase() + text.slice(1);
                if (!result.endsWith('.')) result += '.';
                break;
            case 'translate_ar':
                result = `[ترجمة AI]:\n${text}\n\n(نص مترجم للعربية)`;
                break;
        }

        return {
            original: text,
            result,
            action
        };
    }
};

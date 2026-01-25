use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PREAMBLE_PATTERNS: Vec<Regex> = vec![
        // 1. Simple affirmartions
        Regex::new(r"(?i)^(Sure|Certainly|Of course|Okay|Absolutely|Yes)(,|!)?\s*").unwrap(),
        
        // 2. I can help / happy to help (Match until punctuation or newline)
        Regex::new(r"(?i)^I(\s*('?d|'ll| am| would| will))? (be happy to|can|could) (help|provide|show|assist).*?(\.|:|\n)\s*").unwrap(),
        
        // 3. Here is the code (Match until punctuation or newline)
        Regex::new(r"(?i)^(Here is|Here's|Here are) (the|a|an|what|how).*?(\.|:|\n)\s*").unwrap(),
        
        // 4. "To do X..." or "The following code..."
        Regex::new(r"(?i)^(To|The) (following|code|solution|example).*?(\.|:|\n)\s*").unwrap(),
    ];
}

/// Clean AI conversational filler from text
pub fn scrub_text(text: &str) -> String {
    let mut result = text.trim().to_string();
    let mut changed = true;
    
    // Iteratively remove matching prefixes until no more match
    while changed {
        changed = false;
        for pattern in PREAMBLE_PATTERNS.iter() {
            if let Some(mat) = pattern.find(&result) {
                // Only remove if it's at the start
                if mat.start() == 0 {
                    result = result[mat.end()..].trim().to_string();
                    changed = true;
                    // Break inner loop to restart checking from the top with cleaned string
                    break; 
                }
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrub_preamble() {
        let input = "Sure! Here is the code you asked for:\n\nconst x = 1;";
        let expected = "const x = 1;";
        assert_eq!(scrub_text(input), expected);
    }

    #[test]
    fn test_scrub_complex() {
        let input = "Certainly, I can help with that. Here's the updated code:\n\nfunction test() {}";
        let expected = "function test() {}";
        assert_eq!(scrub_text(input), expected);
    }
}

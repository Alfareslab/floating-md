use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PREAMBLE_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)^(Here is|Here's|Here is the|Here's the) (code|solution|example|snippet|implementation|updated code) (you (asked|requested) for|for you|below|following)(:|.)?\s*").unwrap(),
        Regex::new(r"(?i)^(Certainly|Sure|Of course|Okay)(,|!) (I can|here is|here's|I'll|I will) (help|provide|show) (you|with) (that|the code)(:|.)?\s*").unwrap(),
        Regex::new(r"(?i)^I'd be happy to (help|provide) (you|with) (that|the code)(:|.)?\s*").unwrap(),
        Regex::new(r"(?i)^Here function that (implements|does|achieves)(.*?)(:|.)?\s*").unwrap(),
    ];
}

/// Clean AI conversational filler from text
pub fn scrub_text(text: &str) -> String {
    let mut result = text.trim().to_string();
    
    // 1. Remove standard AI preambles
    for pattern in PREAMBLE_PATTERNS.iter() {
        if let Some(mat) = pattern.find(&result) {
            // Only remove if it's at the start
            if mat.start() == 0 {
                result = result[mat.end()..].trim().to_string();
                // We don't break, in case there are multiple layers of fluff (rare but possible)
            }
        }
    }
    
    // 2. Extra feature: Extract code block if it's the *only* thing relevant
    // If the text contains ``` ... ```, and the text outside is small/fluff, extract just the code.
    // For now, let's keep it safe and only remove preambles.
    
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

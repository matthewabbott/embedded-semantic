use std::collections::HashSet;

pub fn clean_text(text: &str) -> String {
    // Remove extra whitespace and normalize
    text.split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn to_sentences(text: &str) -> Vec<String> {
    text.split(|c| matches!(c, '.' | '?' | '!'))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn get_stop_words() -> HashSet<&'static str> {
    // Common English stop words
    vec!["the", "be", "to", "of", "and", "a", "in", "that", "have", "i", 
         "it", "for", "not", "on", "with", "he", "as", "you", "do", "at"]
        .into_iter()
        .collect()
}

pub fn remove_stop_words(text: &str) -> String {
    let stop_words = get_stop_words();
    text.split_whitespace()
        .filter(|word| !stop_words.contains(&word.to_lowercase().as_str()))
        .collect::<Vec<&str>>()
        .join(" ")
}
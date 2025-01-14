// text_processing/chunking.rs
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use super::utils::{clean_text, to_sentences, remove_stop_words};

#[wasm_bindgen]
pub fn create_semantic_chunks(text: String, window_size: usize, threshold: f32) -> js_sys::Array {
    let chunks = calculate_chunks(&text, window_size, threshold);
    
    let js_array = js_sys::Array::new();
    for chunk in chunks {
        js_array.push(&chunk.into());
    }
    js_array
}

fn calculate_lexical_score(window1: &str, window2: &str) -> f32 {
    let mut word_counts1: HashMap<String, f32> = HashMap::new();
    let mut word_counts2: HashMap<String, f32> = HashMap::new();

    // Clean and remove stop words before calculating scores
    let clean_window1 = remove_stop_words(&clean_text(window1));
    let clean_window2 = remove_stop_words(&clean_text(window2));

    // Count words in each window
    for word in clean_window1.split_whitespace() {
        *word_counts1.entry(word.to_lowercase()).or_insert(0.0) += 1.0;
    }
    for word in clean_window2.split_whitespace() {
        *word_counts2.entry(word.to_lowercase()).or_insert(0.0) += 1.0;
    }

    // Calculate cosine similarity
    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;

    for (word, count1) in &word_counts1 {
        norm1 += count1 * count1;
        if let Some(count2) = word_counts2.get(word) {
            dot_product += count1 * count2;
        }
    }
    for (_, count2) in &word_counts2 {
        norm2 += count2 * count2;
    }

    // Return dissimilarity score
    if norm1 == 0.0 || norm2 == 0.0 {
        1.0 // Maximum dissimilarity if either window is empty after preprocessing
    } else {
        1.0 - (dot_product / (norm1.sqrt() * norm2.sqrt()))
    }
}

fn calculate_chunks(text: &str, window_size: usize, threshold: f32) -> Vec<String> {
    let mut chunks = Vec::new();
    let sentences = to_sentences(text);

    let mut current_chunk = String::new();
    let mut window_start = 0;

    while window_start + window_size * 2 <= sentences.len() {
        let window1 = &sentences[window_start..window_start + window_size].join(" ");
        let window2 = &sentences[window_start + window_size..window_start + window_size * 2].join(" ");
        
        let dissimilarity = calculate_lexical_score(window1, window2);
        
        if dissimilarity > threshold {
            // Found a topic boundary
            if !current_chunk.is_empty() {
                current_chunk.push_str(". ");
            }
            current_chunk.push_str(window1);
            chunks.push(current_chunk);
            current_chunk = String::new();
        } else {
            if !current_chunk.is_empty() {
                current_chunk.push_str(". ");
            }
            current_chunk.push_str(&sentences[window_start]);
        }
        
        window_start += 1;
    }

    // Add remaining text
    if window_start < sentences.len() {
        if !current_chunk.is_empty() {
            current_chunk.push_str(". ");
        }
        current_chunk.push_str(&sentences[window_start..].join(". "));
    }
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}
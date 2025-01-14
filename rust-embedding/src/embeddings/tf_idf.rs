use super::traits::{Document, SparseDocument, DocumentCollection};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct TfIdfDocument {
    text: String,
    terms: HashMap<String, f32>,
    magnitude: f32,
}

#[wasm_bindgen]
pub struct TfIdfCollection {
    documents: Vec<TfIdfDocument>,
}

impl Document for TfIdfDocument {
    fn new(text: String) -> Self {
        let terms = Self::compute_term_frequencies(&text);
        let magnitude = Self::calculate_magnitude(&terms);
        
        TfIdfDocument {
            text,
            terms,
            magnitude,
        }
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn get_magnitude(&self) -> f32 {
        self.magnitude
    }
}

impl SparseDocument for TfIdfDocument {
    fn get_sparse_vector(&self) -> &HashMap<String, f32> {
        &self.terms
    }
}

impl DocumentCollection for TfIdfCollection {
    type Doc = TfIdfDocument;

    fn new() -> Self {
        TfIdfCollection {
            documents: Vec::new()
        }
    }

    fn add_document(&mut self, doc: Self::Doc) {
        self.documents.push(doc);
    }

    fn search(&self, query: String) -> js_sys::Array {
        let query_doc = TfIdfDocument::new(query);
        let results = js_sys::Array::new();

        for doc in &self.documents {
            let similarity = self.calculate_similarity(&query_doc, doc);
            let result = js_sys::Object::new();
            js_sys::Reflect::set(&result, &"text".into(), &doc.text.clone().into()).unwrap();
            js_sys::Reflect::set(&result, &"score".into(), &similarity.into()).unwrap();
            results.push(&result);
        }

        results
    }
}

// WASM-exposed implementations
#[wasm_bindgen]
impl TfIdfDocument {
    #[wasm_bindgen(constructor)]
    pub fn create(text: String) -> TfIdfDocument {
        Self::new(text)
    }

    fn compute_term_frequencies(text: &str) -> HashMap<String, f32> {
        let mut frequencies: HashMap<String, f32> = HashMap::new();
        
        for term in text.to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()))
        {
            if !term.is_empty() {
                *frequencies.entry(term.to_string()).or_insert(0.0) += 1.0;
            }
        }

        frequencies
    }

    fn calculate_magnitude(terms: &HashMap<String, f32>) -> f32 {
        terms.values()
            .map(|freq| freq * freq)
            .sum::<f32>()
            .sqrt()
    }
}

#[wasm_bindgen]
impl TfIdfCollection {
    #[wasm_bindgen(constructor)]
    pub fn create() -> TfIdfCollection {
        Self::new()
    }

    fn calculate_similarity(&self, doc1: &TfIdfDocument, doc2: &TfIdfDocument) -> f32 {
        let vec1 = doc1.get_sparse_vector();
        let vec2 = doc2.get_sparse_vector();
        
        let mut dot_product = 0.0;
        for (term, freq1) in vec1 {
            if let Some(freq2) = vec2.get(term) {
                dot_product += freq1 * freq2;
            }
        }

        let magnitude1 = doc1.get_magnitude();
        let magnitude2 = doc2.get_magnitude();

        if magnitude1 > 0.0 && magnitude2 > 0.0 {
            dot_product / (magnitude1 * magnitude2)
        } else {
            0.0
        }
    }
}
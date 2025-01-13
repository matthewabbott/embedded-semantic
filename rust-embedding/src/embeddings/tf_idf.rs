use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use super::traits::{Embedding, DocumentCollection};

#[wasm_bindgen]
pub struct TfIdfDocument {
    text: String,
    terms: HashMap<String, f32>,
    magnitude: f32,
}

#[wasm_bindgen]
pub struct TfIdfCollection {
    documents: Vec<TfIdfDocument>,
    document_frequencies: HashMap<String, f32>,
    total_docs: f32,
}

// Regular trait implementations (not exposed to WASM)
impl Embedding for TfIdfDocument {
    fn new(text: String) -> Self {
        let terms = Self::compute_term_frequencies(&text);
        let magnitude = Self::calculate_magnitude(&terms);
        
        TfIdfDocument {
            text,
            terms,
            magnitude,
        }
    }
	
	// TODO
    fn get_text(&self) -> &str {
        &self.text
    }

	// TODO
    fn get_vector(&self) -> &HashMap<String, f32> {
        &self.terms
    }

	// TODO
    fn get_magnitude(&self) -> f32 {
        self.magnitude
    }
}

impl DocumentCollection for TfIdfCollection {
    type Doc = TfIdfDocument;

    fn new() -> Self {
        TfIdfCollection {
            documents: Vec::new(),
            document_frequencies: HashMap::new(),
            total_docs: 0.0,
        }
    }

    fn add_document(&mut self, doc: Self::Doc) {
        for term in doc.terms.keys() {
            *self.document_frequencies.entry(term.clone()).or_insert(0.0) += 1.0;
        }
        
        self.documents.push(doc);
        self.total_docs += 1.0;
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

    #[wasm_bindgen]
    pub fn add_document(&mut self, doc: TfIdfDocument) {
        DocumentCollection::add_document(self, doc)
    }

    #[wasm_bindgen]
    pub fn search(&self, query: String) -> js_sys::Array {
        DocumentCollection::search(self, query)
    }

    fn calculate_similarity(&self, doc1: &TfIdfDocument, doc2: &TfIdfDocument) -> f32 {
        let mut similarity = 0.0;

        for (term, freq1) in &doc1.terms {
            if let Some(freq2) = doc2.terms.get(term) {
                let idf = (self.total_docs / (self.document_frequencies.get(term).unwrap_or(&1.0) + 1.0)).ln();
                similarity += freq1 * freq2 * idf * idf;
            }
        }

        if doc1.magnitude > 0.0 && doc2.magnitude > 0.0 {
            similarity / (doc1.magnitude * doc2.magnitude)
        } else {
            0.0
        }
    }
}
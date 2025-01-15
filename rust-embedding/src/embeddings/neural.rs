use super::traits::{Document, DenseDocument, DocumentCollection};
use super::word_vectors::{get_test_vectors, EMBEDDING_SIZE};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct NeuralDocument {
    text: String,
    embedding: Vec<f32>,
}

impl NeuralDocument {
    fn compute_embedding(text: &str) -> Vec<f32> {
        let word_vectors = get_test_vectors();
        let mut embedding = vec![0.0; EMBEDDING_SIZE];
        let mut word_count = 0;

        // Simple averaging of word vectors
        for word in text.split_whitespace() {
            if let Some(vector) = word_vectors.get(&word.to_lowercase()) {
                for (i, &value) in vector.iter().enumerate() {
                    embedding[i] += value;
                }
                word_count += 1;
            }
        }

        // Normalize by word count
        if word_count > 0 {
            for value in embedding.iter_mut() {
                *value /= word_count as f32;
            }
        }

        embedding
    }
}

impl Document for NeuralDocument {
    fn new(text: String) -> Self {
        let embedding = Self::compute_embedding(&text);
        NeuralDocument {
            text,
            embedding,
        }
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn get_magnitude(&self) -> f32 {
        self.embedding.iter()
            .map(|x| x * x)
            .sum::<f32>()
            .sqrt()
    }
}

impl DenseDocument for NeuralDocument {
    fn get_dense_vector(&self) -> &[f32] {
        &self.embedding
    }
}

#[wasm_bindgen]
pub struct NeuralCollection {
    documents: Vec<NeuralDocument>,
}

impl DocumentCollection for NeuralCollection {
    type Doc = NeuralDocument;

    fn new() -> Self {
        NeuralCollection {
            documents: Vec::new(),
        }
    }

    fn add_document(&mut self, doc: Self::Doc) {
        self.documents.push(doc);
    }

    fn search(&self, query: String) -> js_sys::Array {
        let query_doc = NeuralDocument::new(query);
        let query_embedding = query_doc.get_dense_vector();
        
        let results = js_sys::Array::new();
        
        for doc in &self.documents {
            let similarity = calculate_cosine_similarity(query_embedding, doc.get_dense_vector());
            
            let result = js_sys::Object::new();
            js_sys::Reflect::set(&result, &"text".into(), &doc.text.clone().into()).unwrap();
            js_sys::Reflect::set(&result, &"score".into(), &similarity.into()).unwrap();
            results.push(&result);
        }
        
        results
    }
}

#[wasm_bindgen]
impl NeuralDocument {
    #[wasm_bindgen(constructor)]
    pub fn create(text: String) -> NeuralDocument {
        Self::new(text)
    }
}

#[wasm_bindgen]
impl NeuralCollection {
    #[wasm_bindgen(constructor)]
    pub fn create() -> NeuralCollection {
        Self::new()
    }

    #[wasm_bindgen]
    pub fn add_document(&mut self, doc: NeuralDocument) {
        DocumentCollection::add_document(self, doc)
    }

    #[wasm_bindgen]
    pub fn search(&self, query: String) -> js_sys::Array {
        DocumentCollection::search(self, query)
    }
}

// Helper function for cosine similarity
fn calculate_cosine_similarity(vec1: &[f32], vec2: &[f32]) -> f32 {
    let dot_product: f32 = vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| a * b)
        .sum();

    let norm1: f32 = vec1.iter()
        .map(|x| x * x)
        .sum::<f32>()
        .sqrt();

    let norm2: f32 = vec2.iter()
        .map(|x| x * x)
        .sum::<f32>()
        .sqrt();

    if norm1 > 0.0 && norm2 > 0.0 {
        dot_product / (norm1 * norm2)
    } else {
        0.0
    }
}
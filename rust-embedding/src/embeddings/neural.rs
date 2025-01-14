use super::traits::{Document, DenseDocument, DocumentCollection};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct NeuralDocument {
    text: String,
    embedding: Vec<f32>,
}

#[wasm_bindgen]
impl NeuralDocument {
    #[wasm_bindgen(constructor)]
    pub fn create(text: String) -> NeuralDocument {
        Self::new(text)
    }
}

impl Document for NeuralDocument {
    fn new(text: String) -> Self {
        NeuralDocument {
            text,
            embedding: vec![0.0; 384],
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
        // Create a dummy query embedding
        let query_embedding = vec![0.0; 384];
        
        // Calculate cosine similarities
        let results = js_sys::Array::new();
        
        for doc in &self.documents {
            // Calculate cosine similarity between embeddings
            let similarity = calculate_cosine_similarity(&query_embedding, &doc.embedding);
            
            let result = js_sys::Object::new();
            js_sys::Reflect::set(&result, &"text".into(), &doc.text.clone().into()).unwrap();
            js_sys::Reflect::set(&result, &"score".into(), &similarity.into()).unwrap();
            results.push(&result);
        }
        
        results
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
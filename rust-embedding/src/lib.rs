use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct Document {
    text: String,
    embedding: Vec<f32>
}

#[wasm_bindgen]
impl Document {
    #[wasm_bindgen(constructor)]
    pub fn new(text: String) -> Document {
        // Toy embedding: just count frequencies of each character
        let mut embedding = vec![0.0; 128]; // For ASCII characters
        for c in text.chars() {
            if (c as usize) < 128 {
                embedding[c as usize] += 1.0;
            }
        }
        
        // Normalize the vector
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for x in embedding.iter_mut() {
                *x /= magnitude;
            }
        }

        Document {
            text,
            embedding
        }
    }

    pub fn similarity(&self, other: &Document) -> f32 {
        // Compute cosine similarity
        let dot_product: f32 = self.embedding.iter()
            .zip(other.embedding.iter())
            .map(|(a, b)| a * b)
            .sum();
        dot_product
    }
}
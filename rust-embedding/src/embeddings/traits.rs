use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// Base trait for common functionality
pub trait Document {
    fn new(text: String) -> Self;
    fn get_text(&self) -> &str;
    fn get_magnitude(&self) -> f32;
}

// For TF-IDF sparse vectors
pub trait SparseDocument: Document {
    fn get_sparse_vector(&self) -> &HashMap<String, f32>;
}

// For neural net dense vectors
pub trait DenseDocument: Document {
    fn get_dense_vector(&self) -> &[f32];
}

// Collection traits
pub trait DocumentCollection {
    type Doc: Document;
    
    fn new() -> Self;
    fn add_document(&mut self, doc: Self::Doc);
    fn search(&self, query: String) -> js_sys::Array;
}
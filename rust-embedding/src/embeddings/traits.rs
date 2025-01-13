use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub trait Embedding {
    fn new(text: String) -> Self;
    fn get_text(&self) -> &str;
    fn get_vector(&self) -> &HashMap<String, f32>;
    fn get_magnitude(&self) -> f32;
}

pub trait DocumentCollection {
    type Doc: Embedding;
    
    fn new() -> Self;
    fn add_document(&mut self, doc: Self::Doc);
    fn search(&self, query: String) -> js_sys::Array;
}
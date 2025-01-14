use super::traits::{Embedding, DocumentCollection};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct NeuralDocument {
    text: String,
    embedding: Vec<f32>,
}

#[wasm_bindgen]
pub struct NeuralCollection {
    documents: Vec<NeuralDocument>,
}
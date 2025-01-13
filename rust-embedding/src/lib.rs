use wasm_bindgen::prelude::*;

mod embeddings;

// Re-export the types to expose to JavaScript
pub use embeddings::{TfIdfDocument, TfIdfCollection};

// TODO: add a version or init function
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
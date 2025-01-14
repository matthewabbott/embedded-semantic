use wasm_bindgen::prelude::*;

mod embeddings;
mod text_processing;

pub use embeddings::{TfIdfDocument, TfIdfCollection};
pub use text_processing::create_semantic_chunks;

// TODO: add a version or init function
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
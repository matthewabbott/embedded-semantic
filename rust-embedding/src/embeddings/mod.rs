pub mod traits;
pub mod tf_idf;
pub mod neural;

pub use tf_idf::{TfIdfDocument, TfIdfCollection};
pub use neural::{NeuralDocument, NeuralCollection};
pub use traits::{Document, SparseDocument, DenseDocument, DocumentCollection};
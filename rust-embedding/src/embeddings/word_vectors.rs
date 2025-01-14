use std::collections::HashMap;
use std::sync::OnceLock;

pub const EMBEDDING_SIZE: usize = 50;  // Small dimension for testing

// Use OnceLock for lazy initialization of the vectors
static WORD_VECTORS: OnceLock<HashMap<String, Vec<f32>>> = OnceLock::new();

pub fn get_test_vectors() -> &'static HashMap<String, Vec<f32>> {
    WORD_VECTORS.get_or_init(|| {
        let mut vectors = HashMap::new();
        
        // Common words with simple made up test vectors
        vectors.insert("the".to_string(), vec![0.1; EMBEDDING_SIZE]);
        vectors.insert("and".to_string(), vec![0.2; EMBEDDING_SIZE]);
        vectors.insert("is".to_string(), vec![0.3; EMBEDDING_SIZE]);
        vectors.insert("in".to_string(), vec![0.15; EMBEDDING_SIZE]);
        vectors.insert("of".to_string(), vec![0.25; EMBEDDING_SIZE]);
        vectors.insert("to".to_string(), vec![0.35; EMBEDDING_SIZE]);
        
        // Content words, slightly more varied vectors
        vectors.insert("cat".to_string(), 
            (0..EMBEDDING_SIZE).map(|i| (i as f32 * 0.1).sin()).collect());
        vectors.insert("dog".to_string(), 
            (0..EMBEDDING_SIZE).map(|i| (i as f32 * 0.2).sin()).collect());
        vectors.insert("house".to_string(), 
            (0..EMBEDDING_SIZE).map(|i| (i as f32 * 0.3).sin()).collect());
        
        vectors
    })
}
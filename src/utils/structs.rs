use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};


// Create a struct to hold the hash and error message
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Hash {
  hash: Option<String>,
  error: Option<String>,
}

// Implement the struct
#[wasm_bindgen]
impl Hash {
  // Create a new instance of the struct
  pub fn new(hash: Option<String>, error: Option<String>) -> Hash {
    Hash { hash, error }
  }

  // Get the hash
  pub fn hash(&self) -> Option<String> {
    self.hash.clone()
  }

  // Get the error message
  pub fn error(&self) -> Option<String> {
    self.error.clone()
  }
}
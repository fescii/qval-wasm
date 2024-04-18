// Path: src/hash/fns.rs

// Import the necessary modules
extern crate hex;


// Function to convert digest hash to a hex of different length
// and add a prefix to the hash
// and return the result as string
pub async fn convert_digest_to_hex(digested_hash: &[u8], section: &str) -> String {
  match section {
    "U" => format!("U0{}", hex::encode(&digested_hash)[..10].to_uppercase()),
    "P" => format!("P0{}", hex::encode(&digested_hash)[..14].to_uppercase()),
    "T" => format!("T0{}", hex::encode(&digested_hash)[..14].to_uppercase()),
    "N" => format!("N0{}", hex::encode(&digested_hash)[..14].to_uppercase()),
    _ => format!("R0{}", hex::encode(&digested_hash)[..14].to_uppercase()),
  }
}
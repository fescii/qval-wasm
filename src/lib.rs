use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::to_value;
use rand::Rng;
use sha2::Sha256;
use hmac::{Hmac, Mac};
extern crate hex;

//  Import local modules
mod utils;
use utils::{
	fns::convert_digest_to_hex,
	structs::Hash
};


#[wasm_bindgen]
pub async fn gen_hash(secrete: &str, section: &str, key: &str) -> JsValue {
	// Check if all parameters are provided
	if secrete.is_empty() || section.is_empty() || key.is_empty() {
		let error = Hash::new(None, Some("All parameters are required".to_string()));
		return to_value(&error).unwrap();
	}

	// Generate a random number
	let random: u64 = rand::thread_rng().gen_range(0..100);

	// Combine the secret and random number
	let secret_and_number = format!("{}-{}", secrete, random);

	// Match to create hmac instance
	let mut hmac = match Hmac::<Sha256>::new_from_slice(secret_and_number.as_bytes()) {
		Ok(hmac) => hmac,
		Err(_) => {
			let error = Hash::new(None, Some("Failed to create hmac instance".to_string()));
			return to_value(&error).unwrap();
		}
	};

	// Update the hmac instance with the key
	hmac.update(key.as_bytes());

	// Finalize the hmac instance
	let digested_hash = hmac.finalize().into_bytes();

	// Call the function to convert the digest hash to hex
	let hash = convert_digest_to_hex(&digested_hash, section).await;

	let hash_struct = Hash::new(Some(hash), None);

	return to_value(&hash_struct).unwrap();
}

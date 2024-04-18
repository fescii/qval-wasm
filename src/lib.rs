use wasm_bindgen::prelude::*;
use rand::Rng;
use sha2::Sha256;
use hmac::{Hmac, Mac};
// use wasm_bindgen_futures::JsFuture;
// use wasm_bindgen_futures::JsFuture;
extern crate hex;

//  Import local modules
mod utils;
use utils::{
	fns::convert_digest_to_hex,
	structs::Hash
};


#[wasm_bindgen]
pub async fn gen_hash(secrete: &str, section: &str, key: &str) -> Hash {
	// Check if all parameters are provided
	if secrete.is_empty() || section.is_empty() || key.is_empty() {
		return Hash::new(None, Some("All parameters are required".to_string()));
	}

	let random: u64 = rand::thread_rng().gen_range(0..100);
	let secret_and_number = format!("{}-{}", secrete, random);

	// Match to create hmac instance
	let mut hmac = match Hmac::<Sha256>::new_from_slice(secret_and_number.as_bytes()) {
		Ok(hmac) => hmac,
		Err(_) => return Hash::new(None, Some("Error creating hmac instance".to_string())),
	};

	hmac.update(key.as_bytes());

	let digested_hash = hmac.finalize().into_bytes();

	// Call the function to convert the digest hash to hex
	let hash = convert_digest_to_hex(&digested_hash, section).await;

	return Hash::new(Some(hash), None);
}

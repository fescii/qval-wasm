use wasm_bindgen::prelude::*;
use rand::Rng;
use sha2::Sha256;
use hmac::{Hmac, Mac};
// use wasm_bindgen_futures::JsFuture;
// use wasm_bindgen_futures::JsFuture;
extern crate hex;


#[wasm_bindgen]
pub async fn gen_hash(secrete: &str, section: &str, key: &str) -> String {
    let random: u64 = rand::thread_rng().gen_range(0..100);
    let secret_and_number = format!("{}-{}", secrete, random);

    let mut hmac = Hmac::<Sha256>::new_from_slice(secret_and_number.as_bytes()).expect("Failed to create HMAC");

    hmac.update(key.as_bytes());

    let digested_hash = hmac.finalize().into_bytes();

    match section {
        "U" => format!("U0{}", hex::encode(&digested_hash[..10]).to_uppercase()),
        "P" => format!("P0{}", hex::encode(&digested_hash[..14]).to_uppercase()),
        "T" => format!("T0{}", hex::encode(&digested_hash[..10]).to_uppercase()),
        "N" => format!("N0{}", hex::encode(&digested_hash[..14]).to_uppercase()),
        _ => format!("R0{}", hex::encode(&digested_hash[..14]).to_uppercase()),
    }
}

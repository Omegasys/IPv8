use base64::{encode, decode};
use sha2::{Sha256, Digest};

pub fn encrypt_data_a(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    encode(hasher.finalize())
}

pub fn encrypt_data_b(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes().iter().rev().collect::<Vec<_>>());
    encode(hasher.finalize())
}

pub fn decrypt_data_a(data: &str) -> String {
    let decoded = decode(data).unwrap_or_default();
    String::from_utf8(decoded).unwrap_or_else(|_| "Invalid Data".to_string())
}

pub fn decrypt_data_b(data: &str) -> String {
    let decoded = decode(data).unwrap_or_default();
    String::from_utf8(decoded.into_iter().rev().collect()).unwrap_or_else(|_| "Invalid Data".to_string())
}

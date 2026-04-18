use base64::{engine::general_purpose, Engine};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct IPv8Address {
    pub raw: Vec<u8>, // internal binary representation
}

impl IPv8Address {
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let bytes: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        Self { raw: bytes }
    }

    pub fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.raw)
    }

    pub fn from_base64(s: &str) -> Result<Self, base64::DecodeError> {
        let decoded = general_purpose::STANDARD.decode(s)?;
        Ok(Self { raw: decoded })
    }
}

use sha2::{Digest, Sha256};

pub fn sha256_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn sha256_verify(data: &[u8], expected: &[u8]) -> bool {
    sha256_hash(data) == expected
}

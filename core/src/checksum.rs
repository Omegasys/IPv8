use sha2::{Digest, Sha256};

pub fn compute_checksum(data: &[u8]) -> u32 {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    // Take first 4 bytes for a simple checksum
    u32::from_be_bytes([result[0], result[1], result[2], result[3]])
}

pub fn verify_checksum(data: &[u8], checksum: u32) -> bool {
    compute_checksum(data) == checksum
}

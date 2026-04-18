use crate::encryption::{encrypt, decrypt, EncryptionResult};
use crate::hashing::sha256_hash;

/// Simplified IPsec-like wrapper (NOT full IPsec implementation)
pub struct IPSecPacket {
    pub encrypted_payload: Vec<u8>,
    pub nonce: [u8; 12],
    pub integrity_hash: Vec<u8>,
}

pub fn protect_packet(
    key: &[u8; 32],
    payload: &[u8],
) -> IPSecPacket {
    let enc = encrypt(key, payload);

    let integrity_hash = sha256_hash(&enc.ciphertext);

    IPSecPacket {
        encrypted_payload: enc.ciphertext,
        nonce: enc.nonce,
        integrity_hash,
    }
}

pub fn verify_and_decrypt(
    key: &[u8; 32],
    packet: &IPSecPacket,
) -> Option<Vec<u8>> {
    // Verify integrity first
    let computed_hash = sha256_hash(&packet.encrypted_payload);
    if computed_hash != packet.integrity_hash {
        return None;
    }

    // Decrypt
    decrypt(key, &packet.nonce, &packet.encrypted_payload)
}

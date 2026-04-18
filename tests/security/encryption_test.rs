use ipv8_crypto::encryption::{encrypt, decrypt};

#[test]
fn test_encrypt_decrypt_cycle() {
    let key = [0u8; 32];
    let data = b"secure message";

    let enc = encrypt(&key, data);
    let dec = decrypt(&key, &enc.nonce, &enc.ciphertext).unwrap();

    assert_eq!(data.to_vec(), dec);
}

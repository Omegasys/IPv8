use x25519_dalek::{PublicKey, StaticSecret};
use rand_core::OsRng;

pub struct KeyPair {
    pub private: StaticSecret,
    pub public: PublicKey,
}

pub fn generate_keypair() -> KeyPair {
    let private = StaticSecret::new(OsRng);
    let public = PublicKey::from(&private);

    KeyPair { private, public }
}

pub fn compute_shared_secret(
    private: &StaticSecret,
    peer_public: &PublicKey,
) -> [u8; 32] {
    private.diffie_hellman(peer_public).to_bytes()
}

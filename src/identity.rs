use crate::address::IPv8Address;
use rand::Rng;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

const ROTATION_INTERVAL: u64 = 300; // 5 minutes

#[derive(Debug, Clone)]
pub struct IPv8Identity {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub address: IPv8Address,
    pub last_rotation: u64,
}

impl IPv8Identity {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let private_key: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

        let mut hasher = Sha256::new();
        hasher.update(&private_key);
        let public_key = hasher.finalize().to_vec();

        let address = IPv8Address::from_public_key(&public_key);

        IPv8Identity {
            private_key,
            public_key,
            address,
            last_rotation: current_time(),
        }
    }

    pub fn should_rotate(&self) -> bool {
        current_time() - self.last_rotation >= ROTATION_INTERVAL
    }

    pub fn rotate(&mut self) {
        let new_id = IPv8Identity::new();
        self.private_key = new_id.private_key;
        self.public_key = new_id.public_key;
        self.address = new_id.address;
        self.last_rotation = current_time();
        println!("🔁 Identity rotated → {}", self.address.address);
    }
}

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

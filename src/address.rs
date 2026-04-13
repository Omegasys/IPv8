use base64::{encode};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct IPv8Address {
    pub address: String,
}

impl IPv8Address {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut address = String::new();
        for _ in 0..16 {
            let group: String = (0..8)
                .map(|_| rng.gen_range(0..64) as u8)
                .map(|byte| base64::alphabet::BASE64_STANDARD[byte as usize] as char)
                .collect();
            address.push_str(&group);
            address.push(':');
        }
        address.pop();
        IPv8Address { address }
    }

    pub fn from_public_key(pubkey: &[u8]) -> Self {
        let mut hasher = sha2::Sha256::new();
        hasher.update(pubkey);
        let hash = hasher.finalize();
        let encoded = encode(hash);
        let chars: Vec<char> = encoded.chars().collect();

        let mut address = String::new();
        for i in 0..16 {
            let start = i * 8;
            let end = start + 8;
            let group: String = chars.get(start..end).unwrap_or(&['A'; 8]).iter().collect();
            address.push_str(&group);
            address.push(':');
        }
        address.pop();
        IPv8Address { address }
    }

    pub fn is_valid(&self) -> bool {
        self.address.split(':').count() == 16
    }
}

#[derive(Debug, Clone)]
pub enum IPVersion {
    V4,
    V6,
    V8,
}

#[derive(Debug, Clone)]
pub struct NetworkAddress {
    pub ip_version: IPVersion,
    pub raw: String,
}

impl NetworkAddress {
    pub fn spoof_as(&self, version: IPVersion) -> Self {
        let fake = match version {
            IPVersion::V4 => format!("192.168.{}.{}", rand::random::<u8>(), rand::random::<u8>()),
            IPVersion::V6 => format!("2001:db8::{:x}", rand::random::<u16>()),
            IPVersion::V8 => IPv8Address::new().address,
        };
        NetworkAddress { ip_version: version, raw: fake }
    }
}

use ipv8_core::address::IPv8Address;

/// Simulated IPv4 translator for IPv8 compatibility
pub struct IPv4Translator;

impl IPv4Translator {
    /// Convert IPv8 address into IPv4-compatible representation (simulated)
    pub fn to_ipv4(addr: &IPv8Address) -> String {
        let base64 = addr.to_base64();

        // Fake IPv4 mapping (for simulation only)
        let hash = &base64[..8.min(base64.len())];

        format!(
            "{}.{}.{}.{}",
            hash.bytes().nth(0).unwrap_or(0),
            hash.bytes().nth(1).unwrap_or(0),
            hash.bytes().nth(2).unwrap_or(0),
            hash.bytes().nth(3).unwrap_or(0),
        )
    }

    /// Convert IPv4 string back into IPv8 (lossy simulation)
    pub fn from_ipv4(ipv4: &str) -> IPv8Address {
        let bytes: Vec<u8> = ipv4
            .split('.')
            .filter_map(|s| s.parse::<u8>().ok())
            .collect();

        let mut padded = bytes;
        padded.resize(16, 0);

        IPv8Address { raw: padded }
    }
}

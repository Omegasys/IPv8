use ipv8_core::address::IPv8Address;

/// IPv6 compatibility translator
pub struct IPv6Translator;

impl IPv6Translator {
    /// Convert IPv8 address into IPv6-like compressed string
    pub fn to_ipv6(addr: &IPv8Address) -> String {
        let chunks: Vec<String> = addr
            .raw
            .chunks(2)
            .map(|pair| {
                let value = ((pair[0] as u16) << 8) | (pair[1] as u16);
                format!("{:x}", value)
            })
            .collect();

        chunks.join(":")
    }

    /// Convert IPv6-like string into IPv8 format
    pub fn from_ipv6(ipv6: &str) -> IPv8Address {
        let mut raw = Vec::new();

        for segment in ipv6.split(':') {
            let value = u16::from_str_radix(segment, 16).unwrap_or(0);
            raw.push((value >> 8) as u8);
            raw.push((value & 0xFF) as u8);
        }

        raw.resize(16, 0);

        IPv8Address { raw }
    }
}

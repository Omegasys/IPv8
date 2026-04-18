use ipv8_core::address::IPv8Address;
use crate::ipv4::translator::IPv4Translator;
use crate::ipv6::translator::IPv6Translator;

pub enum IPMode {
    IPv4,
    IPv6,
    IPv8,
}

/// Dual-stack compatibility layer
pub struct DualStack;

impl DualStack {
    pub fn detect(addr: &str) -> IPMode {
        if addr.contains('.') {
            IPMode::IPv4
        } else if addr.contains(':') {
            IPMode::IPv6
        } else {
            IPMode::IPv8
        }
    }

    pub fn normalize(addr: &str) -> IPv8Address {
        match Self::detect(addr) {
            IPMode::IPv4 => IPv4Translator::from_ipv4(addr),
            IPMode::IPv6 => IPv6Translator::from_ipv6(addr),
            IPMode::IPv8 => {
                // treat as base64-encoded IPv8
                IPv8Address::from_base64(addr).unwrap_or_else(|_| IPv8Address::new_random())
            }
        }
    }
}

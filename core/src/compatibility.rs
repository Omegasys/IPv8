#[derive(Debug)]
pub enum IPVersion {
    IPv4,
    IPv6,
    IPv8,
}

pub struct Compatibility;

impl Compatibility {
    pub fn detect_version(addr: &str) -> IPVersion {
        if addr.contains('.') {
            IPVersion::IPv4
        } else if addr.contains(':') {
            IPVersion::IPv6
        } else {
            IPVersion::IPv8
        }
    }

    pub fn is_compatible(_from: IPVersion, _to: IPVersion) -> bool {
        // Placeholder: allow all for now
        true
    }
}

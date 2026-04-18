use ipv8_core::packet::Packet;

#[derive(Debug)]
pub enum ProxyType {
    None,
    Http(String),
    Socks(String),
}

pub struct Proxy {
    pub proxy_type: ProxyType,
}

impl Proxy {
    pub fn new(proxy_type: ProxyType) -> Self {
        Self { proxy_type }
    }

    pub fn route(&self, packet: Packet) -> Packet {
        match &self.proxy_type {
            ProxyType::None => packet,
            ProxyType::Http(addr) => {
                println!("Routing via HTTP proxy: {}", addr);
                packet
            }
            ProxyType::Socks(addr) => {
                println!("Routing via SOCKS proxy: {}", addr);
                packet
            }
        }
    }
}

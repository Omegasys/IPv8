use ipv8_core::packet::Packet;

pub struct TlsAdapter {
    pub secure: bool,
}

impl TlsAdapter {
    pub fn new(secure: bool) -> Self {
        Self { secure }
    }

    pub fn encrypt_channel(&self, packet: Packet) -> Packet {
        if self.secure {
            println!("Encrypting packet via TLS layer (simulated)");
        }

        packet
    }

    pub fn decrypt_channel(&self, packet: Packet) -> Packet {
        if self.secure {
            println!("Decrypting packet via TLS layer (simulated)");
        }

        packet
    }
}

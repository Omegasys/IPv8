use ipv8_core::{packet::Packet, header::Header};
use ipv8_core::address::IPv8Address;

pub fn generate(count: usize) -> Vec<Packet> {
    let mut packets = Vec::new();

    for _ in 0..count {
        let src = IPv8Address::new_random();
        let dst = IPv8Address::new_random();

        let header = Header::new(src, dst);
        let payload = vec![0u8; 128];

        packets.push(Packet::new(header, payload));
    }

    packets
}

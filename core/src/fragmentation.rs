use crate::packet::Packet;

pub fn fragment(packet: &Packet, max_size: usize) -> Vec<Packet> {
    let mut fragments = Vec::new();
    let payload = &packet.payload;

    let mut start = 0;

    while start < payload.len() {
        let end = usize::min(start + max_size, payload.len());

        let fragment_payload = payload[start..end].to_vec();

        let mut new_packet = packet.clone();
        new_packet.payload = fragment_payload;
        new_packet.header.payload_length = new_packet.payload.len() as u32;

        fragments.push(new_packet);

        start = end;
    }

    fragments
}

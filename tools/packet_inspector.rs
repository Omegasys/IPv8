use ipv8_core::packet::Packet;

pub fn inspect(packet: &Packet) {
    println!("--- IPv8 Packet Inspector ---");
    println!("Header: {:?}", packet.header);
    println!("Payload size: {}", packet.payload.len());
    println!("Checksum: {:?}", packet.checksum);
}

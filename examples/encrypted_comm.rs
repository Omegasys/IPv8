use ipv8_crypto::ipsec_wrapper::protect_packet;
use ipv8_core::{packet::Packet, header::Header, address::IPv8Address};

fn main() {
    let key = [1u8; 32];

    let src = IPv8Address::new_random();
    let dst = IPv8Address::new_random();

    let header = Header::new(src, dst);
    let packet = Packet::new(header, b"secret data".to_vec());

    let secured = protect_packet(&key, &packet.payload);

    println!("Encrypted packet ready: {:?}", secured);
}

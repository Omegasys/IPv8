use ipv8_core::{packet::Packet, header::Header, address::IPv8Address};

#[test]
fn test_packet_creation_performance() {
    let src = IPv8Address::new_random();
    let dst = IPv8Address::new_random();

    for _ in 0..10_000 {
        let header = Header::new(src.clone(), dst.clone());
        let _packet = Packet::new(header, vec![0u8; 512]);
    }

    assert!(true);
}

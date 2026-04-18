use ipv8_core::address::IPv8Address;

#[test]
fn test_address_generation() {
    let addr = IPv8Address::new_random();
    assert_eq!(addr.raw.len(), 16);
}

#[test]
fn test_base64_roundtrip() {
    let addr = IPv8Address::new_random();
    let encoded = addr.to_base64();
    let decoded = IPv8Address::from_base64(&encoded).unwrap();

    assert_eq!(addr.raw, decoded.raw);
}

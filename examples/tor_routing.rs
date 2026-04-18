use ipv8_network::tor_adapter::TorAdapter;
use ipv8_core::{packet::Packet, header::Header, address::IPv8Address};

fn main() {
    let adapter = TorAdapter::new(true);

    let src = IPv8Address::new_random();
    let dst = IPv8Address::new_random();

    let packet = Packet::new(Header::new(src, dst), b"tor traffic".to_vec());

    let routed = adapter.route_through_tor(packet);

    println!("Packet routed via Tor: {:?}", routed);
}

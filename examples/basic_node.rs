use ipv8_network::node::Node;
use ipv8_core::address::IPv8Address;

fn main() {
    let node = Node::new(IPv8Address::new_random());
    let dst = IPv8Address::new_random();

    let packet = node.send_packet(&dst, b"hello ipv8".to_vec());

    println!("Packet created: {:?}", packet);
}

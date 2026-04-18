use ipv8_network::node::Node;
use ipv8_core::address::IPv8Address;

pub fn run_simulation(node_count: usize) {
    println!("Simulating {} IPv8 nodes...", node_count);

    let nodes: Vec<Node> = (0..node_count)
        .map(|_| Node::new(IPv8Address::new_random()))
        .collect();

    println!("Created {} nodes", nodes.len());
}

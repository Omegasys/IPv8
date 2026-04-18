use ipv8_core::{packet::Packet, address::IPv8Address};
use crate::router::Router;

#[derive(Debug)]
pub struct Node {
    pub address: IPv8Address,
}

impl Node {
    pub fn new(address: IPv8Address) -> Self {
        Self { address }
    }

    pub fn send_packet(&self, destination: &IPv8Address, payload: Vec<u8>) -> Packet {
        let header = ipv8_core::header::Header::new(self.address.clone(), destination.clone());
        let packet = Packet::new(header, payload);

        // Select route (simulation)
        let route = Router::select_route(&self.address, destination);

        println!("Sending packet through {} hops", route.hops.len());

        packet
    }
}

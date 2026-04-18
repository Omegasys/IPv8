use ipv8_core::address::IPv8Address;

#[derive(Debug)]
pub struct Route {
    pub hops: Vec<IPv8Address>,
}

pub struct Router;

impl Router {
    pub fn select_route(source: &IPv8Address, destination: &IPv8Address) -> Route {
        let mut hops = Vec::new();

        hops.push(source.clone());

        // Simulate at least 2 routers (your requirement)
        hops.push(IPv8Address::new_random());
        hops.push(IPv8Address::new_random());

        hops.push(destination.clone());

        Route { hops }
    }
}

use crate::address::IPv8Address;

#[derive(Debug)]
pub struct Route {
    pub hops: Vec<IPv8Address>,
}

pub struct Router;

impl Router {
    pub fn select_path(source: &IPv8Address, destination: &IPv8Address) -> Route {
        // Placeholder: simulate multi-hop route
        let mut hops = Vec::new();

        hops.push(source.clone());

        // Simulate 2 intermediate routers
        hops.push(IPv8Address::new_random());
        hops.push(IPv8Address::new_random());

        hops.push(destination.clone());

        Route { hops }
    }
}

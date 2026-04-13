use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Router {
    pub routes: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    pub fn add_route(&mut self, destination: &str, next_hop: &str) {
        self.routes.insert(destination.to_string(), next_hop.to_string());
    }

    pub fn resolve(&self, destination: &str) -> Option<&String> {
        self.routes.get(destination)
    }
}

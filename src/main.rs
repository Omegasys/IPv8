mod identity;
mod address;
mod router;
mod encryption;
mod transport;

use crate::router::Router;
use crate::transport::tcp::TcpServer;
use std::sync::Arc;
use crate::transport::tls::load_tls_config;

fn main() {
    // Create routers
    let router_count = 5;
    let mut routers = Vec::new();

    for _ in 0..router_count {
        let mut router = Router::new();
        for _ in 0..5 {
            let src = crate::address::IPv8Address::new().address;
            let dst = crate::address::IPv8Address::new().address;
            router.add_route(&src, &dst);
        }
        routers.push(router);
    }

    // TLS config
    let tls_config = Arc::new(load_tls_config());

    // Start TCP server
    let server = TcpServer::new("0.0.0.0:8080", routers, tls_config);
    server.run();
}

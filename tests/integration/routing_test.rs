use ipv8_network::router::Router;
use ipv8_core::address::IPv8Address;

#[test]
fn test_route_generation() {
    let src = IPv8Address::new_random();
    let dst = IPv8Address::new_random();

    let route = Router::select_route(&src, &dst);

    assert!(route.hops.len() >= 3);
}

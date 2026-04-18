use ipv8_compat::dual_stack::DualStack;

fn main() {
    let ipv4 = "192.168.1.10";
    let ipv6 = "2001:db8::1";

    let ipv8_from_v4 = DualStack::normalize(ipv4);
    let ipv8_from_v6 = DualStack::normalize(ipv6);

    println!("IPv4 → IPv8: {:?}", ipv8_from_v4);
    println!("IPv6 → IPv8: {:?}", ipv8_from_v6);
}

use base64::{encode, decode};
use sha2::{Sha256, Digest};
use rand::Rng;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::thread;

#[derive(Debug, Clone)]
struct IPv8Address {
    address: String,
}

impl IPv8Address {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut address = String::new();
        for _ in 0..16 {
            let group: String = (0..8)
                .map(|_| rng.gen_range(0..64) as u8)
                .map(|byte| base64::alphabet::BASE64_STANDARD[byte as usize] as char)
                .collect();
            address.push_str(&group);
            address.push(':');
        }
        address.pop(); // Remove trailing colon
        IPv8Address { address }
    }
    fn is_valid(&self) -> bool {
        self.address.split(':').count() == 16
    }
}

#[derive(Debug)]
struct Router {
    routes: HashMap<String, String>,
}

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, destination: &str, next_hop: &str) {
        self.routes.insert(destination.to_string(), next_hop.to_string());
    }

    fn resolve(&self, destination: &str) -> Option<&String> {
        self.routes.get(destination)
    }
}

fn encrypt_data_a(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    encode(hasher.finalize())
}

fn encrypt_data_b(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes().iter().rev().collect::<Vec<_>>());
    encode(hasher.finalize())
}

fn decrypt_data_a(data: &str) -> String {
    let decoded = decode(data).expect("Failed to decode data");
    String::from_utf8(decoded).unwrap_or_else(|_| "Invalid Data".to_string())
}

fn decrypt_data_b(data: &str) -> String {
    let decoded = decode(data).expect("Failed to decode data");
    String::from_utf8(decoded.into_iter().rev().collect()).unwrap_or_else(|_| "Invalid Data".to_string())
}

fn nat_translate(source_ip: &str, destination_ip: &str) -> (String, String) {
    let translated_source = format!("{}:translated", source_ip);
    let translated_destination = format!("{}:translated", destination_ip);
    (translated_source, translated_destination)
}

fn send_data(mut stream: TcpStream, data: &str) {
    stream.write_all(data.as_bytes()).unwrap();
}

fn receive_data(mut stream: TcpStream) -> String {
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer[..size]).to_string()
}

fn route_data(source: &IPv8Address, destination: &IPv8Address, routers: &Vec<Router>) {
    let mut rng = rand::thread_rng();

    println!("Routing data from {} to {}", source.address, destination.address);

    // Encrypt using method A
    let encrypted_data = encrypt_data_a("Hello, world!");

    // Multi-hop forwarding (forward direction)
    let mut current = source.address.clone();
    for _ in 0..rng.gen_range(2..5) {
        if let Some(router) = routers.get(rng.gen_range(0..routers.len())) {
            if let Some(next_hop) = router.resolve(&current) {
                println!("Forward hop through: {}", next_hop);
                current = next_hop.clone();
            }
        }
    }

    // Final delivery at destination
    println!("Data delivered to {}: {}", destination.address, encrypted_data);

    // Encrypt using method B for reverse direction
    let encrypted_response = encrypt_data_b("ACK");

    // Multi-hop forwarding (return direction)
    let mut current = destination.address.clone();
    for _ in 0..rng.gen_range(2..5) {
        if let Some(router) = routers.get(rng.gen_range(0..routers.len())) {
            if let Some(next_hop) = router.resolve(&current) {
                println!("Return hop through: {}", next_hop);
                current = next_hop.clone();
            }
        }
    }

    // Decrypt using method A at source
    let decrypted_response = decrypt_data_a(&encrypted_response);
    println!("Decrypted response at source: {}", decrypted_response);
}

fn handle_client(mut stream: TcpStream, routers: Vec<Router>) {
    let source = IPv8Address::new();
    let destination = IPv8Address::new();

    println!("New connection from: {}", source.address);

    route_data(&source, &destination, &routers);
    let data = receive_data(stream.try_clone().unwrap());
    println!("Received: {}", data);

    // Encrypt with method B at endpoint
    let encrypted_data = encrypt_data_b(&data);
    send_data(stream, &encrypted_data);
}

fn main() {
    let router_count = 5;
    let mut routers = Vec::new();

    // Create routers with random routing tables
    for _ in 0..router_count {
        let mut router = Router::new();
        for _ in 0..5 {
            let src = IPv8Address::new().address;
            let dst = IPv8Address::new().address;
            router.add_route(&src, &dst);
        }
        routers.push(router);
    }

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("IPv8 Server running on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let routers_clone = routers.clone();
                thread::spawn(move || {
                    handle_client(stream, routers_clone);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
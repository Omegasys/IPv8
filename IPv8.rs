use base64::{encode, decode};
use sha2::{Sha256, Digest};
use rand::Rng;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

const ROTATION_INTERVAL: u64 = 300; // 5 minutes

// =======================
// IPv8 Identity (NEW)
// =======================
#[derive(Debug, Clone)]
struct IPv8Identity {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    address: IPv8Address,
    last_rotation: u64,
}

impl IPv8Identity {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        let private_key: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

        let mut hasher = Sha256::new();
        hasher.update(&private_key);
        let public_key = hasher.finalize().to_vec();

        let address = IPv8Address::from_public_key(&public_key);

        IPv8Identity {
            private_key,
            public_key,
            address,
            last_rotation: current_time(),
        }
    }

    fn should_rotate(&self) -> bool {
        current_time() - self.last_rotation >= ROTATION_INTERVAL
    }

    fn rotate(&mut self) {
        let new_id = IPv8Identity::new();
        self.private_key = new_id.private_key;
        self.public_key = new_id.public_key;
        self.address = new_id.address;
        self.last_rotation = current_time();

        println!("🔁 Identity rotated → {}", self.address.address);
    }
}

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// =======================
// IPv8 Address
// =======================
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

        address.pop();
        IPv8Address { address }
    }

    // NEW: deterministic from key
    fn from_public_key(pubkey: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(pubkey);
        let hash = hasher.finalize();

        let encoded = encode(hash);

        let mut address = String::new();
        let chars: Vec<char> = encoded.chars().collect();

        for i in 0..16 {
            let start = i * 8;
            let end = start + 8;

            let group: String = chars
                .get(start..end)
                .unwrap_or(&['A'; 8])
                .iter()
                .collect();

            address.push_str(&group);
            address.push(':');
        }

        address.pop();
        IPv8Address { address }
    }

    fn is_valid(&self) -> bool {
        self.address.split(':').count() == 16
    }
}

// =======================
// Router
// =======================
#[derive(Debug, Clone)]
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

// =======================
// Encryption
// =======================
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
    String::from_utf8(decoded.into_iter().rev().collect())
        .unwrap_or_else(|_| "Invalid Data".to_string())
}

// =======================
// Networking
// =======================
fn send_data(mut stream: TcpStream, data: &str) {
    stream.write_all(data.as_bytes()).unwrap();
}

fn receive_data(mut stream: TcpStream) -> String {
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer[..size]).to_string()
}

// =======================
// Routing
// =======================
fn route_data(identity: &IPv8Identity, destination: &IPv8Address, routers: &Vec<Router>) {
    let mut rng = rand::thread_rng();

    println!(
        "Routing from {} → {}",
        identity.address.address, destination.address
    );

    let encrypted_data = encrypt_data_a("Hello, world!");

    let mut current = identity.address.address.clone();

    for _ in 0..rng.gen_range(2..5) {
        if let Some(router) = routers.get(rng.gen_range(0..routers.len())) {
            if let Some(next_hop) = router.resolve(&current) {
                println!("Forward hop → {}", next_hop);
                current = next_hop.clone();
            }
        }
    }

    println!("Delivered to {}: {}", destination.address, encrypted_data);
}

// =======================
// Client Handler
// =======================
fn handle_client(mut stream: TcpStream, routers: Vec<Router>) {
    let mut identity = IPv8Identity::new();

    // 🔁 Rotate on every new connection
    identity.rotate();

    let destination = IPv8Address::new();

    println!("New connection → {}", identity.address.address);

    route_data(&identity, &destination, &routers);

    let data = receive_data(stream.try_clone().unwrap());
    println!("Received: {}", data);

    let encrypted_data = encrypt_data_b(&data);
    send_data(stream, &encrypted_data);
}

// =======================
// Main
// =======================
fn main() {
    let router_count = 5;
    let mut routers = Vec::new();

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
}        }
    }
}
use std::net::{TcpListener, TcpStream};
use std::thread;
use crate::router::Router;
use std::sync::Arc;
use crate::transport::tls::handle_tls_client;
use crate::transport::websocket::handle_ws_client;

pub struct TcpServer {
    pub addr: String,
    pub routers: Vec<Router>,
    pub tls_config: Arc<rustls::ServerConfig>,
}

impl TcpServer {
    pub fn new(addr: &str, routers: Vec<Router>, tls_config: Arc<rustls::ServerConfig>) -> Self {
        TcpServer {
            addr: addr.to_string(),
            routers,
            tls_config,
        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Server running on {}", self.addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let routers_clone = self.routers.clone();
                    let tls_config_clone = self.tls_config.clone();

                    thread::spawn(move || {
                        // You can switch between TLS or WebSocket:
                        handle_tls_client(stream.try_clone().unwrap(), routers_clone, tls_config_clone.clone());
                        // OR for websocket:
                        // handle_ws_client(stream.try_clone().unwrap(), routers_clone);
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }
}

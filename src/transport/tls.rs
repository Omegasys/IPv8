use rustls::{ServerConfig, ServerConnection, StreamOwned};
use std::sync::Arc;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use crate::router::Router;
use crate::identity::IPv8Identity;
use crate::address::IPv8Address;
use crate::encryption::encrypt_data_b;

pub fn load_tls_config() -> ServerConfig {
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    let cert_chain = rustls_pemfile::certs(cert_file)
        .unwrap()
        .into_iter()
        .map(rustls::Certificate)
        .collect();

    let mut keys = rustls_pemfile::pkcs8_private_keys(key_file).unwrap();
    let private_key = rustls::PrivateKey(keys.remove(0));

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private

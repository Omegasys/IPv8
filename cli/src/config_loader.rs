use std::fs;

#[derive(Debug)]
pub struct Config {
    pub raw: String,
}

pub fn load_config(path: &str) -> Config {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|_| "default config".to_string());

    Config { raw: content }
}
